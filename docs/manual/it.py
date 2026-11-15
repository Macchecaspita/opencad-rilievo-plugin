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
    "d_sketch": "eidotipo: vani numerati, spigoli con lettere (scritte da Lettere vertici)",
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
    "p2b": "Con <b>Lettere vertici</b> seleziona i vani — uno alla volta nell'ordine che vuoi dare ai numeri, "
           "oppure tutti insieme con una finestra (numerati dall'alto in basso, da sinistra a destra) — e premi Invio: "
           "sugli spigoli compaiono le lettere A, B, C … e in ogni vano il suo numero e la superficie; lo spigolo B "
           "del vano 2 si chiama 2B. Una finestrella chiede l'altezza delle lettere: rimpiccioliscile dove gli spigoli "
           "sono fitti. Stampa la pianta: è il tuo eidotipo.",
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
    "p4b": "Dall'elenco della finestra scegli <b>Aggiungi porta o finestra</b> e <b>clicca sul disegno</b>, sul muro tra i "
           "due vani, vicino alla porta. Il plugin riconosce da solo i due lati (li evidenzia in verde) e chiede:",
    "t4": [["Casella", "Esempio"],
           ["Distanza dallo spigolo 1B allo stipite (misurata nel vano 1)", "1,00"],
           ["Larghezza", "0,90"],
           ["Spessore del muro, misurato nella porta", "0,30"],
           ["Distanza dallo spigolo 2D allo stipite (misurata nel vano 2)", "1,10"]],
    "p4c": "La distanza si prende dallo spigolo fino allo stipite; il plugin propone lo spigolo più vicino al punto "
           "cliccato e puoi sceglierne l'altro. Basta questo: i due vani "
           "risultano collegati (stipiti affacciati, muro parallelo, spessore noto). Una misura in più attraverso la "
           "porta (per esempio da 1B a 2D) fa da controllo.",
    "p4d": "Senza porta (o per controllo) scegli dall'elenco <b>Altre misure tra vani</b>:",
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
    "fix": ["disegna il perimetro come <b>polilinea chiusa</b>, in coordinate vere, e i vani a occhio al suo interno;", "dopo aver scelto i vani in <b>Misure e diagonali</b>, la riga di comando chiede il <b>perimetro fisso</b> (facoltativo): cliccalo, oppure premi Invio per continuare senza. Si può aggiungere anche dopo, dall'elenco della finestra;", "nella pagina <b>Muri esterni</b> scrivi lo spessore dei muri esterni (misurato nelle finestre) e controlla i lati dei vani proposti lungo il perimetro: ognuno resta parallelo al perimetro, a quella distanza;", "un vano d'angolo con due lati sul perimetro non ha bisogno di diagonali per la posizione; un vano con un solo lato sul perimetro va ancora collegato agli altri (per esempio con una porta)."],
    "h_hatch": "Muri pieni tra vani: il retino",
    "p_hatch": "Un muro pieno non si può attraversare per misurare, ma il suo spessore si misura nelle porte. Se nel disegno metti un <b>retino</b> (tratteggio) nei muri tra i vani, il plugin li riconosce da solo:",
    "hatch": ["trova i due lati di vani diversi che si fronteggiano con il retino in mezzo e li propone nella pagina <b>Muri pieni tra vani</b>;", "scrivi lo <b>spessore misurato in una porta</b>; se nello stesso muro c'è già una porta comune con lo spessore, la casella può restare vuota;", "nel calcolo i due lati restano paralleli a quella distanza, con il loro scarto in tabella."],
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
    "calc": [
        "<b>Misure e diagonali</b>: clicca tutti i vani (stesso ordine dell'eidotipo) e premi Invio. Si apre l'elenco dei "
        "vani con il loro stato: apri un vano alla volta e scrivi lati e diagonali, aggiungi le porte con un clic sul "
        "disegno; quando tutto è completo l'elenco propone <b>Calcola</b>.",
        "Se le misure non bastano o non tornano, la finestra si riapre con il motivo e con le misure già scritte.",
        "Il risultato va sul layer <b>RILIEVO_CORRETTO</b>, con la superficie di ogni vano; quote, diagonali e tabella su "
        "<b>RILIEVO_QUOTE</b>; porte e finestre su <b>RILIEVO_APERTURE</b>. Il disegno a occhio resta per confronto. "
        "<b>Applica al disegno</b> porta poi le polilinee originali nella forma calcolata.",
        "Leggi la tabella degli scarti: uno scarto rosso indica la misura da ricontrollare. Con una sola misura in più "
        "l'errore si spalma su più misure: prenderne un'altra aiuta a trovarlo.",
        "<b>Libretto misure</b> salva tutto in un CSV per Excel o Calc (misure, scarti, coordinate, superfici).",
    ],
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
    "end": "Room Survey è un progetto hobbistico, non adatto a un uso professionale: l'autore non si assume "
           "responsabilità per il suo uso. Codice e aggiornamenti: github.com/Macchecaspita/opencad-rilievo-plugin",
}
