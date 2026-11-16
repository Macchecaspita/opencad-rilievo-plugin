"""Testi del manuale — italiano."""

OUT = "Manuale_rilievo.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — manuale di rilievo",
    "foot": "Room Survey — manuale di rilievo · progetto hobbistico, non per uso professionale",
    "subtitle": "Come rilevare un appartamento a misure dirette<br/>e ricostruirlo in Open CAD Studio",
    "warn": "<b>Avvertenza.</b> Room Survey è un progetto hobbistico, realizzato per divertimento e <b>non adatto "
            "a un uso professionale</b>. È fornito così com'è, senza alcuna garanzia; l'autore non si assume alcuna "
            "responsabilità per il suo uso né per i risultati. Controlla sempre i risultati con misure di verifica.",

    # disegni
    "d_room": "Vano {r}",
    "d_sketch": "eidotipo: vani numerati, spigoli con lettere (scritte dal pulsante 3. Vani)",
    "d_check": "B–D controllo",
    "d_out1": "C–E: passa fuori,",
    "d_out2": "non misurabile",
    "d_from1b": "distanza da 1B",
    "d_width": "0,90 larghezza",
    "d_thick": "spessore 0,30 (nello stipite)",
    "d_from2d": "1,10 distanza da 2D",
    "d_furn": "mobile",
    "d_blocked": "A–C bloccata",

    "h1": "1. Il metodo in breve",
    "p1": "Si disegna la pianta <b>a occhio</b>, si misurano in cantiere i lati e alcune diagonali di ogni vano, "
          "più qualche misura che lega i vani tra loro, e il plugin ricostruisce la pianta <b>vera</b> compensando "
          "le misure ai minimi quadrati. Gli scarti dicono subito se una misura è sbagliata.",
    "t1": [["", "Dove", "Cosa si fa"],
           ["1", "in studio", "eidotipo: si disegna ogni vano come polilinea chiusa (a occhio) e si scrivono le lettere"],
           ["2", "in cantiere", "si misurano lati, diagonali, spessori dei muri, porte e finestre, segnandoli sull'eidotipo"],
           ["3", "in studio", "si inseriscono le misure, si calcola, si leggono gli scarti, si completa il disegno"]],
    "h_tools": "Attrezzatura",
    "tools": [
        "distanziometro laser (meglio con misura dal retro dello strumento) e una cordella metrica da 5 m per le "
        "misure corte (spessori, mazzette, riseghe);",
        "l'eidotipo stampato, matita e gomma; una tavoletta rigida;",
        "un metro rigido o una squadra per verificare gli angoli quando le diagonali sono impedite;",
        "macchina fotografica o telefono: una foto per vano, con le lettere degli spigoli scritte a matita sull'eidotipo.",
    ],

    "h2": "2. Prima del sopralluogo: l'eidotipo nel CAD",
    "p2a": "Disegna <b>ogni vano</b> come <b>polilinea chiusa</b> (PLINEA … Chiudi), seguendo il <b>filo interno</b> "
           "dei muri. Le misure non contano: conta che ogni spigolo vero sia un vertice della polilinea, nell'ordine "
           "giusto. Riseghe, nicchie, pilastri sporgenti e smussi sono vertici anche loro (vano 3 dell'esempio). "
           "Il corridoio è un vano come gli altri.",
    "p2b": "Premi <b>3. Vani: numeri, lettere e misure</b> e seleziona i vani — uno alla volta nell'ordine che vuoi dare ai numeri, oppure tutti insieme con una finestra (numerati dall'alto in basso, da sinistra a destra) — e premi Invio: in ogni vano compare il suo numero in grassetto e sugli spigoli le lettere A, B, C …; lo spigolo B del vano 2 si chiama 2B. Si apre anche la finestra delle misure: per ora chiudila con <b>Chiudi</b> e rispondi di <b>tenere</b> le lettere. Stampa la pianta: è il tuo eidotipo.",
    "t2": [["Consiglio", "Perché"],
           ["Numera i vani in un ordine che segui anche in cantiere", "Si scrive più in fretta e si sbaglia meno vano."],
           ["Polilinea sul filo interno", "Le misure si prendono da dentro i vani; i muri esterni si ricavano dopo dagli spessori."],
           ["Nessun vertice inutile", "Un vertice su un lato dritto (angolo di 180°) chiede misure inutili."]],

    "h3": "3. In cantiere: misurare un vano",
    "p3a": "Un vano di <b>n</b> spigoli ha 2n − 3 incognite: si chiude con <b>tutti i lati</b> più <b>n − 3 "
           "diagonali</b>. Un rettangolo vuole 1 diagonale, un vano a L 3, il bagno con risega dell'esempio 3.",
    "p3b": "Misura i lati <b>alla stessa altezza</b> (per esempio a 1 m da terra, sopra battiscopa e mobili bassi), "
           "da spigolo a spigolo. Poi le diagonali. Le diagonali proposte dal plugin passano tutte <b>dentro</b> il vano: "
           "in un vano a L non c'è mai una diagonale che attraversa il muro.",
    "cap3": "Sinistra: lati (blu) e diagonale (arancio) di un vano fuori squadra; la seconda diagonale (verde) è la misura "
            "di controllo. Destra: vano a L, le tre diagonali utili e una da evitare.",
    "h_check": "La misura di controllo",
    "p_check": "Prendi <b>sempre una diagonale in più</b> del minimo. Senza misure in più la forma è esatta qualunque cosa "
               "tu scriva, e una cifra letta male (6,85 al posto di 6,58) passa inosservata. Con una misura in più il "
               "plugin calcola gli <b>scarti</b> e σ₀ e colora in rosso le misure oltre la tolleranza (di solito 1–2 cm).",
    "h_obst": "Quando un mobile impedisce una diagonale",
    "obst": [
        "Prova prima una diagonale diversa (B–D al posto di A–C), o alzala sopra il mobile tenendo il "
        "distanziometro in bolla.",
        "Se non si può, usa un <b>vincolo</b> (nella scheda del vano): spigolo e angolo, per esempio "
        "<b>A, 90</b>, dopo averlo verificato con la squadra; oppure un lato <b>orizzontale</b> o <b>verticale</b>. "
        "Ogni vincolo vale una misura e riceve anche lui uno scarto.",
        "Se manca ancora qualcosa, l'opzione <b>completa la forma con gli angoli del disegno</b> (attiva di "
        "solito) prende dal disegno quello che manca, e la tabella lo segnala in rosso. In quel punto il "
        "risultato vale quanto il tuo disegno a occhio: annotalo.",
    ],

    "h4": "4. Collegare i vani tra loro",
    "p4a": "Ogni vano, misurato da solo, ha la forma giusta ma non sa dove si trova rispetto agli altri. Per "
           "collegarlo servono <b>almeno 3 misure</b> verso il vano accanto, oppure una <b>porta comune</b>. "
           "Il modo più comodo è la porta.",
    "cap4": "Una porta tra il vano 1 e il vano 2: distanza dallo spigolo e larghezza misurate da ciascuna parte, "
            "spessore del muro misurato nello stipite.",
    "p4b": "Nella finestra delle misure, sotto i vani, premi <b>＋ Aggiungi porta o finestra</b> e <b>clicca sul disegno</b>, sul muro tra i due vani, vicino alla porta. Il plugin riconosce da solo i due lati (li evidenzia in verde) e chiede:",
    "t4": [["Casella", "Esempio"],
           ["Distanza dallo spigolo 1B allo stipite (misurata nel vano 1)", "1,00"],
           ["Larghezza", "0,90"],
           ["Spessore del muro, misurato nella porta", "0,30"],
           ["Distanza dallo spigolo 2D allo stipite (misurata nel vano 2)", "1,10"]],
    "p4c": "La distanza si prende dallo spigolo fino allo stipite; il plugin propone lo spigolo più vicino al punto "
           "cliccato e puoi sceglierne l'altro. Basta questo: i due vani "
           "risultano collegati (stipiti affacciati, muro parallelo, spessore noto). Una misura in più attraverso la "
           "porta (per esempio da 1B a 2D) fa da controllo.",
    "p4d": "Senza porta (o per controllo) usa la sezione <b>Altre misure tra vani</b>, in fondo alla finestra:",
    "links": [
        "<b>spessore di un muro</b>: Da = un lato (1B-1C), A = lo spigolo dell'altro vano dietro quel muro (2A), "
        "Misurata = lo spessore preso in squadra nella porta;",
        "<b>misure attraverso la porta</b>: Da = 1B, A = 2D, tra spigoli dei due vani che si vedono dalla porta; "
        "prendine due verso spigoli diversi.",
    ],
    "h_order": "Ordine consigliato per l'appartamento d'esempio",
    "p_order": "Parti dal <b>corridoio</b> (vano 4), che tocca quasi tutti gli altri, e collega ogni vano al corridoio con "
               "la sua porta: soggiorno e camera a sinistra, cucina e camera 2 a destra; il bagno si collega alla camera. "
               "In questo modo ogni vano è legato da una catena corta e un errore resta circoscritto.",

    "h_fix": "Perimetro fisso rilevato con la stazione totale",
    "p_fix": "Se il perimetro esterno dell'edificio è stato rilevato con la stazione totale (o comunque è esatto), il plugin lo usa come riferimento fisso: non viene mai spostato, e i vani si posizionano da soli al suo interno.",
    "fix": ["disegna il perimetro come <b>polilinea chiusa</b>, in coordinate vere, e i vani a occhio al suo interno;", "premi <b>1. Perimetro esterno</b>, clicca la polilinea e premi Invio (Invio senza clic = nessun perimetro);", "premi <b>2. Spessori muri esterni</b> e clicca i lati del perimetro uno alla volta: per ognuno scrivi lo spessore del muro verso l'interno (misurato nelle finestre). I lati con lo spessore diventano verdi; gli spessori restano scritti sul disegno, sul layer RILIEVO_PERIMETRO. Premi Invio per finire;", "nel calcolo i lati dei vani lungo il perimetro restano paralleli a esso, a quella distanza: un vano d'angolo con due lati sul perimetro non ha bisogno di diagonali per la posizione; un vano con un solo lato sul perimetro va ancora collegato agli altri (per esempio con una porta)."],
    "h_hatch": "Muri pieni tra vani: il retino",
    "p_hatch": "Un muro pieno non si può attraversare per misurare, ma il suo spessore si misura nelle porte. Se nel disegno metti un <b>retino</b> (tratteggio) nei muri tra i vani, il plugin li riconosce da solo:",
    "hatch": ["trova i due lati di vani diversi che si fronteggiano con il retino in mezzo e li propone nella sezione <b>Muri pieni tra vani</b> della finestra delle misure;", "scrivi lo <b>spessore misurato in una porta</b>; se nello stesso muro c'è già una porta comune con lo spessore, la casella può restare vuota;", "nel calcolo i due lati restano paralleli a quella distanza, con il loro scarto in tabella."],
    "p_hatch2": "Un muro pieno fissa la distanza e la rotazione tra due vani, ma non lo scorrimento lungo il muro: serve ancora una porta nello stesso muro, un secondo muro non parallelo verso lo stesso vano, oppure il perimetro fisso.",

    "h5": "5. Porte, finestre e muri esterni",
    "h_dw": "Porte e finestre",
    "p_dw": "Anche finestre e porte verso l'esterno si aggiungono con <b>Aggiungi porta o finestra</b> e un clic sul muro: "
            "tipo (<b>porta</b>, <b>finestra</b>, <b>apertura</b>), distanza dallo spigolo fino alla spalla e larghezza "
            "della luce. La porta è incernierata dalla parte dello spigolo da cui misuri. Le altezze (davanzale, "
            "architrave) annotale sull'eidotipo: il plugin lavora in pianta.",
    "h_ext": "Muri esterni",
    "p_ext": "Il plugin ricostruisce i vani sul filo interno. Per il perimetro esterno misura lo <b>spessore del muro "
             "nelle finestre</b> (mazzetta compresa) e almeno uno per facciata; poi nel CAD ricava il filo esterno "
             "con OFFSET dai lati dei vani, alla distanza misurata, e raccorda gli spigoli. Dove possibile verifica "
             "con una misura esterna (la lunghezza di una facciata) presa da fuori.",

    "h6": "6. In studio: calcolo e controllo",
    "calc": ["Premi <b>3. Vani: numeri, lettere e misure</b>, seleziona tutti i vani (stesso ordine dell'eidotipo) e premi Invio. Si apre <b>una sola finestra</b> con tutti i vani uno sotto l'altro, dal vano 1 all'ultimo, ciascuno con il suo titolo e il suo stato: scorrila con la barra a destra e scrivi lati e diagonali. Sotto i vani ci sono porte e finestre, muri pieni, altre misure tra vani e opzioni.", "Premi <b>▶ Calcola</b>: le polilinee dei vani vengono portate nella forma calcolata e <b>il disegno si aggiorna da solo</b> (Ctrl+Z annulla). Se le misure non bastano o non tornano, la finestra si riapre con il motivo e con le misure già scritte.", "Quote, diagonali e tabella vanno su <b>RILIEVO_QUOTE</b>; porte e finestre su <b>RILIEVO_APERTURE</b>. Una finestra <b>Risultato del calcolo</b> riassume superfici, misure in più e σ₀.", "Leggi la tabella degli scarti: uno scarto rosso indica la misura da ricontrollare. Con una sola misura in più l'errore si spalma su più misure: prenderne un'altra aiuta a trovarlo.", "<b>Libretto misure</b> salva tutto in un CSV per Excel o Calc (misure, scarti, coordinate, superfici)."],
    "h_vals": "Valori indicativi",
    "t6": [["Grandezza", "Valore tipico", "Se è più alto"],
           ["scarto di un lato o una diagonale", "entro 1 cm", "rimisura; controlla cifre invertite e spigolo giusto"],
           ["σ₀ di un vano", "0,3–0,8 cm", "misure prese ad altezze diverse, spigoli non netti"],
           ["angolo vincolato a 90°", "scarto entro 0,2°", "l'angolo non è retto: togli il vincolo e misura una diagonale"]],
    "h_list": "Lista di controllo prima di lasciare il cantiere",
    "list": [
        "ogni vano ha tutti i lati e almeno n − 3 diagonali (o vincoli verificati), più una misura di controllo;",
        "ogni vano è collegato: porta comune con spessore, oppure 3 misure verso un vano già collegato;",
        "spessori dei muri esterni presi nelle finestre, almeno uno per facciata;",
        "porte e finestre: distanza dallo spigolo, larghezza, altezze annotate;",
        "una foto per vano.",
    ],
    "ex_h": "7. Esempio completo, passo per passo",
    "ex_p": "Un appartamento di sette vani, in un edificio con la facciata sinistra inclinata e un'ala con l'angolo smussato. Il perimetro esterno è stato rilevato con la stazione totale; i muri interni sono pieni e il loro spessore si misura solo nelle porte.",
    "ex_fig": "perimetro E1–E7 rilevato, vani 1–7 disegnati a occhio, muri pieni con il retino",
    "ex_cap": "Il disegno di partenza: perimetro fisso E1–E7, vani numerati con le lettere degli spigoli, muri campiti con il retino, porte P1–P6 e finestre (in blu).",
    "ex_ties": [["Vano", "Lati sul perimetro", "Muri pieni (retino)", "Porte"], ["1", "C–D, D–A", "A–B con 4; B–C con 2", "P5"], ["2", "C–D", "D–A con 1; A–B con 5; B–C con 3", "P2"], ["3", "B–C, C–D, D–E", "A–B con 6; E–A con 2", "P6"], ["4", "A–B, D–A", "C–D con 1; B–C con 5", "P1, P5"], ["5", "A–B", "D–A con 4; C–D con 2; B–C con 6 e 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A con 5; A–B con 7; C–D con 3", "P3, P6"], ["7", "A–B, B–C", "C–D con 6; D–A con 5", "P4"]],
    "ex_ties_note": "Sei vani su sette toccano il perimetro con due lati non paralleli: il perimetro da solo li mette al loro posto. Il vano 2 ha un solo lato sul perimetro, ma il muro pieno verso il vano 1 (non parallelo) lo blocca. Muri pieni e porte fanno così anche da controllo.",
    "ex_steps": [["Il perimetro dalla stazione totale", "Rileva con la stazione totale gli spigoli esterni dell'edificio E1…E7 e uniscili in una polilinea chiusa, in coordinate vere. È l'unico elemento esatto del disegno: il plugin non lo sposterà mai."], ["I vani a occhio", "Dentro il perimetro disegna ogni vano come polilinea chiusa sul filo interno dei muri. Contano solo il numero e l'ordine degli spigoli: il vano 3 ha cinque spigoli perché il muro esterno è smussato, i vani 1 e 4 seguono la facciata inclinata."], ["Il retino nei muri pieni", "Con il comando di campitura (HATCH) riempi lo spazio tra il perimetro e i vani, cioè tutti i muri. Il retino dice al plugin quali vani sono separati da un muro pieno: non serve disegnarlo preciso, basta che stia tra i due lati."], ["Le lettere", "Premi <b>3. Vani: numeri, lettere e misure</b>, clicca i vani nell'ordine 1…7 della figura e premi Invio. Numeri e lettere compaiono subito e si apre la finestra delle misure: chiudila e scegli di tenere le lettere. Stampa il disegno: è l'eidotipo da portare in cantiere."], ["In cantiere: i vani", "In ogni vano misura i lati alla stessa altezza e almeno una diagonale. Ai vani con due lati sul perimetro le diagonali non servono per la forma, ma una diagonale in più è l'unico modo per accorgersi di una cifra letta male. Per il vano 1, per esempio:"], ["In cantiere: spessori e porte", "Misura lo spessore dei muri esterni nelle finestre (qui 0,40) e quello dei muri interni nelle porte (qui 0,30). Per ogni porta annota, da tutte e due le parti, la distanza dallo spigolo allo stipite e la larghezza:"], ["Pulsanti 1 e 2: perimetro e spessori", "In studio premi <b>1. Perimetro esterno</b>, clicca la polilinea E1…E7 e premi Invio. Poi premi <b>2. Spessori muri esterni</b>, clicca il primo lato, scrivi 0,40 e spunta la casella per usarlo su tutti i lati: diventano tutti verdi. Invio per finire."], ["Pulsante 3: i vani e la finestra delle misure", "Premi <b>3. Vani: numeri, lettere e misure</b>, clicca i vani nello stesso ordine 1…7 e premi Invio. Si apre la finestra con i vani dal 1 al 7, uno sotto l'altro; più in basso la sezione <b>Muri pieni tra vani (dal retino)</b> con le righe già proposte: scrivi 0,30 dove non c'è una porta comune, dove c'è lo prende dalla porta."], ["Lati, diagonali e porte", "Scorri la finestra e scrivi lati e diagonali di ogni vano. Poi premi <b>＋ Aggiungi porta o finestra</b> e clicca sul muro di ogni porta P1…P6: il plugin trova i due lati e chiede distanze, larghezza e spessore, poi torna alla finestra. Le finestre si aggiungono allo stesso modo, sul muro esterno."], ["Calcola", "Quando ogni vano mostra ✔ e i vani risultano collegati, premi <b>▶ Calcola</b>: le polilinee dei vani prendono la forma calcolata e il disegno si aggiorna da solo. La finestra <b>Risultato del calcolo</b> riassume misure in più e σ₀; in tabella gli scarti in rosso indicano le misure da ricontrollare. <b>Libretto misure</b> salva tutto in un CSV."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Misura (vano 1)", "Valore", "Nota"], ["1A–1B", "4,10", "lato"], ["1B–1C", "8,30", "lato"], ["1C–1D", "4,58", "lato sul perimetro"], ["1D–1A", "8,31", "lato sul perimetro (inclinato)"], ["1A–1C", "9,26", "diagonale di controllo"], ["1B–1D", "9,48", "seconda diagonale (facoltativa)"]],
    "ex_doors": [["Porta", "Primo vano: lato, distanza", "Secondo vano: lato, distanza", "Larghezza", "Spessore"], ["P1", "4B–4C, da 4B 1,60", "5D–5A, da 5A 1,60", "0,90", "0,30"], ["P2", "5C–5D, da 5D 1,30", "2A–2B, da 2A 1,30", "0,90", "0,30"], ["P3", "5B–5C, da 5B 5,00", "6D–6A, da 6A 1,10", "0,90", "0,30"], ["P4", "5B–5C, da 5B 1,00", "7D–7A, da 7A 1,00", "0,90", "0,30"], ["P5", "4C–4D, da 4C 1,10", "1A–1B, da 1B 1,10", "0,90", "0,30"], ["P6", "6C–6D, da 6D 1,30", "3A–3B, da 3A 1,30", "0,80", "0,30"]],
    "ex_end": "Con il perimetro rilevato una volta sola, nessuna misura attraversa i muri: lati e diagonali si prendono dentro i vani, gli spessori nelle porte e nelle finestre.",
    "end": "Room Survey è un progetto hobbistico, non adatto a un uso professionale: l'autore non si assume "
           "responsabilità per il suo uso. Codice e aggiornamenti: github.com/Macchecaspita/opencad-rilievo-plugin",
}
