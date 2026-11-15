//! Interactive steps: picking rooms, the measures dialog, the measures-book
//! dialog and the guide. Interactive callbacks get no host access, so each
//! step ends by handing the host a one-word apply command that comes back
//! to [`crate::commands`] with full `HostApi` access.
//!
//! Every dialog carries its own short help, and a fold-out "How to measure"
//! section, so the user never has to leave it to know what to type.

use std::collections::HashMap;
use std::sync::Mutex;

use ocs_plugin_api::dialog::{DialogControl, DialogDef, DialogResponse, DialogValue};
use ocs_plugin_api::host::codec::Handle;
use ocs_plugin_api::host::{CommandStep, InteractiveCommand, PreviewWire};

use crate::commands::{self, Room};
use crate::solve::{self, P2};
use crate::survey::{self, Saved, Survey};

const GREEN: [f32; 4] = [0.45, 0.85, 0.45, 1.0];

/// Arguments of a command handed to the host as a bare verb (see
/// [`deferred`]).
static DEFERRED: Mutex<Option<(String, String)>> = Mutex::new(None);

/// Hand `verb args` to the host as the bare `verb`, keeping `args` here.
///
/// The host runs a multi-word line by dispatching it and then feeding the
/// words after the verb — and a final Enter — to whatever command is active,
/// which would be the dialog the verb opens; that Enter would close it at
/// once. A single word is dispatched with nothing fed after it.
pub fn deferred(verb: &str, args: String) -> CommandStep {
    *DEFERRED.lock().unwrap_or_else(|e| e.into_inner()) = Some((verb.to_string(), args));
    CommandStep::Dispatch(verb.to_string())
}

/// The arguments [`deferred`] kept for `verb`, once.
pub fn take_deferred(verb: &str) -> Option<String> {
    let mut slot = DEFERRED.lock().unwrap_or_else(|e| e.into_inner());
    if slot.as_ref().is_some_and(|(v, _)| v.eq_ignore_ascii_case(verb)) {
        return slot.take().map(|(_, a)| a);
    }
    None
}

fn with_msg(msg: &Option<String>, prompt: String) -> String {
    match msg {
        Some(m) => format!("{m}  |  {prompt}"),
        None => prompt,
    }
}

fn ring3(pts: &[P2], z: f64) -> Vec<[f64; 3]> {
    let mut v: Vec<[f64; 3]> = pts.iter().map(|p| [p[0], p[1], z]).collect();
    if let Some(&f) = v.first() {
        v.push(f);
    }
    v
}

// ── Pick rooms (closed polylines) ───────────────────────────────────────────

/// Choose the rooms with the CAD's own selection: single picks, or a window
/// / crossing box dragged over many rooms at once. The CAD highlights what
/// is selected, and the chosen rooms are also outlined in green, so they
/// stand out from the ones not chosen yet. Enter confirms.
pub struct PickRoomTool {
    rooms: Vec<Room>,
    /// Chosen rooms, in the order that gives their numbers.
    chosen: Vec<usize>,
    /// Command dispatched (deferred) with the rooms' handles.
    verb: &'static str,
    title: &'static str,
    msg: Option<String>,
}

impl PickRoomTool {
    pub fn new(rooms: Vec<Room>, verb: &'static str, title: &'static str) -> Self {
        PickRoomTool { rooms, chosen: Vec::new(), verb, title, msg: None }
    }

    /// The smallest room containing a point.
    fn room_at(&self, pt: [f64; 3]) -> Option<usize> {
        self.rooms
            .iter()
            .enumerate()
            .filter(|(_, r)| commands::point_in_polygon(&r.pts, [pt[0], pt[1]]))
            .min_by(|a, b| solve::signed_area(&a.1.pts).abs().total_cmp(&solve::signed_area(&b.1.pts).abs()))
            .map(|(i, _)| i)
    }

    fn toggle(&mut self, idx: Option<usize>) -> CommandStep {
        match idx {
            Some(i) => {
                if let Some(k) = self.chosen.iter().position(|&c| c == i) {
                    self.chosen.remove(k);
                } else {
                    self.chosen.push(i);
                }
                self.msg = None;
            }
            None => self.msg = Some(tr!("lì non c'è una polilinea chiusa", "no closed polyline there", "ahí no hay ninguna polilínea cerrada", "pas de polyligne fermée à cet endroit", "orada kapalı çoklu çizgi yok", "não há nenhuma polilinha fechada aí", "nie ma tu zamkniętej polilinii").into()),
        }
        CommandStep::NeedPoint
    }

    /// Centre of a room's bounding box.
    fn centre(&self, i: usize) -> P2 {
        let p = &self.rooms[i].pts;
        let (x0, x1) = p.iter().fold((f64::MAX, f64::MIN), |a, q| (a.0.min(q[0]), a.1.max(q[0])));
        let (y0, y1) = p.iter().fold((f64::MAX, f64::MIN), |a, q| (a.0.min(q[1]), a.1.max(q[1])));
        [(x0 + x1) / 2.0, (y0 + y1) / 2.0]
    }

    /// Bring the choice in step with the CAD's selection: rooms no longer
    /// selected are dropped, the others keep their place, and the new ones
    /// follow — those taken together by a box from the top down, left to
    /// right, like reading a page.
    fn sync(&mut self, handles: &[Handle]) {
        let selected: Vec<usize> = (0..self.rooms.len()).filter(|&i| handles.contains(&self.rooms[i].handle)).collect();
        self.chosen.retain(|i| selected.contains(i));
        let mut new: Vec<usize> = selected.into_iter().filter(|i| !self.chosen.contains(i)).collect();
        let band = new
            .iter()
            .map(|&i| {
                let p = &self.rooms[i].pts;
                let (y0, y1) = p.iter().fold((f64::MAX, f64::MIN), |a, q| (a.0.min(q[1]), a.1.max(q[1])));
                y1 - y0
            })
            .fold(f64::MAX, f64::min)
            * 0.5;
        new.sort_by(|&a, &b| {
            let (ca, cb) = (self.centre(a), self.centre(b));
            if (ca[1] - cb[1]).abs() > band {
                cb[1].total_cmp(&ca[1])
            } else {
                ca[0].total_cmp(&cb[0])
            }
        });
        self.chosen.extend(new);
        let others = handles.len().saturating_sub(self.chosen.len());
        self.msg = (others > 0 && self.chosen.is_empty())
            .then(|| tr!("nessuna polilinea chiusa tra gli oggetti selezionati", "no closed polyline among the selected objects", "ninguna polilínea cerrada entre los objetos seleccionados", "aucune polyligne fermée parmi les objets sélectionnés", "seçilen nesneler arasında kapalı çoklu çizgi yok", "nenhuma polilinha fechada entre os objetos selecionados", "brak zamkniętej polilinii wśród zaznaczonych obiektów").into());
    }
}

impl InteractiveCommand for PickRoomTool {
    fn prompt(&self) -> String {
        let p = if self.chosen.is_empty() {
            tr!(
                format!(
                    "{}: seleziona i vani (polilinee chiuse): un clic sulla linea, oppure una finestra trascinando il mouse \
                     per prenderne tanti insieme. Invio per continuare, Esc per annullare",
                    self.title
                ),
                format!(
                    "{}: select the rooms (closed polylines): click a line, or drag a window to take many at once. \
                     Enter to go on, Esc to cancel",
                    self.title
                ),
                format!(
                    "{}: seleccione las estancias (polilíneas cerradas): un clic en una línea, o arrastre una ventana para tomar muchas a la vez. Intro para continuar, Esc para cancelar",
                    self.title
                ),
                format!(
                    "{} : sélectionnez les pièces (polylignes fermées) : un clic sur une ligne, ou tracez une fenêtre pour en prendre plusieurs d'un coup. Entrée pour continuer, Échap pour annuler",
                    self.title
                ),
                format!(
                    "{}: odaları (kapalı çoklu çizgiler) seçin: bir çizgiye tıklayın veya birçoğunu birden almak için pencere çekin. Devam için Enter, iptal için Esc",
                    self.title
                ),
                format!(
                    "{}: selecione os compartimentos (polilinhas fechadas): um clique numa linha, ou arraste uma janela para apanhar muitos de uma vez. Enter para continuar, Esc para cancelar",
                    self.title
                ),
                format!(
                    "{}: zaznacz pomieszczenia (zamknięte polilinie): kliknij linię albo przeciągnij okno, aby wziąć wiele naraz. Enter, aby kontynuować, Esc, aby anulować",
                    self.title
                ),
            )
        } else {
            tr!(
                format!(
                    "{}: {} vani selezionati (evidenziati, bordo verde). Aggiungine altri con clic o finestra, Maiusc+clic \
                     per toglierne uno, Invio per continuare",
                    self.title,
                    self.chosen.len()
                ),
                format!(
                    "{}: {} rooms selected (highlighted, green outline). Add more by click or window, Shift+click to drop \
                     one, Enter to go on",
                    self.title,
                    self.chosen.len()
                ),
                format!(
                    "{}: {} estancias seleccionadas (resaltadas, contorno verde). Añada más con clic o ventana, Mayús+clic para quitar una, Intro para continuar",
                    self.title,
                    self.chosen.len()
                ),
                format!(
                    "{} : {} pièces sélectionnées (en surbrillance, contour vert). Ajoutez-en d'autres par clic ou fenêtre, Maj+clic pour en retirer une, Entrée pour continuer",
                    self.title,
                    self.chosen.len()
                ),
                format!(
                    "{}: {} oda seçildi (vurgulu, yeşil kontur). Tıklama veya pencereyle ekleyin, birini çıkarmak için Shift+tık, devam için Enter",
                    self.title,
                    self.chosen.len()
                ),
                format!(
                    "{}: {} compartimentos selecionados (realçados, contorno verde). Acrescente mais com clique ou janela, Shift+clique para retirar um, Enter para continuar",
                    self.title,
                    self.chosen.len()
                ),
                format!(
                    "{}: zaznaczono {} pomieszczeń (podświetlone, zielony obrys). Dodaj kolejne kliknięciem lub oknem, Shift+klik, aby usunąć jedno, Enter, aby kontynuować",
                    self.title,
                    self.chosen.len()
                ),
            )
        };
        with_msg(&self.msg, p)
    }
    fn is_selection_gathering(&self) -> bool {
        true
    }
    fn accepts_drag_selection(&self) -> bool {
        true
    }
    fn on_selection_complete(&mut self, handles: Vec<Handle>) -> CommandStep {
        self.sync(&handles);
        CommandStep::NeedPoint
    }
    // Hosts that send plain clicks instead of selections: a click inside a
    // room, or on its polyline, toggles it.
    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        let i = self.room_at(pt);
        self.toggle(i)
    }
    fn on_object_pick(&mut self, handle: Handle, pt: [f64; 3]) -> CommandStep {
        let i = self.rooms.iter().position(|r| r.handle == handle).or_else(|| self.room_at(pt));
        self.toggle(i)
    }
    fn on_cursor_move(&mut self, _pt: [f64; 3]) -> Vec<PreviewWire> {
        self.chosen.iter().map(|&i| PreviewWire::with_color(ring3(&self.rooms[i].pts, self.rooms[i].z), GREEN)).collect()
    }
    fn on_enter(&mut self) -> CommandStep {
        if self.chosen.is_empty() {
            return CommandStep::Cancel;
        }
        let hs: Vec<String> = self.chosen.iter().map(|&i| format!("{:X}", self.rooms[i].handle.value())).collect();
        deferred(self.verb, hs.join(","))
    }
}

// ── Dialog helpers ──────────────────────────────────────────────────────────

fn info(text: String) -> DialogControl {
    DialogControl::InfoText { text, align_h: None }
}

fn number(id: &str, label: &str, default: f64) -> DialogControl {
    DialogControl::Number { id: id.into(), label: label.into(), default, min: Some(0.0), max: None, step: None }
}

fn checkbox(id: &str, label: &str, default: bool) -> DialogControl {
    DialogControl::Checkbox { id: id.into(), label: label.into(), default }
}

/// A fold-out section of help texts (closed at first).
fn help_section(id: &str, title: &str, paragraphs: Vec<String>) -> DialogControl {
    DialogControl::Fields { id: id.into(), title: Some(title.into()), controls: paragraphs.into_iter().map(info).collect(), collapsed: true }
}

fn get_num(m: &HashMap<String, DialogValue>, id: &str) -> Option<f64> {
    match m.get(id)? {
        DialogValue::Number(v) => Some(*v),
        DialogValue::Text(s) => commands::parse_num(s),
        _ => None,
    }
}

fn get_bool(m: &HashMap<String, DialogValue>, id: &str, default: bool) -> bool {
    match m.get(id) {
        Some(DialogValue::Bool(b)) => *b,
        _ => default,
    }
}

fn get_path(m: &HashMap<String, DialogValue>, id: &str) -> Option<String> {
    match m.get(id)? {
        DialogValue::FilePath(s) | DialogValue::Text(s) => {
            let s = s.trim().trim_matches('"').to_string();
            (!s.is_empty()).then_some(s)
        }
        _ => None,
    }
}

fn csv_picker(default: &str) -> DialogControl {
    DialogControl::FilePicker {
        id: "csv_path".into(),
        label: tr!("File del libretto", "Book file", "Archivo del libro", "Fichier du carnet", "Defter dosyası", "Ficheiro do caderno", "Plik dziennika").into(),
        default_path: default.into(),
        filter_name: tr!("Libretto misure (CSV)", "Measures book (CSV)", "Libro de medidas (CSV)", "Carnet de mesures (CSV)", "Ölçü defteri (CSV)", "Caderno de medidas (CSV)", "Dziennik pomiarów (CSV)").into(),
        extensions: vec!["csv".into()],
        save: true,
    }
}

/// The paragraphs of the fold-out help in the measures dialog.
fn how_to_measure(multi: bool) -> Vec<String> {
    let mut v = vec![
        tr!(
            "COME SI USA — scegli una voce dall'elenco e premi Continua. Apri un vano alla volta, scrivi le sue \
             misure e premi OK: torni qui e l'elenco passa da solo al vano successivo. Quando tutto è a posto \
             l'elenco propone Calcola. Quello che scrivi resta memorizzato anche se chiudi la finestra.",
            "HOW IT WORKS — pick an entry from the list and press Continue. Open one room at a time, type its \
             measures and press OK: you come back here and the list moves on to the next room. When all is set the \
             list proposes Compute. What you type is kept even if you close the window.",
            "CÓMO FUNCIONA — elija una opción de la lista y pulse Continuar. Abra una estancia cada vez, escriba sus medidas y pulse OK: vuelve aquí y la lista pasa sola a la estancia siguiente. Cuando todo está listo la lista propone Calcular. Lo que escribe se conserva aunque cierre la ventana.",
            "MODE D'EMPLOI — choisissez une entrée dans la liste et appuyez sur Continuer. Ouvrez une pièce à la fois, saisissez ses mesures et appuyez sur OK : vous revenez ici et la liste passe d'elle-même à la pièce suivante. Quand tout est prêt, la liste propose Calculer. Ce que vous saisissez est conservé même si vous fermez la fenêtre.",
            "NASIL ÇALIŞIR — listeden bir seçenek seçip Devam'a basın. Her seferinde bir oda açın, ölçülerini girin ve Tamam'a basın: buraya dönersiniz ve liste kendiliğinden sonraki odaya geçer. Her şey hazır olduğunda liste Hesapla'yı önerir. Girdikleriniz pencereyi kapatsanız da saklanır.",
            "COMO FUNCIONA — escolha uma opção da lista e prima Continuar. Abra um compartimento de cada vez, escreva as suas medidas e prima OK: volta aqui e a lista passa sozinha ao compartimento seguinte. Quando tudo está pronto a lista propõe Calcular. O que escreve fica guardado mesmo que feche a janela.",
            "JAK TO DZIAŁA — wybierz pozycję z listy i naciśnij Dalej. Otwieraj po jednym pomieszczeniu, wpisz jego pomiary i naciśnij OK: wracasz tutaj, a lista sama przechodzi do następnego pomieszczenia. Gdy wszystko jest gotowe, lista proponuje Oblicz. To, co wpiszesz, zostaje zachowane, nawet jeśli zamkniesz okno.",
        )
        .to_string(),
        tr!(
            "LATI E DIAGONALI — misura ogni parete da spigolo a spigolo, sempre alla stessa altezza. Un vano di n \
             spigoli vuole tutti i lati e almeno n−3 diagonali (1 per un rettangolo, 3 per una L): quelle proposte \
             passano dentro il vano. Una diagonale in più fa da controllo: è l'unico modo per accorgersi di una \
             cifra letta male.",
            "SIDES AND DIAGONALS — measure each wall from corner to corner, always at the same height. A room of n \
             corners needs all sides and at least n−3 diagonals (1 for a rectangle, 3 for an L): the proposed ones run \
             inside the room. One more diagonal is the check: the only way to notice a misread digit.",
            "LADOS Y DIAGONALES — mida cada pared de esquina a esquina, siempre a la misma altura. Una estancia de n esquinas necesita todos los lados y al menos n−3 diagonales (1 para un rectángulo, 3 para una L): las propuestas pasan por dentro de la estancia. Una diagonal más sirve de control: es la única forma de notar una cifra mal leída.",
            "CÔTÉS ET DIAGONALES — mesurez chaque mur d'angle à angle, toujours à la même hauteur. Une pièce de n angles demande tous les côtés et au moins n−3 diagonales (1 pour un rectangle, 3 pour un L) : celles proposées passent à l'intérieur de la pièce. Une diagonale de plus sert de contrôle : c'est le seul moyen de repérer un chiffre mal lu.",
            "KENARLAR VE KÖŞEGENLER — her duvarı köşeden köşeye, hep aynı yükseklikte ölçün. n köşeli bir oda tüm kenarları ve en az n−3 köşegeni gerektirir (dikdörtgen için 1, L için 3): önerilenler odanın içinden geçer. Bir köşegen fazlası kontrol işi görür: yanlış okunmuş bir rakamı fark etmenin tek yolu budur.",
            "LADOS E DIAGONAIS — meça cada parede de canto a canto, sempre à mesma altura. Um compartimento de n cantos precisa de todos os lados e de pelo menos n−3 diagonais (1 para um retângulo, 3 para um L): as propostas passam por dentro do compartimento. Uma diagonal a mais serve de controlo: é a única forma de dar por um algarismo mal lido.",
            "BOKI I PRZEKĄTNE — zmierz każdą ścianę od narożnika do narożnika, zawsze na tej samej wysokości. Pomieszczenie o n narożnikach wymaga wszystkich boków i co najmniej n−3 przekątnych (1 dla prostokąta, 3 dla kształtu L): proponowane przechodzą wewnątrz pomieszczenia. Jedna przekątna więcej służy jako kontrola: to jedyny sposób, by zauważyć źle odczytaną cyfrę.",
        )
        .to_string(),
        tr!(
            "VINCOLI — al posto di una diagonale: uno spigolo e il suo angolo (A, 90) oppure un lato orizzontale o \
             verticale (A-B, orizzontale). Ognuno vale una misura e riceve il suo scarto.",
            "CONSTRAINTS — instead of a diagonal: a corner and its angle (A, 90) or a horizontal or vertical side \
             (A-B, horizontal). Each counts as a measure and gets its residual.",
            "RESTRICCIONES — en lugar de una diagonal: una esquina y su ángulo (A, 90) o un lado horizontal o vertical (A-B, horizontal). Cada una cuenta como una medida y recibe su residuo.",
            "CONTRAINTES — à la place d'une diagonale : un angle et sa valeur (A, 90) ou un côté horizontal ou vertical (A-B, horizontal). Chacune compte comme une mesure et reçoit son résidu.",
            "KISITLAR — köşegen yerine: bir köşe ve açısı (A, 90) veya yatay ya da dikey bir kenar (A-B, yatay). Her biri bir ölçü sayılır ve kendi artığını alır.",
            "RESTRIÇÕES — em vez de uma diagonal: um canto e o seu ângulo (A, 90) ou um lado horizontal ou vertical (A-B, horizontal). Cada uma conta como uma medida e recebe o seu resíduo.",
            "WARUNKI — zamiast przekątnej: narożnik i jego kąt (A, 90) lub bok poziomy albo pionowy (A-B, poziomy). Każdy liczy się jako pomiar i otrzymuje swoją odchyłkę.",
        )
        .to_string(),
        tr!(
            "PORTE E FINESTRE — scegli \"Aggiungi porta o finestra\" e clicca sul disegno, sul muro, nel punto dove \
             si trova. Poi scrivi la distanza dallo spigolo allo stipite, la larghezza e lo spessore del muro \
             misurato nella porta.",
            "DOORS AND WINDOWS — choose \"Add a door or window\" and click on the drawing, on the wall, where it is. \
             Then type the distance from the corner to the jamb, the width and the wall thickness measured in the door.",
            "PUERTAS Y VENTANAS — elija \"Añadir puerta o ventana\" y haga clic en el dibujo, sobre el muro, donde se encuentra. Después escriba la distancia de la esquina a la jamba, la anchura y el espesor del muro medido en la puerta.",
            "PORTES ET FENÊTRES — choisissez \"Ajouter une porte ou une fenêtre\" et cliquez sur le dessin, sur le mur, à l'endroit où elle se trouve. Saisissez ensuite la distance de l'angle au tableau, la largeur et l'épaisseur du mur mesurée dans la porte.",
            "KAPILAR VE PENCERELER — \"Kapı veya pencere ekle\" seçin ve çizimde, duvarın üzerinde, bulunduğu yere tıklayın. Sonra köşeden kasaya uzaklığı, genişliği ve kapıda ölçülen duvar kalınlığını girin.",
            "PORTAS E JANELAS — escolha \"Acrescentar porta ou janela\" e clique no desenho, sobre a parede, onde ela se encontra. Depois escreva a distância do canto à ombreira, a largura e a espessura da parede medida na porta.",
            "DRZWI I OKNA — wybierz \"Dodaj drzwi lub okno\" i kliknij na rysunku, na ścianie, w miejscu, gdzie się znajdują. Następnie wpisz odległość od narożnika do ościeża, szerokość i grubość ściany zmierzoną w drzwiach.",
        )
        .to_string(),
    ];
    if multi {
        v.push(
            tr!(
                "NOMI DEGLI SPIGOLI — sul disegno ogni spigolo ha solo la sua lettera e il vano il suo numero: lo spigolo \
                 B del vano 3 si scrive 3B (nella scheda del vano 3 basta B).",
                "CORNER NAMES — on the drawing each corner has its letter only and the room its number: corner B of room 3 \
                 is written 3B (in room 3's page B is enough).",
                "NOMBRES DE LAS ESQUINAS — en el dibujo cada esquina lleva solo su letra y la estancia su número: la esquina B de la estancia 3 se escribe 3B (en la ficha de la estancia 3 basta B).",
                "NOMS DES ANGLES — sur le dessin chaque angle porte seulement sa lettre et la pièce son numéro : l'angle B de la pièce 3 s'écrit 3B (dans la fiche de la pièce 3, B suffit).",
                "KÖŞE ADLARI — çizimde her köşede yalnızca harfi, odada ise numarası yazar: 3. odanın B köşesi 3B olarak yazılır (3. odanın sayfasında B yeterlidir).",
                "NOMES DOS CANTOS — no desenho cada canto tem só a sua letra e o compartimento o seu número: o canto B do compartimento 3 escreve-se 3B (na ficha do compartimento 3 basta B).",
                "NAZWY NAROŻNIKÓW — na rysunku każdy narożnik ma tylko swoją literę, a pomieszczenie swój numer: narożnik B pomieszczenia 3 zapisuje się jako 3B (na karcie pomieszczenia 3 wystarczy B).",
            )
            .to_string(),
        );
        v.push(
            tr!(
                "COLLEGARE I VANI — una porta tra due vani li collega da sola: cliccando sul muro tra i due, il plugin \
                 riconosce i due lati; servono lo spessore del muro e la distanza della porta dallo spigolo misurata \
                 in tutti e due i vani. Senza porta, usa \"Altre misure tra vani\": due spigoli (1B, 2D) per una \
                 distanza, oppure un lato e uno spigolo (1B-1C, 2A) per lo spessore di un muro.",
                "LINKING ROOMS — a door between two rooms links them by itself: click the wall between them and the \
                 plugin finds both sides; it needs the wall thickness and the door's distance from the corner measured \
                 in both rooms. With no door, use \"Other measures between rooms\": two corners (1B, 2D) for a distance, \
                 or a side and a corner (1B-1C, 2A) for a wall thickness.",
                "ENLAZAR ESTANCIAS — una puerta entre dos estancias las enlaza por sí sola: haga clic en el muro entre ellas y el plugin encuentra los dos lados; necesita el espesor del muro y la distancia de la puerta a la esquina medida en las dos estancias. Sin puerta, use \"Otras medidas entre estancias\": dos esquinas (1B, 2D) para una distancia, o un lado y una esquina (1B-1C, 2A) para un espesor de muro.",
                "LIER LES PIÈCES — une porte entre deux pièces les lie à elle seule : cliquez sur le mur qui les sépare et le plugin trouve les deux côtés ; il faut l'épaisseur du mur et la distance de la porte à l'angle mesurée dans les deux pièces. Sans porte, utilisez \"Autres mesures entre pièces\" : deux angles (1B, 2D) pour une distance, ou un côté et un angle (1B-1C, 2A) pour une épaisseur de mur.",
                "ODALARI BAĞLAMAK — iki oda arasındaki kapı onları tek başına bağlar: aradaki duvara tıklayın, eklenti iki kenarı bulur; duvar kalınlığı ve iki odada ölçülen kapının köşeye uzaklığı gerekir. Kapı yoksa \"Odalar arası diğer ölçüler\" kullanın: uzaklık için iki köşe (1B, 2D) veya duvar kalınlığı için bir kenar ve bir köşe (1B-1C, 2A).",
                "LIGAR COMPARTIMENTOS — uma porta entre dois compartimentos liga-os por si só: clique na parede entre eles e o plugin encontra os dois lados; é precisa a espessura da parede e a distância da porta ao canto medida nos dois compartimentos. Sem porta, use \"Outras medidas entre compartimentos\": dois cantos (1B, 2D) para uma distância, ou um lado e um canto (1B-1C, 2A) para uma espessura de parede.",
                "WIĄZANIE POMIESZCZEŃ — drzwi między dwoma pomieszczeniami same je wiążą: kliknij ścianę między nimi, a wtyczka znajdzie oba boki; potrzebna jest grubość ściany i odległość drzwi od narożnika zmierzona w obu pomieszczeniach. Bez drzwi użyj \"Inne pomiary między pomieszczeniami\": dwa narożniki (1B, 2D) dla odległości albo bok i narożnik (1B-1C, 2A) dla grubości ściany.",
            )
            .to_string(),
        );
    }
    v.push(
        tr!(
            "MISURE MANCANTI E UNITÀ — una casella vuota non viene usata; se mancano misure (mobili, ostacoli) il \
             plugin completa la forma con gli angoli del disegno e lo segnala in rosso (si può spegnere nelle \
             Opzioni). Le misure vanno nelle unità del disegno; la virgola decimale va bene.",
            "MISSING MEASURES AND UNITS — an empty box is not used; if measures are missing (furniture, obstacles) the \
             plugin finishes the shape with the drawing's angles and says so in red (can be turned off in Options). \
             Type the measures in drawing units; a decimal comma is fine.",
            "MEDIDAS QUE FALTAN Y UNIDADES — una casilla vacía no se usa; si faltan medidas (muebles, obstáculos) el plugin completa la forma con los ángulos del dibujo y lo indica en rojo (se puede desactivar en Opciones). Escriba las medidas en unidades del dibujo; la coma decimal vale.",
            "MESURES MANQUANTES ET UNITÉS — une case vide n'est pas utilisée ; s'il manque des mesures (meubles, obstacles), le plugin complète la forme avec les angles du dessin et le signale en rouge (désactivable dans Options). Saisissez les mesures en unités du dessin ; la virgule décimale est acceptée.",
            "EKSİK ÖLÇÜLER VE BİRİMLER — boş bir kutu kullanılmaz; ölçü eksikse (mobilya, engeller) eklenti şekli çizimin açılarıyla tamamlar ve bunu kırmızıyla belirtir (Seçenekler'den kapatılabilir). Ölçüleri çizim biriminde girin; ondalık virgül kabul edilir.",
            "MEDIDAS EM FALTA E UNIDADES — uma caixa vazia não é usada; se faltarem medidas (móveis, obstáculos) o plugin completa a forma com os ângulos do desenho e assinala-o a vermelho (pode desativar-se em Opções). Escreva as medidas em unidades do desenho; a vírgula decimal é aceite.",
            "BRAKUJĄCE POMIARY I JEDNOSTKI — puste pole nie jest używane; jeśli brakuje pomiarów (meble, przeszkody), wtyczka uzupełnia kształt kątami z rysunku i zaznacza to na czerwono (można wyłączyć w Opcjach). Wpisuj pomiary w jednostkach rysunku; przecinek dziesiętny jest akceptowany.",
        )
        .to_string(),
    );
    v
}

// ── Doors by a click on the drawing ─────────────────────────────────────────

/// The wall a door or window was clicked on: a side of a room, from the
/// corner nearer the click, and — when another room lies just behind it —
/// the facing side of that room, from the corner facing the first one.
#[derive(Clone, Debug, PartialEq)]
pub struct DoorSpot {
    pub room: usize,
    pub from: usize,
    pub to: usize,
    pub other: Option<(usize, usize, usize)>,
}

impl DoorSpot {
    pub fn encode(&self) -> String {
        let o = match self.other {
            Some((q, k, l)) => format!("{q},{k},{l}"),
            None => "-".into(),
        };
        format!("{},{},{};{o}", self.room, self.from, self.to)
    }

    pub fn decode(s: &str) -> Option<Self> {
        let mut parts = s.split(';');
        let three = |t: &str| -> Option<(usize, usize, usize)> {
            let v: Vec<usize> = t.split(',').filter_map(|x| x.trim().parse().ok()).collect();
            (v.len() == 3).then(|| (v[0], v[1], v[2]))
        };
        let (room, from, to) = three(parts.next()?)?;
        let other = parts.next().and_then(three);
        Some(DoorSpot { room, from, to, other })
    }

    /// Whether the indices exist in `survey` and name real sides.
    fn fits(&self, survey: &Survey) -> bool {
        let side = |r: usize, i: usize, j: usize| {
            survey.rooms.get(r).is_some_and(|room| {
                let n = room.len();
                i < n && j < n && (j == (i + 1) % n || i == (j + 1) % n)
            })
        };
        side(self.room, self.from, self.to) && self.other.is_none_or(|(q, k, l)| q != self.room && side(q, k, l))
    }
}

/// The side nearest to `pt` and, if any, the facing side of the room behind
/// that wall (roughly parallel, opposite the click, on the outside).
pub fn find_spot(rooms: &[Vec<P2>], pt: P2) -> Option<DoorSpot> {
    let longest = rooms
        .iter()
        .flat_map(|r| (0..r.len()).map(move |i| solve::dist(r[i], r[(i + 1) % r.len()])))
        .fold(0.0f64, f64::max);
    let mut best: Option<(f64, usize, usize)> = None;
    for (r, room) in rooms.iter().enumerate() {
        for i in 0..room.len() {
            let d = crate::labels::seg_dist(pt, room[i], room[(i + 1) % room.len()]);
            if best.is_none_or(|b| d < b.0) {
                best = Some((d, r, i));
            }
        }
    }
    let (d, r, i) = best?;
    if d > longest * 0.3 {
        return None;
    }
    let j = (i + 1) % rooms[r].len();
    let (a, b) = (rooms[r][i], rooms[r][j]);
    let len = solve::dist(a, b);
    if len <= 0.0 {
        return None;
    }
    let (from, to) = if solve::dist(pt, a) <= solve::dist(pt, b) { (i, j) } else { (j, i) };
    let u = [(b[0] - a[0]) / len, (b[1] - a[1]) / len];
    let on_first = solve::foot(pt, a, b);

    let mut other: Option<(f64, usize, usize, usize)> = None;
    for (q, room) in rooms.iter().enumerate() {
        if q == r {
            continue;
        }
        for k in 0..room.len() {
            let l = (k + 1) % room.len();
            let (c, e) = (room[k], room[l]);
            let le = solve::dist(c, e);
            if le <= 0.0 {
                continue;
            }
            let v = [(e[0] - c[0]) / le, (e[1] - c[1]) / le];
            if (u[0] * v[1] - u[1] * v[0]).abs() > 0.5 {
                continue; // not parallel enough
            }
            let t = ((pt[0] - c[0]) * v[0] + (pt[1] - c[1]) * v[1]) / le;
            if !(0.0..=1.0).contains(&t) {
                continue; // the click is not in front of this side
            }
            let on_second = solve::foot(pt, c, e);
            let gap = solve::dist(on_first, on_second);
            if gap > longest * 0.2 {
                continue;
            }
            // The wall lies outside both rooms.
            if gap > longest * 1e-6 {
                let mid = [(on_first[0] + on_second[0]) / 2.0, (on_first[1] + on_second[1]) / 2.0];
                if commands::point_in_polygon(&rooms[r], mid) || commands::point_in_polygon(room, mid) {
                    continue;
                }
            }
            if other.is_none_or(|o| gap < o.0) {
                let near = rooms[r][from];
                let (f2, t2) = if solve::dist(near, c) <= solve::dist(near, e) { (k, l) } else { (l, k) };
                other = Some((gap, q, f2, t2));
            }
        }
    }
    Some(DoorSpot { room: r, from, to, other: other.map(|o| (o.1, o.2, o.3)) })
}

/// Click on a wall to place a door or window; then the measures dialog
/// opens again on the door's page.
pub struct PickDoorTool {
    rooms: Vec<Vec<P2>>,
    z: f64,
    /// The rooms' handles, as the dialog commands take them.
    key: String,
    msg: Option<String>,
}

impl PickDoorTool {
    pub fn new(rooms: Vec<Vec<P2>>, z: f64, key: String) -> Self {
        PickDoorTool { rooms, z, key, msg: None }
    }

    fn pick(&mut self, pt: [f64; 3]) -> CommandStep {
        match find_spot(&self.rooms, [pt[0], pt[1]]) {
            Some(s) => deferred("RV_PORTAOPEN", format!("{}|{}", self.key, s.encode())),
            None => {
                self.msg = Some(tr!("lì non c'è il muro di un vano scelto", "no wall of a chosen room there", "ahí no hay ningún muro de una estancia elegida", "pas de mur d'une pièce choisie à cet endroit", "orada seçilen bir odanın duvarı yok", "não há aí nenhuma parede de um compartimento escolhido", "nie ma tu ściany wybranego pomieszczenia").into());
                CommandStep::NeedPoint
            }
        }
    }
}

impl InteractiveCommand for PickDoorTool {
    fn prompt(&self) -> String {
        with_msg(
            &self.msg,
            tr!(
                "Porta o finestra: clicca sul muro nel punto dove si trova (tra due vani il plugin riconosce i due lati, \
                 in verde). Invio per tornare alla finestra senza aggiungerla"
                    .to_string(),
                "Door or window: click the wall where it is (between two rooms the plugin finds both sides, in green). \
                 Enter to go back to the window without adding it"
                    .to_string(),
                "Puerta o ventana: haga clic en el muro donde se encuentra (entre dos estancias el plugin encuentra los dos lados, en verde). Intro para volver a la ventana sin añadirla"
                    .to_string(),
                "Porte ou fenêtre : cliquez sur le mur à l'endroit où elle se trouve (entre deux pièces le plugin trouve les deux côtés, en vert). Entrée pour revenir à la fenêtre sans l'ajouter"
                    .to_string(),
                "Kapı veya pencere: bulunduğu yerde duvara tıklayın (iki oda arasında eklenti iki kenarı bulur, yeşil). Eklemeden pencereye dönmek için Enter"
                    .to_string(),
                "Porta ou janela: clique na parede onde ela se encontra (entre dois compartimentos o plugin encontra os dois lados, a verde). Enter para voltar à janela sem a acrescentar"
                    .to_string(),
                "Drzwi lub okno: kliknij ścianę w miejscu, gdzie się znajdują (między dwoma pomieszczeniami wtyczka znajduje oba boki, na zielono). Enter, aby wrócić do okna bez dodawania"
                    .to_string(),
            ),
        )
    }
    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        self.pick(pt)
    }
    fn needs_object_pick(&self) -> bool {
        true
    }
    fn on_object_pick(&mut self, _handle: Handle, pt: [f64; 3]) -> CommandStep {
        self.pick(pt)
    }
    fn on_cursor_move(&mut self, pt: [f64; 3]) -> Vec<PreviewWire> {
        let Some(s) = find_spot(&self.rooms, [pt[0], pt[1]]) else { return Vec::new() };
        let seg = |r: usize, i: usize, j: usize| {
            let (a, b) = (self.rooms[r][i], self.rooms[r][j]);
            PreviewWire::with_color(vec![[a[0], a[1], self.z], [b[0], b[1], self.z]], GREEN)
        };
        let mut w = vec![seg(s.room, s.from, s.to)];
        if let Some((q, k, l)) = s.other {
            w.push(seg(q, k, l));
        }
        w
    }
    fn on_enter(&mut self) -> CommandStep {
        deferred("RV_VANOREOPEN", self.key.clone())
    }
}

/// Click on the outer outline surveyed exactly (a closed polyline); then
/// the measures dialog opens again on the outer walls page.
pub struct PickOuterTool {
    key: String,
}

impl PickOuterTool {
    pub fn new(key: String) -> Self {
        PickOuterTool { key }
    }
}

impl InteractiveCommand for PickOuterTool {
    fn prompt(&self) -> String {
        tr!(
            "Poligono esterno: clicca sulla polilinea chiusa rilevata con la stazione totale (resterà ferma). Invio per tornare alla finestra",
            "Outer outline: click the closed polyline surveyed with the total station (it will not move). Enter to go back to the window",
            "Polígono exterior: haga clic en la polilínea cerrada levantada con la estación total (no se moverá). Intro para volver a la ventana",
            "Polygone extérieur : cliquez sur la polyligne fermée relevée à la station totale (elle ne bougera pas). Entrée pour revenir à la fenêtre",
            "Dış poligon: total station ile ölçülen kapalı çoklu çizgiye tıklayın (yerinden oynamaz). Pencereye dönmek için Enter",
            "Polígono exterior: clique na polilinha fechada levantada com a estação total (não se moverá). Enter para voltar à janela",
            "Obrys zewnętrzny: kliknij zamkniętą polilinię zmierzoną tachimetrem (nie zostanie przesunięta). Enter, aby wrócić do okna",
        )
        .to_string()
    }
    fn on_point(&mut self, _pt: [f64; 3]) -> CommandStep {
        CommandStep::NeedPoint
    }
    fn needs_object_pick(&self) -> bool {
        true
    }
    fn on_object_pick(&mut self, handle: Handle, _pt: [f64; 3]) -> CommandStep {
        deferred("RV_ESTERNOSET", format!("{}|{:X}", self.key, handle.value()))
    }
    fn on_enter(&mut self) -> CommandStep {
        deferred("RV_VANOREOPEN", self.key.clone())
    }
}

// ── Measures dialog ─────────────────────────────────────────────────────────

fn tf(id: String, label: String, default: String) -> DialogControl {
    DialogControl::Text { id, label, default }
}

/// Lay controls out `per_row` to a row; the last row is padded with empty
/// texts so every box keeps the same width.
fn grid(controls: Vec<DialogControl>, per_row: usize) -> Vec<DialogControl> {
    let mut out = Vec::new();
    let mut it = controls.into_iter().peekable();
    while it.peek().is_some() {
        let mut row: Vec<DialogControl> = it.by_ref().take(per_row).collect();
        while row.len() < per_row {
            row.push(info(String::new()));
        }
        out.push(DialogControl::row(row));
    }
    out
}

fn get_text(m: &HashMap<String, DialogValue>, id: &str) -> String {
    match m.get(id) {
        Some(DialogValue::Text(s)) => s.trim().to_string(),
        Some(DialogValue::Number(v)) => v.to_string(),
        _ => String::new(),
    }
}

fn get_choice(m: &HashMap<String, DialogValue>, id: &str, default: usize) -> usize {
    match m.get(id) {
        Some(DialogValue::Choice { index, .. }) => *index,
        _ => default,
    }
}

fn filled(row: &[String], cols: &[usize]) -> bool {
    cols.iter().any(|&c| row.get(c).is_some_and(|s| !s.trim().is_empty()))
}

fn cell(row: &[String], c: usize) -> String {
    row.get(c).cloned().unwrap_or_default()
}

/// Where the measures dialog opens.
#[derive(Clone, Debug, PartialEq)]
pub enum Start {
    Hub,
    Door(DoorSpot),
    Outer,
}

#[derive(Clone, Debug, PartialEq)]
enum Page {
    Hub,
    Room(usize),
    Links,
    Openings,
    Options,
    Door(DoorSpot),
    Outer,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
    Room(usize),
    AddDoor,
    Links,
    Openings,
    Options,
    PickOuter,
    Outer,
    Compute,
}

/// The measures dialog: a small hub listing the rooms with their state,
/// from which one room at a time is opened (its sides, diagonals and
/// constraints), doors and windows are added by a click on the drawing,
/// and the calculation is started. Each page stays small whatever the
/// number of rooms.
///
/// Plain text boxes rather than table controls: the host sizes its dialog
/// window to the content and measures a table's rows at their minimum
/// height, so a table opens squashed.
pub struct MeasureDialog {
    handles: Vec<Handle>,
    survey: Survey,
    msg: Option<String>,
    saved: Saved,
    page: Page,
    /// What was typed on the door page, kept when it has to be shown again.
    draft: HashMap<String, DialogValue>,
}

impl MeasureDialog {
    /// `saved`: what the user typed last time for these rooms.
    pub fn new(handles: Vec<Handle>, survey: &Survey, saved: &Saved, msg: Option<String>) -> Self {
        MeasureDialog { handles, survey: survey.clone(), msg, saved: saved.clone(), page: Page::Hub, draft: HashMap::new() }
    }

    /// Open on the door page for a wall clicked on the drawing.
    pub fn starting(mut self, start: Start) -> Self {
        match start {
            Start::Door(spot) if spot.fits(&self.survey) => self.page = Page::Door(spot),
            Start::Outer if self.survey.outer >= 3 => self.page = Page::Outer,
            _ => {}
        }
        self
    }

    fn key(&self) -> String {
        self.handles.iter().map(|h| format!("{:X}", h.value())).collect::<Vec<_>>().join(",")
    }

    fn keep(&self) {
        commands::remember(&self.handles, self.saved.clone());
    }

    fn multi(&self) -> bool {
        self.survey.multi()
    }

    fn room_name(&self, r: usize) -> String {
        if self.multi() {
            tr!(format!("vano {}", r + 1), format!("room {}", r + 1), format!("estancia {}", r + 1), format!("pièce {}", r + 1), format!("oda {}", r + 1), format!("compartimento {}", r + 1), format!("pomieszczenie {}", r + 1))
        } else {
            tr!("vano".to_string(), "room".to_string(), "estancia".to_string(), "pièce".to_string(), "oda".to_string(), "compartimento".to_string(), "pomieszczenie".to_string())
        }
    }

    /// First row of room `r` in the sides table.
    fn offset(&self, r: usize) -> usize {
        self.survey.rooms[..r].iter().map(|x| x.len()).sum()
    }

    /// The room a diagonal row belongs to (both corners in it); with one
    /// room, every row.
    fn other_room(&self, row: &[String]) -> Option<usize> {
        if !self.multi() {
            return Some(0);
        }
        let a = self.survey.parse_corner(&cell(row, 0))?;
        let b = self.survey.parse_corner(&cell(row, 1))?;
        (a.0 == b.0).then_some(a.0)
    }

    /// The room a constraint row belongs to.
    fn con_room(&self, row: &[String]) -> usize {
        let at = cell(row, 0);
        self.survey
            .parse_corner(&at)
            .map(|c| c.0)
            .or_else(|| self.survey.parse_room_side(&at).map(|s| s.0))
            .or_else(|| self.survey.parse_side(&at).and_then(|(a, _)| self.survey.rooms_of(a).first().copied()))
            .unwrap_or(0)
    }

    /// Diagonal rows room by room, and the rows between rooms.
    fn split_others(&self) -> (Vec<Vec<Vec<String>>>, Vec<Vec<String>>) {
        let mut per = vec![Vec::new(); self.survey.rooms.len()];
        let mut links = Vec::new();
        for row in &self.saved.others {
            match self.other_room(row) {
                Some(r) => per[r].push(row.clone()),
                None => links.push(row.clone()),
            }
        }
        (per, links)
    }

    fn split_cons(&self) -> Vec<Vec<Vec<String>>> {
        let mut per = vec![Vec::new(); self.survey.rooms.len()];
        for row in &self.saved.constraints {
            per[self.con_room(row)].push(row.clone());
        }
        per
    }

    /// In room `r`'s page a bare letter means that room's corner: A → 3A.
    fn norm(&self, r: usize, s: &str) -> String {
        if !self.multi() {
            return s.trim().to_string();
        }
        s.split(['-', '–', '—', '/'])
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .map(|p| {
                if p.chars().all(|c| c.is_ascii_alphabetic()) {
                    format!("{}{}", r + 1, p.to_ascii_uppercase())
                } else {
                    p.to_ascii_uppercase()
                }
            })
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Whether room `r` has what it needs, and its state in words.
    fn room_status(&self, r: usize) -> (bool, String) {
        let n = self.survey.rooms[r].len();
        let off = self.offset(r);
        let sides = (0..n).filter(|i| self.saved.sides.get(off + i).is_some_and(|row| !cell(row, 2).is_empty())).count();
        let (per, _) = self.split_others();
        let diags = per[r].iter().filter(|row| !cell(row, 3).is_empty()).count();
        let cons = self.split_cons()[r].iter().filter(|row| !cell(row, 0).is_empty() && !cell(row, 1).is_empty()).count();
        // Sides along the fixed outline: two of them place the room and fix
        // its turn, so fewer diagonals are needed.
        let k = self.outer_rows_of(r);
        let need = if k >= 2 { n.saturating_sub(2 * k) } else { n.saturating_sub(3) };
        let have = diags + cons;
        if sides == n && have >= need {
            let extra = have - need;
            let s = if extra > 0 {
                tr!(format!("✔ completo, {extra} di controllo"), format!("✔ complete, {extra} as a check"), format!("✔ completa, {extra} de control"), format!("✔ complète, {extra} de contrôle"), format!("✔ tamam, {extra} kontrol"), format!("✔ completo, {extra} de controlo"), format!("✔ kompletne, {extra} kontrolnych"))
            } else {
                tr!("✔ completo (senza misura di controllo)".to_string(), "✔ complete (no check measure)".to_string(), "✔ completa (sin medida de control)".to_string(), "✔ complète (sans mesure de contrôle)".to_string(), "✔ tamam (kontrol ölçüsü yok)".to_string(), "✔ completo (sem medida de controlo)".to_string(), "✔ kompletne (bez pomiaru kontrolnego)".to_string())
            };
            (true, s)
        } else {
            let mut parts = Vec::new();
            if sides < n {
                parts.push(tr!(format!("{} lati", n - sides), format!("{} sides", n - sides), format!("{} lados", n - sides), format!("{} côtés", n - sides), format!("{} kenar", n - sides), format!("{} lados", n - sides), format!("{} boków", n - sides)));
            }
            if have < need {
                parts.push(tr!(format!("{} diagonali o vincoli", need - have), format!("{} diagonals or constraints", need - have), format!("{} diagonales o restricciones", need - have), format!("{} diagonales ou contraintes", need - have), format!("{} köşegen veya kısıt", need - have), format!("{} diagonais ou restrições", need - have), format!("{} przekątnych lub warunków", need - have)));
            }
            let and = tr!(" e ", " and ", " y ", " et ", " ve ", " e ", " i ");
            (false, tr!(format!("✘ mancano {}", parts.join(and)), format!("✘ missing {}", parts.join(and)), format!("✘ faltan {}", parts.join(and)), format!("✘ il manque {}", parts.join(and)), format!("✘ eksik {}", parts.join(and)), format!("✘ faltam {}", parts.join(and)), format!("✘ brakuje {}", parts.join(and))))
        }
    }

    /// Outer-wall rows on room `r`'s sides.
    fn outer_rows_of(&self, r: usize) -> usize {
        if self.survey.outer < 3 {
            return 0;
        }
        self.saved.outer_walls.iter().filter(|row| self.survey.parse_room_side(&cell(row, 0)).is_some_and(|x| x.0 == r)).count()
    }

    /// Rooms (1-based) the measures do not tie to room 1 yet.
    fn unlinked(&self) -> Result<Vec<usize>, String> {
        let p = self.survey.parse(&self.saved, &commands::parse_num)?;
        let meas: Vec<solve::Meas> = p.measures.iter().map(|m| m.meas).collect();
        let groups = solve::groups(self.survey.points.len(), &meas);
        let group_of = |r: usize| groups.iter().position(|g| g.contains(&self.survey.point_of[r][0]));
        let main = group_of(0);
        Ok((1..self.survey.rooms.len()).filter(|&r| group_of(r) != main).map(|r| r + 1).collect())
    }

    /// One line per door or window: "P1 porta: 1B-1C ↔ 2D-2A · 0,90 · muro 0,30".
    fn opening_lines(&self) -> Vec<String> {
        let mut out: Vec<(String, String)> = Vec::new();
        for row in self.saved.openings.iter().filter(|r| filled(r, &[0, 1, 2, 3, 4, 5])) {
            let name = cell(row, 0);
            if let Some(line) = out.iter_mut().find(|(n, _)| n.eq_ignore_ascii_case(&name)) {
                line.1 = line.1.replacen(" · ", &format!(" ↔ {} · ", cell(row, 1)), 1);
                if !cell(row, 5).is_empty() && !line.1.contains(tr!("muro", "wall", "muro", "mur", "duvar", "parede", "ściana")) {
                    line.1 += &format!(" · {} {}", tr!("muro", "wall", "muro", "mur", "duvar", "parede", "ściana"), cell(row, 5));
                }
                continue;
            }
            let mut s = format!("{name} {}: {} · {}", cell(row, 2), cell(row, 1), cell(row, 4));
            if !cell(row, 5).is_empty() {
                s += &format!(" · {} {}", tr!("muro", "wall", "muro", "mur", "duvar", "parede", "ściana"), cell(row, 5));
            }
            out.push((name, s));
        }
        out.into_iter().map(|x| x.1).collect()
    }

    /// The hub's list: what each entry does, its label, and the one
    /// proposed (the first thing still to do).
    fn actions(&self) -> (Vec<Action>, Vec<String>, usize) {
        let mut acts = Vec::new();
        let mut labels = Vec::new();
        let mut first_open = None;
        for r in 0..self.survey.rooms.len() {
            let (ok, status) = self.room_status(r);
            if !ok && first_open.is_none() {
                first_open = Some(acts.len());
            }
            acts.push(Action::Room(r));
            labels.push(tr!(
                format!("Misure del {} — {status}", self.room_name(r)),
                format!("Measures of {} — {status}", self.room_name(r)),
                format!("Medidas de {} — {status}", self.room_name(r)),
                format!("Mesures de la {} — {status}", self.room_name(r)),
                format!("{} ölçüleri — {status}", self.room_name(r)),
                format!("Medidas do {} — {status}", self.room_name(r)),
                format!("Pomiary: {} — {status}", self.room_name(r)),
            ));
        }
        let door = acts.len();
        acts.push(Action::AddDoor);
        labels.push(tr!("Aggiungi porta o finestra (clic sul disegno)", "Add a door or window (click on the drawing)", "Añadir puerta o ventana (clic en el dibujo)", "Ajouter une porte ou une fenêtre (clic sur le dessin)", "Kapı veya pencere ekle (çizime tıkla)", "Acrescentar porta ou janela (clique no desenho)", "Dodaj drzwi lub okno (kliknij na rysunku)").to_string());
        if self.saved.openings.iter().any(|r| filled(r, &[0, 1, 2, 3, 4, 5])) {
            acts.push(Action::Openings);
            labels.push(tr!("Modifica o elimina porte e finestre", "Edit or delete doors and windows", "Modificar o borrar puertas y ventanas", "Modifier ou supprimer portes et fenêtres", "Kapı ve pencereleri düzenle veya sil", "Alterar ou apagar portas e janelas", "Zmień lub usuń drzwi i okna").to_string());
        }
        if self.survey.outer >= 3 {
            acts.push(Action::Outer);
            labels.push(tr!(
                "Muri esterni: spessore verso il poligono esterno fisso",
                "Outer walls: thickness towards the fixed outer outline",
                "Muros exteriores: espesor hacia el polígono exterior fijo",
                "Murs extérieurs : épaisseur vers le polygone extérieur fixe",
                "Dış duvarlar: sabit dış poligona olan kalınlık",
                "Paredes exteriores: espessura até ao polígono exterior fixo",
                "Ściany zewnętrzne: grubość do stałego obrysu zewnętrznego",
            )
            .to_string());
        } else {
            acts.push(Action::PickOuter);
            labels.push(tr!(
                "Poligono esterno fisso, da rilievo celerimetrico (clic sul disegno)",
                "Fixed outer outline, from a total-station survey (click on the drawing)",
                "Polígono exterior fijo, de levantamiento taquimétrico (clic en el dibujo)",
                "Polygone extérieur fixe, issu d'un relevé tachéométrique (clic sur le dessin)",
                "Sabit dış poligon, takeometrik ölçümden (çizime tıkla)",
                "Polígono exterior fixo, de levantamento taqueométrico (clique no desenho)",
                "Stały obrys zewnętrzny z pomiaru tachimetrem (kliknij na rysunku)",
            )
            .to_string());
        }
        if self.multi() {
            acts.push(Action::Links);
            labels.push(tr!("Altre misure tra vani (senza porta)", "Other measures between rooms (no door)", "Otras medidas entre estancias (sin puerta)", "Autres mesures entre pièces (sans porte)", "Odalar arası diğer ölçüler (kapısız)", "Outras medidas entre compartimentos (sem porta)", "Inne pomiary między pomieszczeniami (bez drzwi)").to_string());
        }
        acts.push(Action::Options);
        labels.push(tr!("Opzioni (testi, tolleranza, quote, libretto…)", "Options (texts, tolerance, dimensions, book…)", "Opciones (textos, tolerancia, cotas, libro…)", "Options (textes, tolérance, cotes, carnet…)", "Seçenekler (yazılar, tolerans, ölçülendirme, defter…)", "Opções (textos, tolerância, cotas, caderno…)", "Opcje (teksty, tolerancja, wymiary, dziennik…)").to_string());
        let compute = acts.len();
        acts.push(Action::Compute);
        labels.push(tr!("▶ Calcola", "▶ Compute", "▶ Calcular", "▶ Calculer", "▶ Hesapla", "▶ Calcular", "▶ Oblicz").to_string());
        let default = first_open.unwrap_or_else(|| {
            if self.multi() && self.unlinked().is_ok_and(|u| !u.is_empty()) {
                door
            } else {
                compute
            }
        });
        (acts, labels, default)
    }

    fn hub(&self) -> DialogDef {
        let n_rooms = self.survey.rooms.len();
        let mut def = DialogDef::new(if self.multi() {
            tr!(format!("Rilievo di {n_rooms} vani"), format!("Survey of {n_rooms} rooms"), format!("Levantamiento de {n_rooms} estancias"), format!("Relevé de {n_rooms} pièces"), format!("{n_rooms} odanın ölçümü"), format!("Levantamento de {n_rooms} compartimentos"), format!("Pomiar {n_rooms} pomieszczeń"))
        } else {
            tr!("Rilievo del vano".to_string(), "Room survey".to_string(), "Levantamiento de la estancia".to_string(), "Relevé de la pièce".to_string(), "Oda ölçümü".to_string(), "Levantamento do compartimento".to_string(), "Pomiar pomieszczenia".to_string())
        });
        if let Some(m) = &self.msg {
            def = def.with_control(info(tr!(format!("⚠ Da correggere: {m}."), format!("⚠ To fix: {m}."), format!("⚠ Por corregir: {m}."), format!("⚠ À corriger : {m}."), format!("⚠ Düzeltilecek: {m}."), format!("⚠ A corrigir: {m}."), format!("⚠ Do poprawienia: {m}."))));
        }
        let lines: Vec<DialogControl> = (0..n_rooms)
            .map(|r| {
                let n = self.survey.rooms[r].len();
                let name = self.room_name(r);
                let name = name[..1].to_uppercase() + &name[1..];
                info(tr!(
                    format!("{name} ({n} spigoli): {}", self.room_status(r).1),
                    format!("{name} ({n} corners): {}", self.room_status(r).1),
                    format!("{name} ({n} esquinas): {}", self.room_status(r).1),
                    format!("{name} ({n} angles) : {}", self.room_status(r).1),
                    format!("{name} ({n} köşe): {}", self.room_status(r).1),
                    format!("{name} ({n} cantos): {}", self.room_status(r).1),
                    format!("{name} ({n} narożników): {}", self.room_status(r).1),
                ))
            })
            .collect();
        for row in grid(lines, if n_rooms > 6 { 2 } else { 1 }) {
            def = def.with_control(row);
        }
        if self.multi() {
            def = def.with_control(info(match self.unlinked() {
                Ok(u) if u.is_empty() => tr!("Collegamenti: ✔ tutti i vani sono collegati tra loro.".to_string(), "Links: ✔ all rooms are linked.".to_string(), "Enlaces: ✔ todas las estancias están enlazadas.".to_string(), "Liaisons : ✔ toutes les pièces sont liées.".to_string(), "Bağlantılar: ✔ tüm odalar bağlı.".to_string(), "Ligações: ✔ todos os compartimentos estão ligados.".to_string(), "Powiązania: ✔ wszystkie pomieszczenia są powiązane.".to_string()),
                Ok(u) => {
                    let list = u.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
                    tr!(
                        format!("Collegamenti: ✘ vani non ancora collegati al vano 1: {list}. Aggiungi una porta sul muro che li separa."),
                        format!("Links: ✘ rooms not yet linked to room 1: {list}. Add a door on the wall between them."),
                        format!("Enlaces: ✘ estancias aún no enlazadas con la estancia 1: {list}. Añada una puerta en el muro que las separa."),
                        format!("Liaisons : ✘ pièces pas encore liées à la pièce 1 : {list}. Ajoutez une porte sur le mur qui les sépare."),
                        format!("Bağlantılar: ✘ 1. odaya henüz bağlı olmayan odalar: {list}. Onları ayıran duvara bir kapı ekleyin."),
                        format!("Ligações: ✘ compartimentos ainda não ligados ao compartimento 1: {list}. Acrescente uma porta na parede que os separa."),
                        format!("Powiązania: ✘ pomieszczenia jeszcze niepowiązane z pomieszczeniem 1: {list}. Dodaj drzwi w ścianie, która je dzieli."),
                    )
                }
                Err(e) => format!("⚠ {e}"),
            }));
        }
        if self.survey.outer >= 3 {
            let n = self.saved.outer_walls.iter().filter(|r| !cell(r, 0).trim().is_empty()).count();
            let t = if self.saved.outer_t.trim().is_empty() { "?".to_string() } else { self.saved.outer_t.trim().to_string() };
            let m = self.survey.outer;
            def = def.with_control(info(tr!(
                format!("Poligono esterno fisso ({m} vertici, non viene modificato): {n} lati dei vani lungo i muri esterni, spessore {t}."),
                format!("Fixed outer outline ({m} corners, never changed): {n} room sides along the outer walls, thickness {t}."),
                format!("Polígono exterior fijo ({m} vértices, no se modifica): {n} lados de estancias junto a los muros exteriores, espesor {t}."),
                format!("Polygone extérieur fixe ({m} sommets, jamais modifié) : {n} côtés de pièces le long des murs extérieurs, épaisseur {t}."),
                format!("Sabit dış poligon ({m} köşe, değiştirilmez): dış duvarlar boyunca {n} oda kenarı, kalınlık {t}."),
                format!("Polígono exterior fixo ({m} vértices, não é alterado): {n} lados de compartimentos junto às paredes exteriores, espessura {t}."),
                format!("Stały obrys zewnętrzny ({m} wierzchołków, nie jest zmieniany): {n} boków pomieszczeń wzdłuż ścian zewnętrznych, grubość {t}."),
            )));
        }
        let doors = self.opening_lines();
        if !doors.is_empty() {
            let shown: Vec<String> = doors.iter().take(8).cloned().collect();
            let more = doors.len() - shown.len();
            let mut text = format!("{}: {}", tr!("Porte e finestre", "Doors and windows", "Puertas y ventanas", "Portes et fenêtres", "Kapılar ve pencereler", "Portas e janelas", "Drzwi i okna"), shown.join("   |   "));
            if more > 0 {
                text += &tr!(format!("   … e altre {more}"), format!("   … and {more} more"), format!("   … y {more} más"), format!("   … et {more} de plus"), format!("   … ve {more} tane daha"), format!("   … e mais {more}"), format!("   … i {more} więcej"));
            }
            def = def.with_control(info(text));
        }
        let (_, labels, default) = self.actions();
        def.with_control(DialogControl::Choice {
            id: "do".into(),
            label: tr!("Cosa vuoi fare?", "What next?", "¿Qué quiere hacer?", "Que voulez-vous faire ?", "Ne yapmak istiyorsunuz?", "O que quer fazer?", "Co chcesz zrobić?").into(),
            options: labels,
            default_index: default,
        })
        .with_control(help_section("help", tr!("Come si usa (aiuto) ▸", "How it works (help) ▸", "Cómo funciona (ayuda) ▸", "Mode d'emploi (aide) ▸", "Nasıl çalışır (yardım) ▸", "Como funciona (ajuda) ▸", "Jak to działa (pomoc) ▸"), how_to_measure(self.multi())))
        .submit_label(tr!("Continua ▸", "Continue ▸", "Continuar ▸", "Continuer ▸", "Devam ▸", "Continuar ▸", "Dalej ▸"))
        .cancel_label(tr!("Chiudi", "Close", "Cerrar", "Fermer", "Kapat", "Fechar", "Zamknij"))
        .width(660.0)
    }

    /// The diagonal and constraint rows shown on room `r`'s page: the ones
    /// kept, then empty spare rows.
    fn room_view(&self, r: usize) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
        let (per, _) = self.split_others();
        let mut others: Vec<Vec<String>> = per[r].iter().filter(|x| filled(x, &[0, 1, 3])).cloned().collect();
        others.extend(std::iter::repeat_n(vec![String::new(); 4], 2));
        let mut cons: Vec<Vec<String>> = self.split_cons()[r].iter().filter(|x| filled(x, &[0, 1])).cloned().collect();
        let total = (cons.len() + 1).max(3).div_ceil(3) * 3;
        cons.resize(total, vec![String::new(); 2]);
        (others, cons)
    }

    fn links_view(&self) -> Vec<Vec<String>> {
        let (_, links) = self.split_others();
        let mut v: Vec<Vec<String>> = links.into_iter().filter(|x| filled(x, &[0, 1, 3])).collect();
        v.extend(std::iter::repeat_n(vec![String::new(); 4], 4));
        v
    }

    fn openings_view(&self) -> Vec<Vec<String>> {
        let mut v: Vec<Vec<String>> = self.saved.openings.iter().filter(|x| filled(x, &[0, 1, 2, 3, 4, 5])).cloned().collect();
        v.extend(std::iter::repeat_n(vec![String::new(); 6], 2));
        v
    }

    fn distance_rows(rows: &[Vec<String>]) -> Vec<DialogControl> {
        let mut cells = Vec::new();
        for (k, r) in rows.iter().enumerate() {
            let drawn = r.get(2).filter(|d| !d.is_empty()).map(|d| format!(" ({d})")).unwrap_or_default();
            cells.push(tf(format!("o{k}a"), tr!("Da", "From", "Desde", "De", "Başlangıç", "De", "Od").into(), cell(r, 0)));
            cells.push(tf(format!("o{k}b"), tr!("A", "To", "Hasta", "À", "Bitiş", "Até", "Do").into(), cell(r, 1)));
            cells.push(tf(format!("o{k}m"), format!("{}{drawn}", tr!("Misurata", "Measured", "Medida", "Mesurée", "Ölçülen", "Medida", "Zmierzony")), cell(r, 3)));
        }
        grid(cells, 6)
    }

    fn room_page(&self, r: usize) -> DialogDef {
        let n = self.survey.rooms[r].len();
        let off = self.offset(r);
        let need = n.saturating_sub(3);
        let name = self.room_name(r);
        let name = name[..1].to_uppercase() + &name[1..];
        let (first, last) = (self.survey.corner(r, 0), self.survey.corner(r, n - 1));
        let mut def = DialogDef::new(tr!(format!("{name} — lati, diagonali, vincoli"), format!("{name} — sides, diagonals, constraints"), format!("{name} — lados, diagonales, restricciones"), format!("{name} — côtés, diagonales, contraintes"), format!("{name} — kenarlar, köşegenler, kısıtlar"), format!("{name} — lados, diagonais, restrições"), format!("{name} — boki, przekątne, warunki")))
            .with_control(info(tr!(
                format!("LATI — spigoli {first} … {last}. Scrivi la misura sotto ogni lato (tra parentesi la lunghezza disegnata)."),
                format!("SIDES — corners {first} … {last}. Type the measure under each side (the drawn length in brackets)."),
                format!("LADOS — esquinas {first} … {last}. Escriba la medida bajo cada lado (entre paréntesis la longitud dibujada)."),
                format!("CÔTÉS — angles {first} … {last}. Saisissez la mesure sous chaque côté (entre parenthèses la longueur dessinée)."),
                format!("KENARLAR — köşeler {first} … {last}. Her kenarın altına ölçüyü girin (parantez içinde çizilen uzunluk)."),
                format!("LADOS — cantos {first} … {last}. Escreva a medida sob cada lado (entre parênteses o comprimento desenhado)."),
                format!("BOKI — narożniki {first} … {last}. Wpisz pomiar pod każdym bokiem (w nawiasie narysowana długość)."),
            )));
        let sides: Vec<DialogControl> = (0..n)
            .map(|i| {
                let row = self.saved.sides.get(off + i).cloned().unwrap_or_default();
                tf(format!("s{i}"), format!("{}  ({})", cell(&row, 0), cell(&row, 1)), cell(&row, 2))
            })
            .collect();
        for row in grid(sides, 4) {
            def = def.with_control(row);
        }
        let (others, cons) = self.room_view(r);
        def = def.with_control(info(if need == 0 {
            tr!(
                "DIAGONALI — un triangolo non ne ha bisogno.".to_string(),
                "DIAGONALS — a triangle needs none.".to_string(),
                "DIAGONALES — un triángulo no las necesita.".to_string(),
                "DIAGONALES — un triangle n'en a pas besoin.".to_string(),
                "KÖŞEGENLER — üçgenin köşegene ihtiyacı yoktur.".to_string(),
                "DIAGONAIS — um triângulo não precisa delas.".to_string(),
                "PRZEKĄTNE — trójkąt ich nie potrzebuje.".to_string(),
            )
        } else {
            tr!(
                format!(
                    "DIAGONALI — ne servono almeno {need} (contando anche i vincoli); una in più fa da controllo. Quelle \
                     proposte passano dentro il vano; nelle righe vuote puoi scriverne altre{}.",
                    if self.multi() { " (basta la lettera: A, C)" } else { "" }
                ),
                format!(
                    "DIAGONALS — at least {need} are needed (constraints count too); one more is a check. The proposed \
                     ones run inside the room; use the empty rows for more{}.",
                    if self.multi() { " (the letter is enough: A, C)" } else { "" }
                ),
                format!(
                    "DIAGONALES — se necesitan al menos {need} (las restricciones también cuentan); una más sirve de control. Las propuestas pasan por dentro de la estancia; use las filas vacías para otras{}.",
                    if self.multi() { " (the letter is enough: A, C)" } else { "" }
                ),
                format!(
                    "DIAGONALES — il en faut au moins {need} (les contraintes comptent aussi) ; une de plus sert de contrôle. Celles proposées passent à l'intérieur de la pièce ; utilisez les lignes vides pour d'autres{}.",
                    if self.multi() { " (the letter is enough: A, C)" } else { "" }
                ),
                format!(
                    "KÖŞEGENLER — en az {need} gerekli (kısıtlar da sayılır); bir fazlası kontrol işi görür. Önerilenler odanın içinden geçer; başkaları için boş satırları kullanın{}.",
                    if self.multi() { " (the letter is enough: A, C)" } else { "" }
                ),
                format!(
                    "DIAGONAIS — são precisas pelo menos {need} (as restrições também contam); uma a mais serve de controlo. As propostas passam por dentro do compartimento; use as linhas vazias para outras{}.",
                    if self.multi() { " (the letter is enough: A, C)" } else { "" }
                ),
                format!(
                    "PRZEKĄTNE — potrzeba co najmniej {need} (warunki też się liczą); jedna więcej służy jako kontrola. Proponowane przechodzą wewnątrz pomieszczenia; użyj pustych wierszy dla innych{}.",
                    if self.multi() { " (the letter is enough: A, C)" } else { "" }
                ),
            )
        }));
        for row in Self::distance_rows(&others) {
            def = def.with_control(row);
        }
        def = def.with_control(info(
            tr!(
                "VINCOLI (facoltativi) — uno spigolo e il suo angolo (A, 90), oppure un lato e orizzontale / verticale (A-B, orizzontale).",
                "CONSTRAINTS (optional) — a corner and its angle (A, 90), or a side and horizontal / vertical (A-B, horizontal).",
                "RESTRICCIONES (opcionales) — una esquina y su ángulo (A, 90), o un lado y horizontal / vertical (A-B, horizontal).",
                "CONTRAINTES (facultatives) — un angle et sa valeur (A, 90), ou un côté et horizontal / vertical (A-B, horizontal).",
                "KISITLAR (isteğe bağlı) — bir köşe ve açısı (A, 90) veya bir kenar ve yatay / dikey (A-B, yatay).",
                "RESTRIÇÕES (opcionais) — um canto e o seu ângulo (A, 90), ou um lado e horizontal / vertical (A-B, horizontal).",
                "WARUNKI (opcjonalne) — narożnik i jego kąt (A, 90) albo bok i poziomy / pionowy (A-B, poziomy).",
            )
            .to_string(),
        ));
        let mut cells = Vec::new();
        for (k, c) in cons.iter().enumerate() {
            cells.push(tf(format!("c{k}a"), tr!("Spigolo o lato", "Corner or side", "Esquina o lado", "Angle ou côté", "Köşe veya kenar", "Canto ou lado", "Narożnik lub bok").into(), cell(c, 0)));
            cells.push(tf(format!("c{k}b"), tr!("Vincolo", "Constraint", "Restricción", "Contrainte", "Kısıt", "Restrição", "Warunek").into(), cell(c, 1)));
        }
        for row in grid(cells, 6) {
            def = def.with_control(row);
        }
        def.submit_label("OK ▸").cancel_label(tr!("◂ Indietro", "◂ Back", "◂ Atrás", "◂ Retour", "◂ Geri", "◂ Voltar", "◂ Wstecz")).width(700.0)
    }

    fn links_page(&self) -> DialogDef {
        let mut def = DialogDef::new(tr!("Altre misure tra vani", "Other measures between rooms", "Otras medidas entre estancias", "Autres mesures entre pièces", "Odalar arası diğer ölçüler", "Outras medidas entre compartimentos", "Inne pomiary między pomieszczeniami")).with_control(info(
            tr!(
                "Servono solo dove non c'è una porta, o come controllo. Da e A: due spigoli di vani diversi (1B, 2D) per \
                 una distanza presa attraverso un'apertura, oppure un lato e uno spigolo (1B-1C, 2A) per lo spessore di un muro.",
                "Needed only where there is no door, or as a check. From and To: two corners of different rooms (1B, 2D) \
                 for a distance taken through an opening, or a side and a corner (1B-1C, 2A) for a wall thickness.",
                "Solo hacen falta donde no hay puerta, o como control. Desde y Hasta: dos esquinas de estancias distintas (1B, 2D) para una distancia tomada a través de una abertura, o un lado y una esquina (1B-1C, 2A) para un espesor de muro.",
                "Nécessaires seulement là où il n'y a pas de porte, ou comme contrôle. De et À : deux angles de pièces différentes (1B, 2D) pour une distance prise à travers une ouverture, ou un côté et un angle (1B-1C, 2A) pour une épaisseur de mur.",
                "Yalnızca kapı olmayan yerlerde veya kontrol olarak gerekir. Başlangıç ve Bitiş: bir açıklıktan geçen uzaklık için farklı odaların iki köşesi (1B, 2D) veya duvar kalınlığı için bir kenar ve bir köşe (1B-1C, 2A).",
                "Só são precisas onde não há porta, ou como controlo. De e Até: dois cantos de compartimentos diferentes (1B, 2D) para uma distância tomada através de uma abertura, ou um lado e um canto (1B-1C, 2A) para uma espessura de parede.",
                "Potrzebne tylko tam, gdzie nie ma drzwi, lub jako kontrola. Od i Do: dwa narożniki różnych pomieszczeń (1B, 2D) dla odległości mierzonej przez otwór albo bok i narożnik (1B-1C, 2A) dla grubości ściany.",
            )
            .to_string(),
        ));
        for row in Self::distance_rows(&self.links_view()) {
            def = def.with_control(row);
        }
        def.submit_label("OK ▸").cancel_label(tr!("◂ Indietro", "◂ Back", "◂ Atrás", "◂ Retour", "◂ Geri", "◂ Voltar", "◂ Wstecz")).width(700.0)
    }

    fn openings_page(&self) -> DialogDef {
        let mut def = DialogDef::new(tr!("Porte e finestre inserite", "Doors and windows", "Puertas y ventanas", "Portes et fenêtres", "Kapılar ve pencereler", "Portas e janelas", "Drzwi i okna")).with_control(info(
            tr!(
                "Correggi i valori, oppure svuota una riga per eliminare la porta o la finestra. Una porta tra due vani \
                 occupa due righe con lo stesso nome, una per vano. La distanza si misura dal primo spigolo scritto nel lato.",
                "Fix the values, or empty a row to delete the door or window. A door between two rooms takes two rows \
                 with the same name, one per room. The distance is taken from the first corner written in the side.",
                "Corrija los valores, o vacíe una fila para borrar la puerta o ventana. Una puerta entre dos estancias ocupa dos filas con el mismo nombre, una por estancia. La distancia se toma desde la primera esquina escrita en el lado.",
                "Corrigez les valeurs, ou videz une ligne pour supprimer la porte ou la fenêtre. Une porte entre deux pièces occupe deux lignes portant le même nom, une par pièce. La distance se prend depuis le premier angle écrit dans le côté.",
                "Değerleri düzeltin veya kapıyı ya da pencereyi silmek için bir satırı boşaltın. İki oda arasındaki kapı aynı adı taşıyan iki satır kaplar, her oda için bir tane. Uzaklık, kenarda yazılan ilk köşeden alınır.",
                "Corrija os valores, ou esvazie uma linha para apagar a porta ou janela. Uma porta entre dois compartimentos ocupa duas linhas com o mesmo nome, uma por compartimento. A distância toma-se a partir do primeiro canto escrito no lado.",
                "Popraw wartości albo opróżnij wiersz, aby usunąć drzwi lub okno. Drzwi między dwoma pomieszczeniami zajmują dwa wiersze o tej samej nazwie, po jednym na pomieszczenie. Odległość mierzy się od pierwszego narożnika wpisanego w boku.",
            )
            .to_string(),
        ));
        let labels = [
            tr!("Nome", "Name", "Nombre", "Nom", "Ad", "Nome", "Nazwa"),
            tr!("Lato", "Side", "Lado", "Côté", "Kenar", "Lado", "Bok"),
            tr!("Tipo", "Kind", "Tipo", "Type", "Tür", "Tipo", "Typ"),
            tr!("Distanza", "Distance", "Distancia", "Distance", "Uzaklık", "Distância", "Odległość"),
            tr!("Larghezza", "Width", "Anchura", "Largeur", "Genişlik", "Largura", "Szerokość"),
            tr!("Spessore muro", "Wall", "Muro", "Mur", "Duvar", "Parede", "Ściana"),
        ];
        let mut cells = Vec::new();
        for (k, r) in self.openings_view().iter().enumerate() {
            for (c, l) in labels.iter().enumerate() {
                cells.push(tf(format!("p{k}_{c}"), l.to_string(), cell(r, c)));
            }
        }
        for row in grid(cells, 6) {
            def = def.with_control(row);
        }
        def.submit_label("OK ▸").cancel_label(tr!("◂ Indietro", "◂ Back", "◂ Atrás", "◂ Retour", "◂ Geri", "◂ Voltar", "◂ Wstecz")).width(700.0)
    }

    fn options_page(&self) -> DialogDef {
        let saved = &self.saved;
        DialogDef::new(tr!("Opzioni del rilievo", "Survey options", "Opciones del levantamiento", "Options du relevé", "Ölçüm seçenekleri", "Opções do levantamento", "Opcje pomiaru"))
            .with_control(DialogControl::row(vec![
                number("h", tr!("Altezza testi", "Text height", "Altura de texto", "Hauteur de texte", "Yazı yüksekliği", "Altura do texto", "Wysokość tekstu"), saved.h),
                number("tol", tr!("Tolleranza scarti", "Residual tolerance", "Tolerancia de residuos", "Tolérance des résidus", "Artık toleransı", "Tolerância dos resíduos", "Tolerancja odchyłek"), saved.tol),
                DialogControl::Choice {
                    id: "unit".into(),
                    label: tr!("Unità del disegno", "Drawing unit", "Unidad del dibujo", "Unité du dessin", "Çizim birimi", "Unidade do desenho", "Jednostka rysunku").into(),
                    options: vec![tr!("metri", "metres", "metros", "mètres", "metre", "metros", "metry").into(), tr!("centimetri", "centimetres", "centímetros", "centimètres", "santimetre", "centímetros", "centymetry").into(), tr!("millimetri", "millimetres", "milímetros", "millimètres", "milimetre", "milímetros", "milimetry").into()],
                    default_index: saved.unit.min(2),
                },
            ]))
            .with_control(checkbox(
                "fill",
                tr!(
                    "Se mancano misure (mobili, ostacoli), completa la forma con gli angoli del disegno",
                    "If measures are missing (furniture, obstacles), finish the shape with the drawing's angles",
                    "Si faltan medidas (muebles, obstáculos), completar la forma con los ángulos del dibujo",
                    "S'il manque des mesures (meubles, obstacles), compléter la forme avec les angles du dessin",
                    "Ölçü eksikse (mobilya, engeller), şekli çizimin açılarıyla tamamla",
                    "Se faltarem medidas (móveis, obstáculos), completar a forma com os ângulos do desenho",
                    "Jeśli brakuje pomiarów (meble, przeszkody), uzupełnij kształt kątami z rysunku",
                ),
                saved.fill,
            ))
            .with_control(checkbox(
                "replace",
                tr!(
                    "Modifica subito le polilinee originali (senza copia su RILIEVO_CORRETTO); altrimenti usa poi Applica al disegno",
                    "Change the original polylines straight away (no copy on RILIEVO_CORRETTO); otherwise use Apply to drawing later",
                    "Modificar enseguida las polilíneas originales (sin copia en RILIEVO_CORRETTO); si no, use después Aplicar al dibujo",
                    "Modifier tout de suite les polylignes d'origine (sans copie sur RILIEVO_CORRETTO) ; sinon utilisez ensuite Appliquer au dessin",
                    "Özgün çoklu çizgileri hemen değiştir (RILIEVO_CORRETTO üzerinde kopya olmadan); aksi halde sonra Çizime uygula kullanın",
                    "Alterar logo as polilinhas originais (sem cópia em RILIEVO_CORRETTO); caso contrário use depois Aplicar ao desenho",
                    "Od razu zmień oryginalne polilinie (bez kopii na RILIEVO_CORRETTO); w przeciwnym razie użyj później Zastosuj na rysunku",
                ),
                saved.replace,
            ))
            .with_control(DialogControl::row(vec![
                checkbox("dims", tr!("Quote", "Dimensions", "Cotas", "Cotes", "Ölçülendirmeler", "Cotas", "Wymiary"), saved.dims),
                checkbox("table", tr!("Tabella scarti", "Residuals table", "Tabla de residuos", "Tableau des résidus", "Artık tablosu", "Tabela de resíduos", "Tabela odchyłek"), saved.table),
                checkbox("csv", tr!("Salva libretto (CSV)", "Save book (CSV)", "Guardar libro (CSV)", "Enregistrer le carnet (CSV)", "Defteri kaydet (CSV)", "Guardar caderno (CSV)", "Zapisz dziennik (CSV)"), saved.csv),
            ]))
            .with_control(csv_picker(&saved.csv_path))
            .submit_label("OK ▸")
            .cancel_label(tr!("◂ Indietro", "◂ Back", "◂ Atrás", "◂ Retour", "◂ Geri", "◂ Voltar", "◂ Wstecz"))
            .width(660.0)
    }

    /// The outer-wall rows shown: the ones kept, then three spare ones.
    fn outer_view(&self) -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = self.saved.outer_walls.iter().filter(|r| filled(r, &[0, 1])).cloned().collect();
        let total = (rows.len() + 3).div_ceil(3) * 3;
        rows.resize(total, vec![String::new(); 2]);
        rows
    }

    fn outer_page(&self) -> DialogDef {
        let mut def = DialogDef::new(tr!("Muri esterni", "Outer walls", "Muros exteriores", "Murs extérieurs", "Dış duvarlar", "Paredes exteriores", "Ściany zewnętrzne")).with_control(info(
            tr!(
                "Il poligono esterno, rilevato con la stazione totale, resta fermo. I lati dei vani scritti qui sotto restano \
                 alla distanza dello spessore del muro dal lato del poligono esterno più vicino e parallelo: così i vani si \
                 posizionano da soli dentro l'edificio. Il plugin ha proposto i lati vicini al poligono esterno: controllali, \
                 svuota quelli sbagliati, aggiungi quelli mancanti. Uno spessore vuoto vale lo spessore dei muri esterni.",
                "The outer outline, surveyed with a total station, stays still. The room sides written below stay at the \
                 wall's thickness from the nearest parallel side of the outline: this way the rooms place themselves inside \
                 the building. The plugin proposed the sides near the outline: check them, empty the wrong ones, add the \
                 missing ones. An empty thickness means the thickness of the outer walls.",
                "El polígono exterior, levantado con la estación total, queda fijo. Los lados de estancias escritos abajo se mantienen a la distancia del espesor del muro del lado del polígono exterior más cercano y paralelo: así las estancias se colocan solas dentro del edificio. El plugin ha propuesto los lados cercanos al polígono exterior: compruébelos, vacíe los equivocados, añada los que falten. Un espesor vacío vale el espesor de los muros exteriores.",
                "Le polygone extérieur, relevé à la station totale, reste fixe. Les côtés de pièces écrits ci-dessous restent à l'épaisseur du mur du côté du polygone extérieur le plus proche et parallèle : ainsi les pièces se placent d'elles-mêmes dans le bâtiment. Le plugin a proposé les côtés proches du polygone extérieur : vérifiez-les, videz ceux qui sont faux, ajoutez ceux qui manquent. Une épaisseur vide vaut l'épaisseur des murs extérieurs.",
                "Total station ile ölçülen dış poligon sabit kalır. Aşağıda yazılı oda kenarları, dış poligonun en yakın ve paralel kenarından duvar kalınlığı kadar uzakta kalır: böylece odalar binanın içine kendiliğinden yerleşir. Eklenti dış poligona yakın kenarları önerdi: kontrol edin, yanlış olanları boşaltın, eksik olanları ekleyin. Boş bir kalınlık, dış duvarların kalınlığı demektir.",
                "O polígono exterior, levantado com a estação total, fica fixo. Os lados de compartimentos escritos abaixo ficam à distância da espessura da parede do lado do polígono exterior mais próximo e paralelo: assim os compartimentos colocam-se sozinhos dentro do edifício. O plugin propôs os lados próximos do polígono exterior: verifique-os, esvazie os errados, acrescente os que faltam. Uma espessura vazia vale a espessura das paredes exteriores.",
                "Obrys zewnętrzny zmierzony tachimetrem pozostaje nieruchomy. Wpisane niżej boki pomieszczeń pozostają w odległości grubości ściany od najbliższego równoległego boku obrysu: dzięki temu pomieszczenia same ustawiają się wewnątrz budynku. Wtyczka zaproponowała boki bliskie obrysu: sprawdź je, opróżnij błędne, dodaj brakujące. Pusta grubość oznacza grubość ścian zewnętrznych.",
            )
            .to_string(),
        ));
        def = def.with_control(tf(
            "outer_t".into(),
            tr!("Spessore dei muri esterni", "Thickness of the outer walls", "Espesor de los muros exteriores", "Épaisseur des murs extérieurs", "Dış duvarların kalınlığı", "Espessura das paredes exteriores", "Grubość ścian zewnętrznych").into(),
            self.saved.outer_t.clone(),
        ));
        let mut cells = Vec::new();
        for (k, r) in self.outer_view().iter().enumerate() {
            cells.push(tf(format!("o{k}_0"), tr!("Lato del vano", "Room side", "Lado de la estancia", "Côté de la pièce", "Oda kenarı", "Lado do compartimento", "Bok pomieszczenia").into(), cell(r, 0)));
            cells.push(tf(format!("o{k}_1"), tr!("Spessore", "Thickness", "Espesor", "Épaisseur", "Kalınlık", "Espessura", "Grubość").into(), cell(r, 1)));
        }
        for row in grid(cells, 6) {
            def = def.with_control(row);
        }
        def.with_control(checkbox(
            "redo",
            tr!(
                "Proponi di nuovo i lati vicini al poligono esterno",
                "Propose the sides near the outer outline again",
                "Proponer de nuevo los lados cercanos al polígono exterior",
                "Proposer de nouveau les côtés proches du polygone extérieur",
                "Dış poligona yakın kenarları yeniden öner",
                "Propor de novo os lados próximos do polígono exterior",
                "Zaproponuj ponownie boki bliskie obrysu zewnętrznego",
            ),
            false,
        ))
        .with_control(checkbox(
            "drop",
            tr!(
                "Non usare più il poligono esterno (per sceglierne un altro)",
                "Stop using the outer outline (to choose another one)",
                "Dejar de usar el polígono exterior (para elegir otro)",
                "Ne plus utiliser le polygone extérieur (pour en choisir un autre)",
                "Dış poligonu artık kullanma (başka birini seçmek için)",
                "Deixar de usar o polígono exterior (para escolher outro)",
                "Nie używaj już obrysu zewnętrznego (aby wybrać inny)",
            ),
            false,
        ))
        .submit_label("OK ▸")
        .cancel_label(tr!("◂ Indietro", "◂ Back", "◂ Atrás", "◂ Retour", "◂ Geri", "◂ Voltar", "◂ Wstecz"))
        .width(700.0)
    }

    fn read_outer(&mut self, m: &HashMap<String, DialogValue>) {
        if get_bool(m, "drop", false) {
            self.saved.outer.clear();
            self.saved.outer_walls.clear();
            self.survey = Survey::new(self.survey.rooms.clone());
            return;
        }
        if m.contains_key("outer_t") {
            self.saved.outer_t = get_text(m, "outer_t");
        }
        if get_bool(m, "redo", false) {
            let t = commands::parse_num(&self.saved.outer_t).filter(|v| *v > 0.0).unwrap_or(0.30 / survey::unit_factor(self.saved.unit));
            let o = self.survey.outline();
            let longest = (0..o.len()).map(|i| solve::dist(o[i], o[(i + 1) % o.len()])).fold(0.0f64, f64::max);
            self.saved.outer_walls = self.survey.suggest_outer_walls((4.0 * t).max(longest * 0.05));
            return;
        }
        let mut rows = Vec::new();
        for (k, old) in self.outer_view().iter().enumerate() {
            let row: Vec<String> = (0..2).map(|c| if m.contains_key(&format!("o{k}_{c}")) { get_text(m, &format!("o{k}_{c}")) } else { cell(old, c) }).collect();
            if !row[0].is_empty() {
                // In a single room a bare "A-B" is enough; with several, "1A-1B".
                rows.push(row);
            }
        }
        self.saved.outer_walls = rows;
    }

    /// A name not used yet: P1, P2 … (F1 … for windows).
    fn free_name(&self, prefix: &str) -> String {
        (1..)
            .map(|k| format!("{prefix}{k}"))
            .find(|n| !self.saved.openings.iter().any(|r| cell(r, 0).eq_ignore_ascii_case(n)))
            .unwrap_or_default()
    }

    fn door_page(&self, spot: &DoorSpot) -> DialogDef {
        let s = &self.survey;
        let d = |id: &str| get_text(&self.draft, id);
        let (c1, c2) = (s.corner(spot.room, spot.from), s.corner(spot.room, spot.to));
        let mut def = DialogDef::new(match spot.other {
            Some((q, _, _)) if self.multi() => tr!(
                format!("Porta tra il vano {} e il vano {}", spot.room + 1, q + 1),
                format!("Door between room {} and room {}", spot.room + 1, q + 1),
                format!("Puerta entre la estancia {} y la estancia {}", spot.room + 1, q + 1),
                format!("Porte entre la pièce {} et la pièce {}", spot.room + 1, q + 1),
                format!("{}. oda ile {}. oda arasındaki kapı", spot.room + 1, q + 1),
                format!("Porta entre o compartimento {} e o compartimento {}", spot.room + 1, q + 1),
                format!("Drzwi między pomieszczeniem {} a pomieszczeniem {}", spot.room + 1, q + 1),
            ),
            _ => tr!(format!("Porta o finestra sul lato {c1}–{c2}"), format!("Door or window on side {c1}–{c2}"), format!("Puerta o ventana en el lado {c1}–{c2}"), format!("Porte ou fenêtre sur le côté {c1}–{c2}"), format!("{c1}–{c2} kenarında kapı veya pencere"), format!("Porta ou janela no lado {c1}–{c2}"), format!("Drzwi lub okno na boku {c1}–{c2}")),
        });
        if let Some(m) = &self.msg {
            def = def.with_control(info(format!("⚠ {m}.")));
        }
        def = def.with_control(info(match spot.other {
            Some((q, k, l)) => tr!(
                format!(
                    "Muro tra il lato {c1}–{c2} (vano {}) e il lato {}–{} (vano {}). Con lo spessore del muro e le due distanze \
                     dagli spigoli, questa porta collega i due vani.",
                    spot.room + 1,
                    s.corner(q, k),
                    s.corner(q, l),
                    q + 1
                ),
                format!(
                    "Wall between side {c1}–{c2} (room {}) and side {}–{} (room {}). With the wall thickness and the two \
                     distances from the corners, this door links the two rooms.",
                    spot.room + 1,
                    s.corner(q, k),
                    s.corner(q, l),
                    q + 1
                ),
                format!(
                    "Muro entre el lado {c1}–{c2} (estancia {}) y el lado {}–{} (estancia {}). Con el espesor del muro y las dos distancias desde las esquinas, esta puerta enlaza las dos estancias.",
                    spot.room + 1,
                    s.corner(q, k),
                    s.corner(q, l),
                    q + 1
                ),
                format!(
                    "Mur entre le côté {c1}–{c2} (pièce {}) et le côté {}–{} (pièce {}). Avec l'épaisseur du mur et les deux distances depuis les angles, cette porte lie les deux pièces.",
                    spot.room + 1,
                    s.corner(q, k),
                    s.corner(q, l),
                    q + 1
                ),
                format!(
                    "{c1}–{c2} kenarı ({}. oda) ile {}–{} kenarı ({}. oda) arasındaki duvar. Duvar kalınlığı ve köşelerden iki uzaklıkla bu kapı iki odayı bağlar.",
                    spot.room + 1,
                    s.corner(q, k),
                    s.corner(q, l),
                    q + 1
                ),
                format!(
                    "Parede entre o lado {c1}–{c2} (compartimento {}) e o lado {}–{} (compartimento {}). Com a espessura da parede e as duas distâncias a partir dos cantos, esta porta liga os dois compartimentos.",
                    spot.room + 1,
                    s.corner(q, k),
                    s.corner(q, l),
                    q + 1
                ),
                format!(
                    "Ściana między bokiem {c1}–{c2} (pomieszczenie {}) a bokiem {}–{} (pomieszczenie {}). Z grubością ściany i dwiema odległościami od narożników te drzwi wiążą oba pomieszczenia.",
                    spot.room + 1,
                    s.corner(q, k),
                    s.corner(q, l),
                    q + 1
                ),
            ),
            None => tr!(
                format!("Lato {c1}–{c2}: dietro questo muro non c'è un altro vano scelto (finestra o porta verso l'esterno)."),
                format!("Side {c1}–{c2}: no other chosen room behind this wall (a window or an outside door)."),
                format!("Lado {c1}–{c2}: no hay otra estancia elegida detrás de este muro (una ventana o una puerta al exterior)."),
                format!("Côté {c1}–{c2} : aucune autre pièce choisie derrière ce mur (une fenêtre ou une porte vers l'extérieur)."),
                format!("{c1}–{c2} kenarı: bu duvarın arkasında seçilmiş başka oda yok (bir pencere veya dışarıya açılan kapı)."),
                format!("Lado {c1}–{c2}: não há outro compartimento escolhido atrás desta parede (uma janela ou uma porta para o exterior)."),
                format!("Bok {c1}–{c2}: za tą ścianą nie ma innego wybranego pomieszczenia (okno lub drzwi na zewnątrz)."),
            ),
        }));
        let kind_default = if spot.other.is_some() { 0 } else { 1 };
        let name_default = if d("name").is_empty() { self.free_name(if spot.other.is_some() { "P" } else { "F" }) } else { d("name") };
        def = def
            .with_control(DialogControl::row(vec![
                tf("name".into(), tr!("Nome", "Name", "Nombre", "Nom", "Ad", "Nome", "Nazwa").into(), name_default),
                DialogControl::Choice {
                    id: "kind".into(),
                    label: tr!("Tipo", "Kind", "Tipo", "Type", "Tür", "Tipo", "Typ").into(),
                    options: vec![tr!("porta", "door", "puerta", "porte", "kapı", "porta", "drzwi").into(), tr!("finestra", "window", "ventana", "fenêtre", "pencere", "janela", "okno").into(), tr!("apertura senza serramento", "plain opening", "abertura sin carpintería", "ouverture sans menuiserie", "doğramasız açıklık", "abertura sem caixilharia", "otwór bez stolarki").into()],
                    default_index: get_choice(&self.draft, "kind", kind_default),
                },
            ]))
            .with_control(DialogControl::row(vec![
                DialogControl::Choice {
                    id: "from1".into(),
                    label: tr!("Distanza misurata dallo spigolo", "Distance measured from corner", "Distancia medida desde la esquina", "Distance mesurée depuis l'angle", "Köşeden ölçülen uzaklık", "Distância medida a partir do canto", "Odległość mierzona od narożnika").into(),
                    options: vec![c1.clone(), c2.clone()],
                    default_index: get_choice(&self.draft, "from1", 0),
                },
                tf("d1".into(), tr!("Distanza dallo spigolo allo stipite", "Distance from corner to jamb", "Distancia de la esquina a la jamba", "Distance de l'angle au tableau", "Köşeden kasaya uzaklık", "Distância do canto à ombreira", "Odległość od narożnika do ościeża").into(), d("d1")),
                tf("w".into(), tr!("Larghezza", "Width", "Anchura", "Largeur", "Genişlik", "Largura", "Szerokość").into(), d("w")),
            ]))
            .with_control(tf(
                "t".into(),
                tr!("Spessore del muro, misurato nella porta", "Wall thickness, measured in the door", "Espesor del muro, medido en la puerta", "Épaisseur du mur, mesurée dans la porte", "Duvar kalınlığı, kapıda ölçülen", "Espessura da parede, medida na porta", "Grubość ściany, zmierzona w drzwiach").into(),
                d("t"),
            ));
        if let Some((q, k, l)) = spot.other {
            def = def
                .with_control(DialogControl::row(vec![
                    DialogControl::Choice {
                        id: "from2".into(),
                        label: tr!(format!("Nel vano {}: distanza dallo spigolo", q + 1), format!("In room {}: distance from corner", q + 1), format!("En la estancia {}: distancia desde la esquina", q + 1), format!("Dans la pièce {} : distance depuis l'angle", q + 1), format!("{}. odada: köşeden uzaklık", q + 1), format!("No compartimento {}: distância a partir do canto", q + 1), format!("W pomieszczeniu {}: odległość od narożnika", q + 1)),
                        options: vec![s.corner(q, k), s.corner(q, l)],
                        default_index: get_choice(&self.draft, "from2", 0),
                    },
                    tf(
                        "d2".into(),
                        tr!(format!("Distanza allo stipite, misurata nel vano {}", q + 1), format!("Distance to the jamb, measured in room {}", q + 1), format!("Distancia a la jamba, medida en la estancia {}", q + 1), format!("Distance au tableau, mesurée dans la pièce {}", q + 1), format!("Kasaya uzaklık, {}. odada ölçülen", q + 1), format!("Distância à ombreira, medida no compartimento {}", q + 1), format!("Odległość do ościeża, zmierzona w pomieszczeniu {}", q + 1)),
                        d("d2"),
                    ),
                ]))
                .with_control(info(
                    tr!(
                        "Se lasci vuota la seconda distanza, i vani restano legati dal solo spessore del muro e la loro posizione \
                         lungo il muro viene presa dal disegno.",
                        "If the second distance is left empty, the rooms are tied by the wall thickness only and their position \
                         along the wall is taken from the drawing.",
                        "Si la segunda distancia se deja vacía, las estancias quedan unidas solo por el espesor del muro y su posición a lo largo del muro se toma del dibujo.",
                        "Si la seconde distance est laissée vide, les pièces sont liées par la seule épaisseur du mur et leur position le long du mur est prise du dessin.",
                        "İkinci uzaklık boş bırakılırsa odalar yalnızca duvar kalınlığıyla bağlanır ve duvar boyunca konumları çizimden alınır.",
                        "Se a segunda distância ficar vazia, os compartimentos ficam ligados só pela espessura da parede e a sua posição ao longo da parede é tirada do desenho.",
                        "Jeśli druga odległość pozostanie pusta, pomieszczenia są powiązane tylko grubością ściany, a ich położenie wzdłuż ściany jest brane z rysunku.",
                    )
                    .to_string(),
                ));
        }
        def.submit_label(tr!("Aggiungi ▸", "Add ▸", "Añadir ▸", "Ajouter ▸", "Ekle ▸", "Acrescentar ▸", "Dodaj ▸")).cancel_label(tr!("◂ Indietro", "◂ Back", "◂ Atrás", "◂ Retour", "◂ Geri", "◂ Voltar", "◂ Wstecz")).width(700.0)
    }

    fn read_room(&mut self, r: usize, m: &HashMap<String, DialogValue>) {
        let off = self.offset(r);
        for i in 0..self.survey.rooms[r].len() {
            let id = format!("s{i}");
            if let (true, Some(row)) = (m.contains_key(&id), self.saved.sides.get_mut(off + i)) {
                row.resize(3, String::new());
                row[2] = get_text(m, &id);
            }
        }
        let (others, cons) = self.room_view(r);
        let rows = self.read_distances(Some(r), &others, m);
        let (mut per, links) = self.split_others();
        per[r] = rows;
        self.saved.others = per.into_iter().flatten().chain(links).collect();

        let mut mine = Vec::new();
        for (k, old) in cons.iter().enumerate() {
            let (a, b) = if m.contains_key(&format!("c{k}a")) {
                (self.norm(r, &get_text(m, &format!("c{k}a"))), get_text(m, &format!("c{k}b")))
            } else {
                (cell(old, 0), cell(old, 1))
            };
            if !(a.is_empty() && b.is_empty()) {
                mine.push(vec![a, b]);
            }
        }
        let mut per = self.split_cons();
        per[r] = mine;
        self.saved.constraints = per.into_iter().flatten().collect();
    }

    /// The Da / A / Misurata rows of a page, with the drawn value refreshed.
    fn read_distances(&self, room: Option<usize>, shown: &[Vec<String>], m: &HashMap<String, DialogValue>) -> Vec<Vec<String>> {
        let dec = commands::decimals(self.saved.tol);
        let mut rows = Vec::new();
        for (k, old) in shown.iter().enumerate() {
            let (a, b, v) = if m.contains_key(&format!("o{k}a")) {
                let (a, b) = (get_text(m, &format!("o{k}a")), get_text(m, &format!("o{k}b")));
                match room {
                    Some(r) => (self.norm(r, &a), self.norm(r, &b), get_text(m, &format!("o{k}m"))),
                    None => (a.to_ascii_uppercase(), b.to_ascii_uppercase(), get_text(m, &format!("o{k}m"))),
                }
            } else {
                (cell(old, 0), cell(old, 1), cell(old, 3))
            };
            if a.is_empty() && b.is_empty() && v.is_empty() {
                continue;
            }
            let drawn = self.survey.drawn_other(&a, &b).map(|d| commands::fmt_num(d, dec)).unwrap_or_default();
            rows.push(vec![a, b, drawn, v]);
        }
        rows
    }

    fn read_links(&mut self, m: &HashMap<String, DialogValue>) {
        let rows = self.read_distances(None, &self.links_view(), m);
        let (per, _) = self.split_others();
        self.saved.others = per.into_iter().flatten().chain(rows).collect();
    }

    fn read_openings(&mut self, m: &HashMap<String, DialogValue>) {
        let mut ops = Vec::new();
        for (k, old) in self.openings_view().iter().enumerate() {
            let row: Vec<String> =
                (0..6).map(|c| if m.contains_key(&format!("p{k}_{c}")) { get_text(m, &format!("p{k}_{c}")) } else { cell(old, c) }).collect();
            if row.iter().any(|x| !x.is_empty()) {
                ops.push(row);
            }
        }
        self.saved.openings = ops;
    }

    fn read_options(&mut self, m: &HashMap<String, DialogValue>) {
        let s = &mut self.saved;
        if let Some(v) = get_num(m, "h").filter(|v| *v > 0.0) {
            s.h = v;
        }
        if let Some(v) = get_num(m, "tol").filter(|v| *v >= 0.0) {
            s.tol = v;
        }
        s.unit = get_choice(m, "unit", s.unit);
        s.dims = get_bool(m, "dims", s.dims);
        s.table = get_bool(m, "table", s.table);
        s.csv = get_bool(m, "csv", s.csv);
        s.fill = get_bool(m, "fill", s.fill);
        s.replace = get_bool(m, "replace", s.replace);
        if let Some(p) = get_path(m, "csv_path") {
            s.csv_path = p;
        }
    }

    /// Add the door or window typed on the door page; an error says what
    /// is missing and nothing is added.
    fn read_door(&mut self, spot: &DoorSpot, m: &HashMap<String, DialogValue>) -> Result<(), String> {
        let s = &self.survey;
        let num = |id: &str| commands::parse_num(&get_text(m, id));
        let name = match get_text(m, "name") {
            n if n.is_empty() => self.free_name("P"),
            n => n,
        };
        if self.saved.openings.iter().any(|r| cell(r, 0).eq_ignore_ascii_case(&name)) {
            return Err(tr!(format!("il nome {name} è già usato: scegline un altro"), format!("the name {name} is already used: choose another"), format!("el nombre {name} ya se usa: elija otro"), format!("le nom {name} est déjà utilisé : choisissez-en un autre"), format!("{name} adı zaten kullanılıyor: başka bir ad seçin"), format!("o nome {name} já está a ser usado: escolha outro"), format!("nazwa {name} jest już używana: wybierz inną")));
        }
        let (d1, w, t) = (get_text(m, "d1"), get_text(m, "w"), get_text(m, "t"));
        if !(num("d1").is_some_and(|v| v >= 0.0) && num("w").is_some_and(|v| v > 0.0)) {
            return Err(tr!(
                "scrivi la distanza dallo spigolo allo stipite e la larghezza".to_string(),
                "type the distance from the corner to the jamb and the width".to_string(),
                "escriba la distancia de la esquina a la jamba y la anchura".to_string(),
                "saisissez la distance de l'angle au tableau et la largeur".to_string(),
                "köşeden kasaya uzaklığı ve genişliği girin".to_string(),
                "escreva a distância do canto à ombreira e a largura".to_string(),
                "wpisz odległość od narożnika do ościeża i szerokość".to_string(),
            ));
        }
        if !t.is_empty() && !num("t").is_some_and(|v| v > 0.0) {
            return Err(tr!(format!("lo spessore \"{t}\" non è una misura valida"), format!("the thickness \"{t}\" is not a valid measure"), format!("el espesor \"{t}\" no es una medida válida"), format!("l'épaisseur \"{t}\" n'est pas une mesure valable"), format!("\"{t}\" kalınlığı geçerli bir ölçü değil"), format!("a espessura \"{t}\" não é uma medida válida"), format!("grubość \"{t}\" nie jest prawidłowym pomiarem")));
        }
        let kind = match get_choice(m, "kind", if spot.other.is_some() { 0 } else { 1 }) {
            0 => tr!("porta", "door", "puerta", "porte", "kapı", "porta", "drzwi"),
            1 => tr!("finestra", "window", "ventana", "fenêtre", "pencere", "janela", "okno"),
            _ => tr!("apertura", "opening", "abertura", "ouverture", "açıklık", "abertura", "otwór"),
        };
        let (i, j) = if get_choice(m, "from1", 0) == 0 { (spot.from, spot.to) } else { (spot.to, spot.from) };
        let side1 = format!("{}-{}", s.corner(spot.room, i), s.corner(spot.room, j));
        let mut second = None;
        let mut walls = Vec::new();
        if let Some((q, k, l)) = spot.other {
            let (k, l) = if get_choice(m, "from2", 0) == 0 { (k, l) } else { (l, k) };
            let d2 = get_text(m, "d2");
            if !d2.is_empty() {
                if !num("d2").is_some_and(|v| v >= 0.0) {
                    return Err(tr!(format!("la distanza \"{d2}\" non è una misura valida"), format!("the distance \"{d2}\" is not a valid measure"), format!("la distancia \"{d2}\" no es una medida válida"), format!("la distance \"{d2}\" n'est pas une mesure valable"), format!("\"{d2}\" uzaklığı geçerli bir ölçü değil"), format!("a distância \"{d2}\" não é uma medida válida"), format!("odległość \"{d2}\" nie jest prawidłowym pomiarem")));
                }
                if t.is_empty() {
                    return Err(tr!(
                        "per collegare i due vani scrivi lo spessore del muro, misurato nella porta".to_string(),
                        "to link the two rooms type the wall thickness, measured in the door".to_string(),
                        "para enlazar las dos estancias escriba el espesor del muro, medido en la puerta".to_string(),
                        "pour lier les deux pièces, saisissez l'épaisseur du mur, mesurée dans la porte".to_string(),
                        "iki odayı bağlamak için kapıda ölçülen duvar kalınlığını girin".to_string(),
                        "para ligar os dois compartimentos escreva a espessura da parede, medida na porta".to_string(),
                        "aby powiązać oba pomieszczenia, wpisz grubość ściany zmierzoną w drzwiach".to_string(),
                    ));
                }
                second = Some(vec![name.clone(), format!("{}-{}", s.corner(q, k), s.corner(q, l)), kind.to_string(), d2, w.clone(), String::new()]);
            } else if !t.is_empty() {
                let dec = commands::decimals(self.saved.tol);
                for c in [k, l] {
                    let b = s.corner(q, c);
                    if !self.saved.others.iter().any(|r| cell(r, 0).eq_ignore_ascii_case(&side1) && cell(r, 1).eq_ignore_ascii_case(&b)) {
                        let drawn = s.drawn_other(&side1, &b).map(|d| commands::fmt_num(d, dec)).unwrap_or_default();
                        walls.push(vec![side1.clone(), b, drawn, t.clone()]);
                    }
                }
            }
        }
        self.saved.openings.push(vec![name, side1, kind.to_string(), d1, w, t]);
        self.saved.openings.extend(second);
        self.saved.others.extend(walls);
        Ok(())
    }
}

impl InteractiveCommand for MeasureDialog {
    fn prompt(&self) -> String {
        tr!(
            "RV_VANO: scegli cosa fare nella finestra (apri \"Come si usa\" per l'aiuto)",
            "RV_VANO: choose what to do in the dialog (open \"How it works\" for help)",
            "RV_VANO: elija qué hacer en la ventana (abra \"Cómo funciona\" para la ayuda)",
            "RV_VANO : choisissez quoi faire dans la fenêtre (ouvrez \"Mode d'emploi\" pour l'aide)",
            "RV_VANO: pencerede ne yapılacağını seçin (yardım için \"Nasıl çalışır\" açın)",
            "RV_VANO: escolha o que fazer na janela (abra \"Como funciona\" para a ajuda)",
            "RV_VANO: wybierz w oknie, co zrobić (otwórz \"Jak to działa\", aby zobaczyć pomoc)",
        )
        .into()
    }
    fn on_point(&mut self, _pt: [f64; 3]) -> CommandStep {
        CommandStep::NeedPoint
    }
    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Cancel
    }
    fn modal_dialog(&self) -> Option<DialogDef> {
        match &self.page {
            Page::Hub => Some(self.hub()),
            Page::Room(r) => Some(self.room_page(*r)),
            Page::Links => Some(self.links_page()),
            Page::Openings => Some(self.openings_page()),
            Page::Options => Some(self.options_page()),
            Page::Door(spot) => Some(self.door_page(spot)),
            Page::Outer => Some(self.outer_page()),
            Page::Done => None,
        }
    }
    fn on_dialog_response(&mut self, response: DialogResponse) -> CommandStep {
        let page = self.page.clone();
        match (page, response) {
            (Page::Done, _) => CommandStep::Cancel,
            (Page::Hub, DialogResponse::Submitted(m)) => {
                let (acts, _, default) = self.actions();
                let act = acts.get(get_choice(&m, "do", default)).copied().unwrap_or(Action::Compute);
                self.msg = None;
                self.keep();
                match act {
                    Action::Room(r) => self.page = Page::Room(r),
                    Action::Links => self.page = Page::Links,
                    Action::Openings => self.page = Page::Openings,
                    Action::Options => self.page = Page::Options,
                    Action::Outer => self.page = Page::Outer,
                    Action::PickOuter => {
                        self.page = Page::Done;
                        return deferred("RV_ESTERNOPICK", self.key());
                    }
                    Action::AddDoor => {
                        self.page = Page::Done;
                        return deferred("RV_PORTAPICK", self.key());
                    }
                    Action::Compute => {
                        self.page = Page::Done;
                        commands::set_pending(self.handles.clone(), self.saved.clone());
                        return CommandStep::Dispatch("RV_VANOAPPLY".into());
                    }
                }
                CommandStep::NeedPoint
            }
            (Page::Hub, _) => {
                self.keep();
                self.page = Page::Done;
                CommandStep::Cancel
            }
            (Page::Door(spot), DialogResponse::Submitted(m)) => {
                match self.read_door(&spot, &m) {
                    Ok(()) => {
                        self.msg = None;
                        self.draft.clear();
                        self.keep();
                        self.page = Page::Hub;
                    }
                    Err(e) => {
                        self.msg = Some(e);
                        self.draft = m;
                    }
                }
                CommandStep::NeedPoint
            }
            (page, DialogResponse::Submitted(m)) => {
                match page {
                    Page::Room(r) => self.read_room(r, &m),
                    Page::Links => self.read_links(&m),
                    Page::Openings => self.read_openings(&m),
                    Page::Options => self.read_options(&m),
                    Page::Outer => self.read_outer(&m),
                    _ => {}
                }
                self.keep();
                self.page = Page::Hub;
                CommandStep::NeedPoint
            }
            (_, _) => {
                // "Back": the host sends no values with it; nothing changes.
                self.msg = None;
                self.draft.clear();
                self.page = Page::Hub;
                CommandStep::NeedPoint
            }
        }
    }
}

// ── Letter size ─────────────────────────────────────────────────────────────

/// Asks the height of the corner letters before writing them. Texts in a
/// drawing have a height in drawing units: they grow with the zoom like the
/// lines. A smaller height keeps them apart where corners crowd together.
pub struct LetterSizeDialog {
    key: String,
    def: DialogDef,
    done: bool,
}

impl LetterSizeDialog {
    pub fn new(key: String, h: f64, auto: f64) -> Self {
        let fmt = |v: f64| commands::fmt_num(v, 3);
        let def = DialogDef::new(tr!("Lettere vertici — dimensione", "Corner letters — size", "Letras de esquinas — tamaño", "Lettres des angles — taille", "Köşe harfleri — boyut", "Letras dos cantos — tamanho", "Litery narożników — rozmiar"))
            .with_control(info(tr!(
                format!(
                    "Altezza delle lettere, nelle unità del disegno. Proposta per questi vani: {}. Se le lettere si \
                     ammucchiano su riseghe e spigoli vicini, rimpiccioliscile (per esempio alla metà); se si leggono male \
                     stampate, ingrandiscile. Rieseguendo il comando le lettere vengono riscritte con la nuova altezza.",
                    fmt(auto)
                ),
                format!(
                    "Letter height, in drawing units. Proposed for these rooms: {}. If the letters crowd on recesses and \
                     close corners, make them smaller (for example half); if they read badly in print, make them bigger. \
                     Running the command again rewrites the letters at the new height.",
                    fmt(auto)
                ),
                format!(
                    "Altura de las letras, en unidades del dibujo. Propuesta para estas estancias: {}. Si las letras se amontonan en retranqueos y esquinas cercanas, hágalas más pequeñas (por ejemplo la mitad); si se leen mal impresas, hágalas más grandes. Al ejecutar de nuevo el comando las letras se reescriben con la nueva altura.",
                    fmt(auto)
                ),
                format!(
                    "Hauteur des lettres, en unités du dessin. Proposée pour ces pièces : {}. Si les lettres s'entassent sur les décrochements et les angles rapprochés, réduisez-les (par exemple de moitié) ; si elles se lisent mal à l'impression, agrandissez-les. En relançant la commande, les lettres sont réécrites à la nouvelle hauteur.",
                    fmt(auto)
                ),
                format!(
                    "Harflerin yüksekliği, çizim biriminde. Bu odalar için önerilen: {}. Harfler girinti ve yakın köşelerde üst üste biniyorsa küçültün (örneğin yarıya); baskıda zor okunuyorsa büyütün. Komut yeniden çalıştırıldığında harfler yeni yükseklikle yeniden yazılır.",
                    fmt(auto)
                ),
                format!(
                    "Altura das letras, em unidades do desenho. Proposta para estes compartimentos: {}. Se as letras se amontoam em reentrâncias e cantos próximos, reduza-as (por exemplo para metade); se se leem mal impressas, aumente-as. Ao executar de novo o comando, as letras são reescritas com a nova altura.",
                    fmt(auto)
                ),
                format!(
                    "Wysokość liter w jednostkach rysunku. Proponowana dla tych pomieszczeń: {}. Jeśli litery nakładają się na uskokach i bliskich narożnikach, zmniejsz je (np. o połowę); jeśli źle się czytają po wydruku, powiększ je. Po ponownym uruchomieniu polecenia litery zostaną napisane od nowa z nową wysokością.",
                    fmt(auto)
                ),
            )))
            .with_control(DialogControl::Number {
                id: "h".into(),
                label: tr!("Altezza lettere", "Letter height", "Altura de las letras", "Hauteur des lettres", "Harf yüksekliği", "Altura das letras", "Wysokość liter").into(),
                default: h,
                min: Some(0.0),
                max: None,
                step: Some(auto / 4.0),
            })
            .submit_label(tr!("Scrivi le lettere", "Write the letters", "Escribir las letras", "Écrire les lettres", "Harfleri yaz", "Escrever as letras", "Napisz litery"))
            .cancel_label(tr!("Annulla", "Cancel", "Cancelar", "Annuler", "İptal", "Cancelar", "Anuluj"))
            .width(520.0);
        LetterSizeDialog { key, def, done: false }
    }
}

impl InteractiveCommand for LetterSizeDialog {
    fn prompt(&self) -> String {
        tr!("RV_ETICHETTE: scegli l'altezza delle lettere", "RV_ETICHETTE: choose the letter height", "RV_ETICHETTE: elija la altura de las letras", "RV_ETICHETTE : choisissez la hauteur des lettres", "RV_ETICHETTE: harf yüksekliğini seçin", "RV_ETICHETTE: escolha a altura das letras", "RV_ETICHETTE: wybierz wysokość liter").into()
    }
    fn on_point(&mut self, _pt: [f64; 3]) -> CommandStep {
        CommandStep::NeedPoint
    }
    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Cancel
    }
    fn modal_dialog(&self) -> Option<DialogDef> {
        (!self.done).then(|| self.def.clone())
    }
    fn on_dialog_response(&mut self, response: DialogResponse) -> CommandStep {
        self.done = true;
        let DialogResponse::Submitted(m) = response else {
            return CommandStep::Cancel;
        };
        match get_num(&m, "h").filter(|v| *v > 0.0) {
            Some(h) => deferred("RV_ETICHETTEAPPLY", format!("{} {h}", self.key)),
            None => CommandStep::Cancel,
        }
    }
}

// ── Measures book dialog ────────────────────────────────────────────────────

pub struct BookDialog {
    def: DialogDef,
    done: bool,
}

impl BookDialog {
    pub fn new(default_path: String) -> Self {
        let def = DialogDef::new(tr!("Libretto delle misure", "Measures book", "Libro de medidas", "Carnet de mesures", "Ölçü defteri", "Caderno de medidas", "Dziennik pomiarów"))
            .with_control(info(
                tr!(
                    "Salva l'ultimo calcolo come file CSV: misure con calcolate e scarti, σ₀, coordinate dei vertici, superfici \
                     e perimetri dei vani. Si apre con Excel o LibreOffice Calc.",
                    "Saves the last calculation as a CSV file: measures with computed values and residuals, σ₀, corner \
                     coordinates, room areas and perimeters. It opens in Excel or LibreOffice Calc.",
                    "Guarda el último cálculo como archivo CSV: medidas con valores calculados y residuos, σ₀, coordenadas de las esquinas, superficies y perímetros de las estancias. Se abre con Excel o LibreOffice Calc.",
                    "Enregistre le dernier calcul dans un fichier CSV : mesures avec valeurs calculées et résidus, σ₀, coordonnées des angles, surfaces et périmètres des pièces. Il s'ouvre avec Excel ou LibreOffice Calc.",
                    "Son hesabı CSV dosyası olarak kaydeder: hesaplanan değerler ve artıklarla ölçüler, σ₀, köşe koordinatları, odaların alanları ve çevreleri. Excel veya LibreOffice Calc ile açılır.",
                    "Guarda o último cálculo como ficheiro CSV: medidas com valores calculados e resíduos, σ₀, coordenadas dos cantos, áreas e perímetros dos compartimentos. Abre-se com Excel ou LibreOffice Calc.",
                    "Zapisuje ostatnie obliczenie jako plik CSV: pomiary z wartościami obliczonymi i odchyłkami, σ₀, współrzędne narożników, powierzchnie i obwody pomieszczeń. Otwiera się w Excelu lub LibreOffice Calc.",
                )
                .to_string(),
            ))
            .with_control(csv_picker(&default_path))
            .with_control(info(
                tr!(
                    "Le coordinate sono nel sistema del disegno, con il vertice A (o 1A) dove l'avevi disegnato.",
                    "Coordinates are in the drawing system, with corner A (or 1A) where you drew it.",
                    "Las coordenadas están en el sistema del dibujo, con la esquina A (o 1A) donde la dibujó.",
                    "Les coordonnées sont dans le système du dessin, avec l'angle A (ou 1A) là où vous l'avez dessiné.",
                    "Koordinatlar çizim sistemindedir; A (veya 1A) köşesi onu çizdiğiniz yerdedir.",
                    "As coordenadas estão no sistema do desenho, com o canto A (ou 1A) onde o desenhou.",
                    "Współrzędne są w układzie rysunku, z narożnikiem A (lub 1A) tam, gdzie go narysowano.",
                )
                .to_string(),
            ))
            .submit_label(tr!("Salva", "Save", "Guardar", "Enregistrer", "Kaydet", "Guardar", "Zapisz"))
            .cancel_label(tr!("Annulla", "Cancel", "Cancelar", "Annuler", "İptal", "Cancelar", "Anuluj"))
            .width(560.0);
        BookDialog { def, done: false }
    }
}

impl InteractiveCommand for BookDialog {
    fn prompt(&self) -> String {
        tr!("RV_LIBRETTO: scegli dove salvare il libretto", "RV_LIBRETTO: choose where to save the book", "RV_LIBRETTO: elija dónde guardar el libro", "RV_LIBRETTO : choisissez où enregistrer le carnet", "RV_LIBRETTO: defterin nereye kaydedileceğini seçin", "RV_LIBRETTO: escolha onde guardar o caderno", "RV_LIBRETTO: wybierz, gdzie zapisać dziennik").into()
    }
    fn on_point(&mut self, _pt: [f64; 3]) -> CommandStep {
        CommandStep::NeedPoint
    }
    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Cancel
    }
    fn modal_dialog(&self) -> Option<DialogDef> {
        (!self.done).then(|| self.def.clone())
    }
    fn on_dialog_response(&mut self, response: DialogResponse) -> CommandStep {
        self.done = true;
        let DialogResponse::Submitted(m) = response else {
            return CommandStep::Cancel;
        };
        match get_path(&m, "csv_path") {
            Some(p) => {
                commands::set_pending_book_path(p);
                CommandStep::Dispatch("RV_LIBRETTOAPPLY".into())
            }
            None => CommandStep::Cancel,
        }
    }
}

// ── Guide ───────────────────────────────────────────────────────────────────

pub fn guide_sections() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            tr!("Avvertenza", "Notice", "Aviso", "Avertissement", "Uyarı", "Aviso", "Uwaga"),
            tr!(
                "Progetto hobbistico, realizzato per divertimento e non adatto a un uso professionale: controlla sempre i \
                 risultati. L'autore non si assume responsabilità per il suo uso.",
                "Hobby project, made for fun and not suitable for professional use: always check the results. The author \
                 takes no responsibility for its use.",
                "Proyecto aficionado, hecho por diversión y no apto para uso profesional: compruebe siempre los resultados. El autor no asume ninguna responsabilidad por su uso.",
                "Projet amateur, réalisé pour le plaisir et non adapté à un usage professionnel : vérifiez toujours les résultats. L'auteur décline toute responsabilité quant à son utilisation.",
                "Hobi projesi, eğlence için yapılmıştır ve profesyonel kullanıma uygun değildir: sonuçları her zaman kontrol edin. Yazar, kullanımından doğacak hiçbir sorumluluğu kabul etmez.",
                "Projeto amador, feito por diversão e não adequado a uso profissional: verifique sempre os resultados. O autor não assume qualquer responsabilidade pelo seu uso.",
                "Projekt hobbystyczny, zrobiony dla zabawy i nieprzeznaczony do użytku zawodowego: zawsze sprawdzaj wyniki. Autor nie ponosi żadnej odpowiedzialności za jego użycie.",
            ),
        ),
        (
            tr!("1. Disegna i vani", "1. Draw the rooms", "1. Dibuje las estancias", "1. Dessinez les pièces", "1. Odaları çizin", "1. Desenhe os compartimentos", "1. Narysuj pomieszczenia"),
            tr!(
                "Disegna ogni vano come polilinea chiusa, a occhio, con i comandi del CAD (PLINEA … Chiudi). Contano solo \
                 il numero e l'ordine dei vertici: forma e dimensioni le sistema il plugin.",
                "Draw each room as a closed polyline, by eye, with the CAD's commands (PLINE … Close). Only the number and \
                 order of the corners matter: the plugin fixes shape and size.",
                "Dibuje cada estancia como polilínea cerrada, a ojo, con los comandos del CAD (PLINE … Cerrar). Solo importan el número y el orden de las esquinas: el plugin corrige forma y tamaño.",
                "Dessinez chaque pièce comme une polyligne fermée, à main levée, avec les commandes du CAO (PLINE … Clore). Seuls le nombre et l'ordre des angles comptent : le plugin corrige forme et dimensions.",
                "Her odayı CAD komutlarıyla (PLINE … Kapat) göz kararı kapalı bir çoklu çizgi olarak çizin. Yalnızca köşelerin sayısı ve sırası önemlidir: eklenti şekli ve boyutu düzeltir.",
                "Desenhe cada compartimento como polilinha fechada, à mão, com os comandos do CAD (PLINE … Fechar). Só importam o número e a ordem dos cantos: o plugin corrige a forma e as dimensões.",
                "Narysuj każde pomieszczenie jako zamkniętą polilinię, na oko, poleceniami CAD (PLINE … Zamknij). Liczą się tylko liczba i kolejność narożników: wtyczka poprawia kształt i wymiary.",
            ),
        ),
        (
            tr!("2. Lettere vertici", "2. Corner letters", "2. Letras de esquinas", "2. Lettres des angles", "2. Köşe harfleri", "2. Letras dos cantos", "2. Litery narożników"),
            tr!(
                "Seleziona i vani (un clic sulla linea, o una finestra per prenderne tanti; Invio): sugli angoli compaiono A, B, C … o, con più vani, 1A, \
                 1B … 2A … Una piccola finestra chiede l'altezza delle lettere: rimpiccioliscile dove gli spigoli sono fitti. \
                 Stampa la pianta e usala in cantiere per segnare le misure.",
                "Select the rooms (click a line, or drag a window for many; Enter): A, B, C … or, with several rooms, 1A, 1B … 2A … \
                 appear at the corners. A small window asks the letter height: make it smaller where corners crowd together. \
                 Print the plan and use it on site to note the measures.",
                "Seleccione las estancias (clic en una línea, o una ventana para muchas; Intro): en las esquinas aparecen A, B, C … o, con varias estancias, 1A, 1B … 2A … Una ventanita pide la altura de las letras: hágalas más pequeñas donde las esquinas están juntas. Imprima el plano y úselo en obra para anotar las medidas.",
                "Sélectionnez les pièces (clic sur une ligne, ou une fenêtre pour plusieurs ; Entrée) : A, B, C … ou, avec plusieurs pièces, 1A, 1B … 2A … apparaissent aux angles. Une petite fenêtre demande la hauteur des lettres : réduisez-la là où les angles sont serrés. Imprimez le plan et servez-vous-en sur le chantier pour noter les mesures.",
                "Odaları seçin (bir çizgiye tıklayın veya birçoğu için pencere; Enter): köşelerde A, B, C … veya birden çok odada 1A, 1B … 2A … belirir. Küçük bir pencere harf yüksekliğini sorar: köşelerin sık olduğu yerde küçültün. Planı yazdırın ve sahada ölçüleri not etmek için kullanın.",
                "Selecione os compartimentos (clique numa linha, ou uma janela para muitos; Enter): nos cantos aparecem A, B, C … ou, com vários compartimentos, 1A, 1B … 2A … Uma janelinha pede a altura das letras: reduza-a onde os cantos estão próximos. Imprima a planta e use-a na obra para anotar as medidas.",
                "Zaznacz pomieszczenia (kliknij linię albo okno dla wielu; Enter): w narożnikach pojawiają się A, B, C … lub przy wielu pomieszczeniach 1A, 1B … 2A … Małe okno pyta o wysokość liter: zmniejsz ją tam, gdzie narożniki są blisko. Wydrukuj rzut i użyj go na budowie do zapisania pomiarów.",
            ),
        ),
        (
            tr!("3. Misure e diagonali", "3. Sides and diagonals", "3. Lados y diagonales", "3. Côtés et diagonales", "3. Kenarlar ve köşegenler", "3. Lados e diagonais", "3. Boki i przekątne"),
            tr!(
                "Scegli gli stessi vani: si apre un elenco dei vani con il loro stato. Apri un vano alla volta e scrivi lati e \
                 diagonali (un vano di n spigoli vuole tutti i lati e almeno n−3 diagonali o vincoli); a ogni OK l'elenco \
                 passa al vano successivo e alla fine propone Calcola. Le misure restano memorizzate anche chiudendo la finestra.",
                "Choose the same rooms: a list of the rooms with their state opens. Open one room at a time and type sides and \
                 diagonals (a room of n corners needs all sides and at least n−3 diagonals or constraints); at every OK the \
                 list moves to the next room and at the end proposes Compute. The measures are kept even if you close the window.",
                "Elija las mismas estancias: se abre una lista de las estancias con su estado. Abra una estancia cada vez y escriba lados y diagonales (una estancia de n esquinas necesita todos los lados y al menos n−3 diagonales o restricciones); con cada OK la lista pasa a la estancia siguiente y al final propone Calcular. Las medidas se conservan aunque cierre la ventana.",
                "Choisissez les mêmes pièces : une liste des pièces avec leur état s'ouvre. Ouvrez une pièce à la fois et saisissez côtés et diagonales (une pièce de n angles demande tous les côtés et au moins n−3 diagonales ou contraintes) ; à chaque OK la liste passe à la pièce suivante et à la fin propose Calculer. Les mesures sont conservées même si vous fermez la fenêtre.",
                "Aynı odaları seçin: odaların durumlarıyla bir listesi açılır. Her seferinde bir oda açın ve kenarları ve köşegenleri girin (n köşeli bir oda tüm kenarları ve en az n−3 köşegen veya kısıt gerektirir); her Tamam'da liste sonraki odaya geçer ve sonunda Hesapla'yı önerir. Ölçüler pencereyi kapatsanız da saklanır.",
                "Escolha os mesmos compartimentos: abre-se uma lista dos compartimentos com o seu estado. Abra um compartimento de cada vez e escreva lados e diagonais (um compartimento de n cantos precisa de todos os lados e de pelo menos n−3 diagonais ou restrições); a cada OK a lista passa ao compartimento seguinte e no fim propõe Calcular. As medidas ficam guardadas mesmo que feche a janela.",
                "Wybierz te same pomieszczenia: otwiera się lista pomieszczeń z ich stanem. Otwieraj po jednym pomieszczeniu i wpisuj boki i przekątne (pomieszczenie o n narożnikach wymaga wszystkich boków i co najmniej n−3 przekątnych lub warunków); po każdym OK lista przechodzi do następnego pomieszczenia, a na końcu proponuje Oblicz. Pomiary są zachowywane, nawet jeśli zamkniesz okno.",
            ),
        ),
        (
            tr!("Porte, finestre e muri tra i vani", "Doors, windows and walls between rooms", "Puertas, ventanas y muros entre estancias", "Portes, fenêtres et murs entre pièces", "Kapılar, pencereler ve odalar arası duvarlar", "Portas, janelas e paredes entre compartimentos", "Drzwi, okna i ściany między pomieszczeniami"),
            tr!(
                "Dall'elenco scegli \"Aggiungi porta o finestra\" e clicca sul disegno, sul muro, dove si trova. Tra due vani il \
                 plugin riconosce i due lati: scrivi la distanza dallo spigolo allo stipite nei due vani, la larghezza e lo \
                 spessore del muro misurato nella porta. Basta questo a collegare i due vani. Un angolo noto o un lato \
                 orizzontale / verticale può prendere il posto di una diagonale.",
                "From the list choose \"Add a door or window\" and click on the drawing, on the wall, where it is. Between two \
                 rooms the plugin finds both sides: type the distance from the corner to the jamb in both rooms, the width \
                 and the wall thickness measured in the door. That alone links the two rooms. A known angle or a horizontal / \
                 vertical side can take the place of a diagonal.",
                "En la lista elija \"Añadir puerta o ventana\" y haga clic en el dibujo, sobre el muro, donde se encuentra. Entre dos estancias el plugin encuentra los dos lados: escriba la distancia de la esquina a la jamba en las dos estancias, la anchura y el espesor del muro medido en la puerta. Basta con eso para enlazar las dos estancias. Un ángulo conocido o un lado horizontal / vertical puede sustituir a una diagonal.",
                "Dans la liste, choisissez \"Ajouter une porte ou une fenêtre\" et cliquez sur le dessin, sur le mur, à l'endroit où elle se trouve. Entre deux pièces, le plugin trouve les deux côtés : saisissez la distance de l'angle au tableau dans les deux pièces, la largeur et l'épaisseur du mur mesurée dans la porte. Cela suffit à lier les deux pièces. Un angle connu ou un côté horizontal / vertical peut remplacer une diagonale.",
                "Listede \"Kapı veya pencere ekle\" seçin ve çizimde, duvarın üzerinde, bulunduğu yere tıklayın. İki oda arasında eklenti iki kenarı bulur: iki odada köşeden kasaya uzaklığı, genişliği ve kapıda ölçülen duvar kalınlığını girin. Bu, iki odayı bağlamaya yeter. Bilinen bir açı veya yatay / dikey bir kenar bir köşegenin yerini tutabilir.",
                "Na lista escolha \"Acrescentar porta ou janela\" e clique no desenho, sobre a parede, onde ela se encontra. Entre dois compartimentos o plugin encontra os dois lados: escreva a distância do canto à ombreira nos dois compartimentos, a largura e a espessura da parede medida na porta. Isso basta para ligar os dois compartimentos. Um ângulo conhecido ou um lado horizontal / vertical pode substituir uma diagonal.",
                "Na liście wybierz \"Dodaj drzwi lub okno\" i kliknij na rysunku, na ścianie, w miejscu, gdzie się znajdują. Między dwoma pomieszczeniami wtyczka znajduje oba boki: wpisz odległość od narożnika do ościeża w obu pomieszczeniach, szerokość i grubość ściany zmierzoną w drzwiach. To wystarczy, by powiązać oba pomieszczenia. Znany kąt albo bok poziomy / pionowy może zastąpić przekątną.",
            ),
        ),
        (
            tr!("Poligono esterno fisso", "Fixed outer outline", "Polígono exterior fijo", "Polygone extérieur fixe", "Sabit dış poligon", "Polígono exterior fixo", "Stały obrys zewnętrzny"),
            tr!(
                "Se il perimetro esterno è stato rilevato con la stazione totale, disegnalo come polilinea chiusa e, nella \
                 finestra delle misure, scegli \"Poligono esterno fisso\" e cliccalo: resta fermo. Scrivi lo spessore dei muri \
                 esterni e controlla i lati dei vani proposti lungo il perimetro: nel calcolo restano a quella distanza dal \
                 poligono esterno, e i vani si posizionano da soli dentro l'edificio, con meno diagonali.",
                "If the outer perimeter was surveyed with a total station, draw it as a closed polyline and, in the measures \
                 window, choose \"Fixed outer outline\" and click it: it stays still. Type the thickness of the outer walls and \
                 check the room sides proposed along the perimeter: the calculation keeps them at that distance from the \
                 outline, and the rooms place themselves inside the building, with fewer diagonals.",
                "Si el perímetro exterior se ha levantado con la estación total, dibújelo como polilínea cerrada y, en la ventana de medidas, elija \"Polígono exterior fijo\" y haga clic en él: queda fijo. Escriba el espesor de los muros exteriores y compruebe los lados de las estancias propuestos a lo largo del perímetro: el cálculo los mantiene a esa distancia del polígono exterior, y las estancias se colocan solas dentro del edificio, con menos diagonales.",
                "Si le périmètre extérieur a été relevé à la station totale, dessinez-le comme une polyligne fermée et, dans la fenêtre des mesures, choisissez \"Polygone extérieur fixe\" et cliquez dessus : il reste fixe. Saisissez l'épaisseur des murs extérieurs et vérifiez les côtés des pièces proposés le long du périmètre : le calcul les maintient à cette distance du polygone extérieur, et les pièces se placent d'elles-mêmes dans le bâtiment, avec moins de diagonales.",
                "Dış çevre total station ile ölçüldüyse, onu kapalı bir çoklu çizgi olarak çizin ve ölçü penceresinde \"Sabit dış poligon\" seçip ona tıklayın: yerinden oynamaz. Dış duvarların kalınlığını girin ve çevre boyunca önerilen oda kenarlarını kontrol edin: hesap onları dış poligondan bu uzaklıkta tutar ve odalar daha az köşegenle binanın içine kendiliğinden yerleşir.",
                "Se o perímetro exterior foi levantado com a estação total, desenhe-o como polilinha fechada e, na janela das medidas, escolha \"Polígono exterior fixo\" e clique nele: fica fixo. Escreva a espessura das paredes exteriores e verifique os lados dos compartimentos propostos ao longo do perímetro: o cálculo mantém-nos a essa distância do polígono exterior, e os compartimentos colocam-se sozinhos dentro do edifício, com menos diagonais.",
                "Jeśli obrys zewnętrzny zmierzono tachimetrem, narysuj go jako zamkniętą polilinię i w oknie pomiarów wybierz \"Stały obrys zewnętrzny\", a następnie kliknij go: pozostaje nieruchomy. Wpisz grubość ścian zewnętrznych i sprawdź boki pomieszczeń zaproponowane wzdłuż obrysu: obliczenie utrzymuje je w tej odległości od obrysu, a pomieszczenia same ustawiają się wewnątrz budynku, z mniejszą liczbą przekątnych.",
            ),
        ),
        (
            tr!("4. Risultato", "4. Result", "4. Resultado", "4. Résultat", "4. Sonuç", "4. Resultado", "4. Wynik"),
            tr!(
                "I vani veri vanno sul layer RILIEVO_CORRETTO con la loro superficie in m²; il disegno originale resta. Porte \
                 e finestre su RILIEVO_APERTURE. Su RILIEVO_QUOTE: lettere, quote dei lati (stile RILIEVO), diagonali, \
                 collegamenti, spessori e la tabella misurata / calcolata / scarto, \
                 con in rosso gli scarti oltre la tolleranza. Se le misure non bastano o non tornano, la finestra si \
                 riapre con il motivo.",
                "The true rooms go on layer RILIEVO_CORRETTO with their area in m²; the original drawing stays. Doors and \
                 windows on RILIEVO_APERTURE. On RILIEVO_QUOTE: letters, side dimensions (style RILIEVO), diagonals, links, \
                 thicknesses and the measured / computed / residual table, \
                 residuals over the tolerance in red. If the measures are not enough or do not fit, the dialog reopens \
                 with the reason.",
                "Las estancias reales van a la capa RILIEVO_CORRETTO con su superficie en m²; el dibujo original se queda. Puertas y ventanas en RILIEVO_APERTURE. En RILIEVO_QUOTE: letras, cotas de los lados (estilo RILIEVO), diagonales, enlaces, espesores y la tabla medida / calculada / residuo, con en rojo los residuos fuera de tolerancia. Si las medidas no bastan o no encajan, la ventana se vuelve a abrir con el motivo.",
                "Les pièces réelles vont sur le calque RILIEVO_CORRETTO avec leur surface en m² ; le dessin d'origine reste. Portes et fenêtres sur RILIEVO_APERTURE. Sur RILIEVO_QUOTE : lettres, cotes des côtés (style RILIEVO), diagonales, liaisons, épaisseurs et le tableau mesurée / calculée / résidu, avec en rouge les résidus hors tolérance. Si les mesures ne suffisent pas ou ne concordent pas, la fenêtre se rouvre avec la raison.",
                "Gerçek odalar alanlarıyla (m²) RILIEVO_CORRETTO katmanına gider; özgün çizim kalır. Kapılar ve pencereler RILIEVO_APERTURE üzerinde. RILIEVO_QUOTE üzerinde: harfler, kenar ölçülendirmeleri (RILIEVO stili), köşegenler, bağlantılar, kalınlıklar ve ölçülen / hesaplanan / artık tablosu; tolerans dışı artıklar kırmızı. Ölçüler yetmez veya uyuşmazsa pencere nedeniyle birlikte yeniden açılır.",
                "Os compartimentos reais vão para a camada RILIEVO_CORRETTO com a sua área em m²; o desenho original fica. Portas e janelas em RILIEVO_APERTURE. Em RILIEVO_QUOTE: letras, cotas dos lados (estilo RILIEVO), diagonais, ligações, espessuras e a tabela medida / calculada / resíduo, com a vermelho os resíduos fora da tolerância. Se as medidas não chegam ou não batem certo, a janela volta a abrir-se com o motivo.",
                "Rzeczywiste pomieszczenia trafiają na warstwę RILIEVO_CORRETTO z powierzchnią w m²; oryginalny rysunek zostaje. Drzwi i okna na RILIEVO_APERTURE. Na RILIEVO_QUOTE: litery, wymiary boków (styl RILIEVO), przekątne, powiązania, grubości i tabela zmierzony / obliczony / odchyłka, z odchyłkami poza tolerancją na czerwono. Jeśli pomiarów jest za mało lub się nie zgadzają, okno otwiera się ponownie z podaniem przyczyny.",
            ),
        ),
        (
            tr!("Applica al disegno", "Apply to drawing", "Aplicar al dibujo", "Appliquer au dessin", "Çizime uygula", "Aplicar ao desenho", "Zastosuj na rysunku"),
            tr!(
                "Controllato il risultato, Applica al disegno (RV_APPLICA) porta i vertici delle polilinee originali nella \
                 posizione calcolata: restano le stesse polilinee, sullo stesso layer, e la copia su RILIEVO_CORRETTO viene \
                 tolta. Ctrl+Z annulla. Nelle Opzioni si può scegliere di farlo subito a ogni calcolo.",
                "Once the result is checked, Apply to drawing (RV_APPLICA) moves the corners of the original polylines to \
                 the computed position: same polylines, same layer, and the copy on RILIEVO_CORRETTO is removed. Ctrl+Z \
                 undoes it. In Options you can choose to do it at every calculation.",
                "Una vez comprobado el resultado, Aplicar al dibujo (RV_APPLICA) lleva las esquinas de las polilíneas originales a la posición calculada: mismas polilíneas, misma capa, y la copia en RILIEVO_CORRETTO se elimina. Ctrl+Z lo deshace. En Opciones se puede elegir hacerlo en cada cálculo.",
                "Une fois le résultat vérifié, Appliquer au dessin (RV_APPLICA) amène les angles des polylignes d'origine à la position calculée : mêmes polylignes, même calque, et la copie sur RILIEVO_CORRETTO est supprimée. Ctrl+Z l'annule. Dans Options vous pouvez choisir de le faire à chaque calcul.",
                "Sonucu kontrol ettikten sonra Çizime uygula (RV_APPLICA) özgün çoklu çizgilerin köşelerini hesaplanan konuma getirir: aynı çoklu çizgiler, aynı katman ve RILIEVO_CORRETTO üzerindeki kopya kaldırılır. Ctrl+Z geri alır. Seçenekler'de bunu her hesapta yapmayı seçebilirsiniz.",
                "Depois de verificar o resultado, Aplicar ao desenho (RV_APPLICA) leva os cantos das polilinhas originais à posição calculada: as mesmas polilinhas, a mesma camada, e a cópia em RILIEVO_CORRETTO é eliminada. Ctrl+Z anula. Em Opções pode escolher fazê-lo a cada cálculo.",
                "Po sprawdzeniu wyniku Zastosuj na rysunku (RV_APPLICA) przenosi narożniki oryginalnych polilinii do obliczonego położenia: te same polilinie, ta sama warstwa, a kopia na RILIEVO_CORRETTO zostaje usunięta. Ctrl+Z cofa. W Opcjach można wybrać, by robić to przy każdym obliczeniu.",
            ),
        ),
        (
            tr!("5. Libretto delle misure", "5. Measures book", "5. Libro de medidas", "5. Carnet de mesures", "5. Ölçü defteri", "5. Caderno de medidas", "5. Dziennik pomiarów"),
            tr!(
                "Libretto misure (RV_LIBRETTO) salva l'ultimo calcolo in un CSV per Excel o Calc: misure, scarti, σ₀, \
                 coordinate dei vertici, superfici e perimetri. Si può anche spuntare nella finestra delle misure.",
                "Measures book (RV_LIBRETTO) saves the last calculation as a CSV for Excel or Calc: measures, residuals, \
                 σ₀, corner coordinates, areas and perimeters. It can also be ticked in the measures dialog.",
                "Libro de medidas (RV_LIBRETTO) guarda el último cálculo en un CSV para Excel o Calc: medidas, residuos, σ₀, coordenadas de las esquinas, superficies y perímetros. También se puede marcar en la ventana de medidas.",
                "Carnet de mesures (RV_LIBRETTO) enregistre le dernier calcul dans un CSV pour Excel ou Calc : mesures, résidus, σ₀, coordonnées des angles, surfaces et périmètres. On peut aussi le cocher dans la fenêtre des mesures.",
                "Ölçü defteri (RV_LIBRETTO) son hesabı Excel veya Calc için bir CSV'ye kaydeder: ölçüler, artıklar, σ₀, köşe koordinatları, alanlar ve çevreler. Ölçü penceresinde de işaretlenebilir.",
                "Caderno de medidas (RV_LIBRETTO) guarda o último cálculo num CSV para Excel ou Calc: medidas, resíduos, σ₀, coordenadas dos cantos, áreas e perímetros. Também se pode assinalar na janela das medidas.",
                "Dziennik pomiarów (RV_LIBRETTO) zapisuje ostatnie obliczenie w pliku CSV dla Excela lub Calc: pomiary, odchyłki, σ₀, współrzędne narożników, powierzchnie i obwody. Można go też zaznaczyć w oknie pomiarów.",
            ),
        ),
        (
            tr!("Manuale di rilievo (PDF)", "Surveying manual (PDF)", "Manual de levantamiento (PDF)", "Manuel de relevé (PDF)", "Ölçüm kılavuzu (PDF)", "Manual de levantamento (PDF)", "Podręcznik pomiaru (PDF)"),
            tr!(
                "Il pulsante Manuale PDF apre una guida passo passo, con disegni, per rilevare un intero appartamento: \
                 eidotipo, misure dei vani, diagonali e controlli, mobili che ostacolano, porte comuni, finestre e muri \
                 esterni, lettura degli scarti, lista di controllo per il cantiere. (In italiano.)",
                "The PDF manual button opens a step-by-step guide, with drawings, to survey a whole flat: sketch, room \
                 measures, diagonals and checks, furniture in the way, shared doors, windows and outer walls, reading the \
                 residuals, a site checklist.",
                "El botón Manual PDF abre una guía paso a paso, con dibujos, para levantar una vivienda entera: croquis, medidas de las estancias, diagonales y controles, muebles que estorban, puertas comunes, ventanas y muros exteriores, lectura de los residuos, lista de comprobación para la obra.",
                "Le bouton Manuel PDF ouvre un guide pas à pas, avec dessins, pour relever un logement entier : croquis, mesures des pièces, diagonales et contrôles, meubles gênants, portes communes, fenêtres et murs extérieurs, lecture des résidus, liste de contrôle pour le chantier.",
                "PDF kılavuz düğmesi, bütün bir konutu ölçmek için çizimli, adım adım bir rehber açar: kroki, oda ölçüleri, köşegenler ve kontroller, engel olan mobilyalar, ortak kapılar, pencereler ve dış duvarlar, artıkların okunması, saha için kontrol listesi.",
                "O botão Manual PDF abre um guia passo a passo, com desenhos, para levantar uma habitação inteira: croqui, medidas dos compartimentos, diagonais e controlos, móveis que atrapalham, portas comuns, janelas e paredes exteriores, leitura dos resíduos, lista de verificação para a obra.",
                "Przycisk Podręcznik PDF otwiera przewodnik krok po kroku, z rysunkami, do pomiaru całego mieszkania: szkic, pomiary pomieszczeń, przekątne i kontrole, przeszkadzające meble, wspólne drzwi, okna i ściany zewnętrzne, odczyt odchyłek, lista kontrolna na budowę.",
            ),
        ),
        (
            tr!("Comandi", "Commands", "Comandos", "Commandes", "Komutlar", "Comandos", "Polecenia"),
            tr!(
                "RV_VANO, RV_ETICHETTE, RV_APPLICA, RV_LIBRETTO, RV_GUIDE, RV_MANUALE; RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO cambia la lingua.",
                "RV_VANO, RV_ETICHETTE, RV_APPLICA, RV_LIBRETTO, RV_GUIDE, RV_MANUALE; RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO changes the language.",
                "RV_VANO, RV_ETICHETTE, RV_APPLICA, RV_LIBRETTO, RV_GUIDE, RV_MANUALE; RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO cambia el idioma.",
                "RV_VANO, RV_ETICHETTE, RV_APPLICA, RV_LIBRETTO, RV_GUIDE, RV_MANUALE ; RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO change la langue.",
                "RV_VANO, RV_ETICHETTE, RV_APPLICA, RV_LIBRETTO, RV_GUIDE, RV_MANUALE; RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO dili değiştirir.",
                "RV_VANO, RV_ETICHETTE, RV_APPLICA, RV_LIBRETTO, RV_GUIDE, RV_MANUALE; RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO muda o idioma.",
                "RV_VANO, RV_ETICHETTE, RV_APPLICA, RV_LIBRETTO, RV_GUIDE, RV_MANUALE; RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO zmienia język.",
            ),
        ),
    ]
}

pub struct GuideDialog {
    def: DialogDef,
    done: bool,
}

impl GuideDialog {
    pub fn new() -> Self {
        let mut def = DialogDef::new(tr!("Room Survey — guida", "Room Survey — guide", "Room Survey — guía", "Room Survey — guide", "Room Survey — rehber", "Room Survey — guia", "Pomoc Room Survey"));
        for (title, body) in guide_sections() {
            def = def.with_control(info(title.to_string())).with_control(info(body.to_string()));
        }
        def = def.submit_label("OK").cancel_label(tr!("Chiudi", "Close", "Cerrar", "Fermer", "Kapat", "Fechar", "Zamknij")).width(600.0);
        GuideDialog { def, done: false }
    }
}

impl Default for GuideDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveCommand for GuideDialog {
    fn prompt(&self) -> String {
        tr!("Guida di Room Survey", "Room Survey guide", "Guía de Room Survey", "Guide de Room Survey", "Room Survey rehberi", "Guia do Room Survey", "Przewodnik Room Survey").into()
    }
    fn on_point(&mut self, _pt: [f64; 3]) -> CommandStep {
        CommandStep::NeedPoint
    }
    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Done
    }
    fn modal_dialog(&self) -> Option<DialogDef> {
        (!self.done).then(|| self.def.clone())
    }
    fn on_dialog_response(&mut self, _response: DialogResponse) -> CommandStep {
        self.done = true;
        CommandStep::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn saved_for(survey: &Survey) -> Saved {
        let (sides, others) = survey.initial(3, &|v, d| format!("{v:.d$}"));
        Saved {
            sides,
            others,
            h: 0.2,
            tol: 0.02,
            dims: true,
            table: true,
            csv: false,
            csv_path: String::new(),
            constraints: vec![],
            openings: vec![],
            unit: 0,
            fill: true,
            replace: false,
            outer: String::new(),
            outer_t: String::new(),
            outer_walls: vec![],
        }
    }

    fn two() -> Survey {
        Survey::new(vec![
            vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]],
            vec![[4.3, 0.0], [7.0, 0.0], [7.0, 3.0], [4.3, 3.0]],
        ])
    }

    fn text(m: &mut HashMap<String, DialogValue>, id: &str, v: &str) {
        m.insert(id.to_string(), DialogValue::Text(v.to_string()));
    }

    fn pick(d: &MeasureDialog, want: Action) -> HashMap<String, DialogValue> {
        let (acts, labels, _) = d.actions();
        let index = acts.iter().position(|a| *a == want).unwrap();
        let mut m = HashMap::new();
        m.insert("do".to_string(), DialogValue::Choice { index, text: labels[index].clone() });
        m
    }

    #[test]
    fn rooms_chosen_by_window_selection() {
        let room = |h: u64, x: f64, y: f64| Room {
            handle: Handle::new(h),
            pts: vec![[x, y], [x + 3.0, y], [x + 3.0, y + 2.0], [x, y + 2.0]],
            z: 0.0,
            had_arcs: false,
        };
        // Two rows of rooms: 1, 2 on top; 3, 4 below.
        let rooms = vec![room(1, 4.0, 0.0), room(2, 0.0, 3.0), room(3, 0.0, 0.1), room(4, 4.0, 3.2)];
        let mut t = PickRoomTool::new(rooms, "RV_VANOOPEN", "RV_VANO");
        assert!(t.is_selection_gathering() && t.accepts_drag_selection());
        // One room clicked first, then a window over everything (plus a stray object).
        t.on_selection_complete(vec![Handle::new(3)]);
        t.on_selection_complete(vec![Handle::new(1), Handle::new(2), Handle::new(3), Handle::new(4), Handle::new(99)]);
        let order: Vec<u64> = t.chosen.iter().map(|&i| t.rooms[i].handle.value()).collect();
        assert_eq!(order, vec![3, 2, 4, 1], "first pick stays first, the rest in reading order");
        assert_eq!(t.on_cursor_move([0.0; 3]).len(), 4);
        // One deselected in the CAD.
        t.on_selection_complete(vec![Handle::new(2), Handle::new(3), Handle::new(1)]);
        assert_eq!(t.chosen.len(), 3);
        assert!(matches!(t.on_enter(), CommandStep::Dispatch(ref c) if c == "RV_VANOOPEN"));
        assert_eq!(take_deferred("RV_VANOOPEN").as_deref(), Some("3,2,1"));
    }

    #[test]
    fn spot_between_two_rooms() {
        let s = two();
        // In the wall between the rooms, near the bottom.
        let spot = find_spot(&s.rooms, [4.1, 0.8]).unwrap();
        assert_eq!(spot, DoorSpot { room: 0, from: 1, to: 2, other: Some((1, 0, 3)) });
        assert_eq!(DoorSpot::decode(&spot.encode()), Some(spot));
        // On an outer wall: no room behind.
        let spot = find_spot(&s.rooms, [2.0, 3.05]).unwrap();
        assert_eq!((spot.room, spot.other), (0, None));
        // On the far side of room 2: not matched with room 1 across it.
        let spot = find_spot(&s.rooms, [7.02, 1.0]).unwrap();
        assert_eq!((spot.room, spot.other), (1, None));
        assert!(find_spot(&s.rooms, [50.0, 50.0]).is_none());
    }

    #[test]
    fn hub_walks_through_the_rooms() {
        let s = two();
        let mut d = MeasureDialog::new(vec![Handle::new(0x71), Handle::new(0x72)], &s, &saved_for(&s), None);
        let (acts, _, default) = d.actions();
        assert_eq!(acts[default], Action::Room(0), "first incomplete room is proposed");
        assert!(d.modal_dialog().unwrap().title.contains('2'));
        // Open room 2 and fill it, with bare letters for an extra diagonal.
        let m = pick(&d, Action::Room(1));
        assert!(matches!(d.on_dialog_response(DialogResponse::Submitted(m)), CommandStep::NeedPoint));
        let mut m = HashMap::new();
        for i in 0..4 {
            text(&mut m, &format!("s{i}"), "3,00");
        }
        text(&mut m, "o0a", "2A");
        text(&mut m, "o0b", "2C");
        text(&mut m, "o0m", "4,03");
        text(&mut m, "o1a", "b");
        text(&mut m, "o1b", "d");
        text(&mut m, "o1m", "4,04");
        text(&mut m, "o2a", "");
        text(&mut m, "o2b", "");
        text(&mut m, "o2m", "");
        text(&mut m, "c0a", "a");
        text(&mut m, "c0b", "90");
        d.on_dialog_response(DialogResponse::Submitted(m));
        assert_eq!(d.page, Page::Hub);
        assert_eq!(d.saved.sides[4][2], "3,00");
        assert!(d.saved.others.iter().any(|r| r[0] == "2B" && r[1] == "2D" && r[3] == "4,04"), "{:?}", d.saved.others);
        assert_eq!(d.saved.constraints, vec![vec!["2A".to_string(), "90".into()]]);
        // Room 1's proposed diagonal is untouched and still first.
        assert!(d.saved.others[0][0].starts_with('1'));
        let (ok, status) = d.room_status(1);
        assert!(ok && status.contains('2'), "{status}");
        assert!(!d.room_status(0).0);
        // Back from a page changes nothing.
        let m = pick(&d, Action::Options);
        d.on_dialog_response(DialogResponse::Submitted(m));
        assert_eq!(d.page, Page::Options);
        d.on_dialog_response(DialogResponse::Cancelled);
        assert_eq!(d.page, Page::Hub);
        // Compute hands over to the command.
        let m = pick(&d, Action::Compute);
        assert!(matches!(d.on_dialog_response(DialogResponse::Submitted(m)), CommandStep::Dispatch(ref c) if c == "RV_VANOAPPLY"));
        assert!(d.modal_dialog().is_none());
    }

    #[test]
    fn door_by_click_links_the_rooms() {
        let s = two();
        let mut saved = saved_for(&s);
        for r in saved.sides.iter_mut() {
            r[2] = r[1].clone();
        }
        for r in saved.others.iter_mut() {
            r[3] = r[2].clone();
        }
        let spot = find_spot(&s.rooms, [4.1, 0.8]).unwrap();
        let mut d = MeasureDialog::new(vec![Handle::new(0x73), Handle::new(0x74)], &s, &saved, None).starting(Start::Door(spot.clone()));
        assert_eq!(d.unlinked().unwrap(), vec![2]);
        assert!(d.modal_dialog().unwrap().title.contains('1'));
        // Missing width: stays on the page, keeps what was typed.
        let mut m = HashMap::new();
        text(&mut m, "name", "P1");
        text(&mut m, "d1", "1,00");
        d.on_dialog_response(DialogResponse::Submitted(m.clone()));
        assert_eq!(d.page, Page::Door(spot));
        assert!(d.msg.is_some() && d.saved.openings.is_empty());
        text(&mut m, "w", "0,90");
        text(&mut m, "t", "0,30");
        text(&mut m, "d2", "1,00");
        d.on_dialog_response(DialogResponse::Submitted(m));
        assert_eq!(d.page, Page::Hub);
        assert_eq!(d.saved.openings.len(), 2);
        assert_eq!(d.saved.openings[0][1], "1B-1C");
        assert_eq!(d.saved.openings[1][1], "2A-2D");
        assert!(d.unlinked().unwrap().is_empty(), "the door links the rooms");
        let (acts, _, default) = d.actions();
        assert_eq!(acts[default], Action::Compute);
        assert!(d.opening_lines()[0].contains("↔"), "{:?}", d.opening_lines());
    }

    #[test]
    fn door_without_second_distance_gives_wall_thickness() {
        let s = two();
        let spot = find_spot(&s.rooms, [4.1, 0.8]).unwrap();
        let mut d = MeasureDialog::new(vec![Handle::new(0x75), Handle::new(0x76)], &s, &saved_for(&s), None).starting(Start::Door(spot));
        let mut m = HashMap::new();
        text(&mut m, "d1", "1");
        text(&mut m, "w", "0,9");
        text(&mut m, "t", "0,30");
        d.on_dialog_response(DialogResponse::Submitted(m));
        assert_eq!(d.saved.openings.len(), 1);
        let walls: Vec<_> = d.saved.others.iter().filter(|r| r[0] == "1B-1C").collect();
        assert_eq!(walls.len(), 2);
        assert_eq!(walls[0][3], "0,30");
    }

    #[test]
    fn single_room_uses_plain_letters() {
        let s = Survey::new(vec![vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]]]);
        let mut d = MeasureDialog::new(vec![Handle::new(0x77)], &s, &saved_for(&s), None);
        let m = pick(&d, Action::Room(0));
        d.on_dialog_response(DialogResponse::Submitted(m));
        let mut m = HashMap::new();
        text(&mut m, "o1a", "b");
        text(&mut m, "o1b", "d");
        text(&mut m, "o1m", "5");
        d.on_dialog_response(DialogResponse::Submitted(m));
        assert!(d.saved.others.iter().any(|r| r[0] == "b" && r[3] == "5"));
        // Closing the hub ends the command.
        assert!(matches!(d.on_dialog_response(DialogResponse::Cancelled), CommandStep::Cancel));
    }
}
