# Registro delle modifiche

Tutte le modifiche rilevanti di Rilievo Vani sono annotate qui.
Formato: [Keep a Changelog](https://keepachangelog.com/it/1.1.0/).
Numerazione: [Semantic Versioning](https://semver.org/lang/it/) —
`0.x.y` indica una versione iniziale, ancora in fase di test.

Le note di ogni release su GitHub vengono prese automaticamente dalla sezione
della versione corrispondente. Scrivi le novità sotto **[Non rilasciato]**:
lo script `scripts/release.ps1` le sposta nella nuova versione al momento
del rilascio.

## [Non rilasciato]

## [0.6.3] - 2026-11-15

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Modificato
- **Nome del vano (e superficie) sempre dentro il vano**, con un margine libero
  verso i lati: il testo si sposta nella parte del vano dove sta (per esempio nel
  braccio più largo di una L), si rimpicciolisce quanto serve e, nei vani lunghi
  e stretti come i corridoi, si scrive in verticale.
- **Lettere vertici chiede l'altezza delle lettere**, con una proposta calcolata
  sui vani: si possono rimpicciolire dove gli spigoli sono fitti. L'altezza scelta
  vale anche per i calcoli successivi; rieseguendo il comando le lettere vengono
  riscritte.

## [0.6.2] - 2026-11-14

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Corretto
- Lettere illeggibili sulle piante con molti lati corti (riseghe, spallette,
  pilastri): l'altezza dei testi ora segue il lato tipico e non il più lungo, fino
  a 1/150 del lato maggiore.
- Sul disegno ogni spigolo porta solo la lettera (`B`), non più numero e lettera
  (`1B`): il numero è nel titolo del vano. Nelle finestre lo spigolo si scrive
  sempre `1B` (o solo `B` nella scheda del suo vano).

## [0.6.1] - 2026-11-14

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Modificato
- **Scelta dei vani con la selezione multipla del CAD**: in *Misure e diagonali* e
  *Lettere vertici* i vani si selezionano con un clic sulla linea oppure con una
  finestra trascinata sopra tanti vani insieme. Quelli selezionati sono evidenziati
  dal CAD e contornati in verde finché non si preme Invio, così si distinguono da
  quelli non ancora scelti; Maiusc+clic ne toglie uno.
- Numerazione: i vani presi uno alla volta seguono l'ordine dei clic; quelli presi
  insieme con una finestra sono numerati dall'alto in basso e da sinistra a destra.
  Gli oggetti che non sono polilinee chiuse vengono ignorati.

## [0.6.0] - 2026-11-14

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Modificato
- **Finestra delle misure rifatta, una scheda per vano.** *Misure e diagonali* apre
  un elenco dei vani con il loro stato (completo, con quante misure di controllo,
  oppure cosa manca) e dei collegamenti tra i vani. Si apre un vano alla volta: la
  sua scheda contiene solo i suoi lati, le sue diagonali e i suoi vincoli. A ogni
  OK l'elenco propone il vano successivo e, quando tutto è a posto, *Calcola*. La
  finestra resta piccola anche con molti vani.
- Nella scheda di un vano basta la lettera dello spigolo (`A`, `C`): il numero del
  vano viene aggiunto da solo.
- Le misure scritte restano memorizzate a ogni passaggio, anche chiudendo la
  finestra: rilanciando il comando sugli stessi vani si riparte da dove si era.
- Opzioni (testi, tolleranza, unità, quote, libretto…) in una scheda a parte.

### Aggiunto
- **Porte e finestre con un clic sul disegno**: dall'elenco, *Aggiungi porta o
  finestra*, poi un clic sul muro nel punto dove si trova. Tra due vani il plugin
  riconosce da solo i due lati (evidenziati in verde) e chiede solo la distanza
  dallo spigolo allo stipite nei due vani, la larghezza e lo **spessore del muro
  misurato nella porta**; la porta collega i due vani. Lo spigolo da cui si misura
  è quello più vicino al clic, e si può cambiare.
- Se della porta si conosce solo lo spessore del muro (senza la distanza nel
  secondo vano), i due vani vengono legati dallo spessore e la posizione lungo il
  muro è presa dal disegno.
- *Modifica o elimina porte e finestre* e *Altre misure tra vani (senza porta)*
  come voci dell'elenco.
- Manuale PDF e guida aggiornati al nuovo modo di lavorare.

## [0.5.1] - 2026-11-13

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Corretto
- Testi illeggibili quando il disegno a occhio è a una scala diversa dalle misure:
  altezza dei testi e tolleranza ora si adattano ai vani **calcolati** (e il
  plugin lo dice sulla riga di comando).
- I testi non vanno più a capo a ogni parola (la casella di testo era larga 10
  unità per impostazione predefinita).
- Tabella delle misure con colonne larghe quanto i numeri, per i caratteri larghi
  del CAD; lettere degli spigoli più distanziate.
- Niente quote sui lati cortissimi (riseghe di pochi centimetri) e niente misura
  scritta sulle diagonali corte: i valori sono comunque nella tabella.
- Con il calcolo completato dal disegno non compare più anche "Nessuna misura di
  controllo".

## [0.5.0] - 2026-11-13

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Aggiunto
- **Applica al disegno** (pulsante nel gruppo Vano, comando `RV_APPLICA`): dopo
  aver controllato il risultato, porta i vertici delle polilinee originali nella
  posizione calcolata. Restano le stesse polilinee (stesso layer, stesse
  proprietà); la copia su RILIEVO_CORRETTO viene tolta. Un solo passo di
  annullamento (Ctrl+Z).
- Nella pagina 2 della finestra, l'opzione *Modifica subito le polilinee
  originali* fa lo stesso a ogni calcolo, senza passare dalla copia.

## [0.4.0] - 2026-11-13

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Aggiunto
- **Calcolo anche con misure incomplete**: quando mobili o ostacoli impediscono
  alcune diagonali (o manca un collegamento), con l'opzione *completa la forma con
  gli angoli del disegno* (pagina 2, attiva di solito) il plugin prende dal disegno
  quello che manca — gli angoli degli spigoli e, tra vani già legati da qualche
  misura, la posizione relativa — con un peso basso, così dove ci sono misure vere
  vincono loro. Il risultato viene segnalato in rosso nella tabella, sulla riga di
  comando e nel libretto: le parti non misurate sono solo indicative.
- **Manuale di rilievo in PDF** (pulsante *Manuale PDF*, comando `RV_MANUALE`),
  incorporato nel plugin: come rilevare un appartamento intero, con disegni —
  eidotipo e numerazione dei vani, lati e diagonali, misura di controllo, mobili
  che ostacolano, porte comuni e spessori, porte e finestre, muri esterni, lettura
  degli scarti, valori indicativi e lista di controllo per il cantiere. Si trova
  anche in `docs/Manuale_rilievo.pdf`.

## [0.3.2] - 2026-11-13

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Modificato
- Le misure si scrivono in **caselle** invece che in tabelle: la finestra si apre
  già alta quanto serve e non va più allargata con il mouse. I lati sono su una
  griglia di 4 per riga, con la lunghezza disegnata tra parentesi; diagonali,
  collegamenti e spessori su righe *Da – A – Misurata*, con alcune righe vuote per
  le misure in più. Lo stesso per vincoli, porte e finestre nella pagina 2. Le
  righe già compilate restano quando la finestra si riapre, con nuove righe vuote
  dopo.

## [0.3.1] - 2026-11-13

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Corretto
- Nella 0.3.0 la finestra delle misure era diventata troppo alta: Open CAD Studio
  la tagliava e la tabella dei lati compariva vuota. Ora la finestra è divisa in
  due pagine: **1/2 lati e diagonali**, poi con *Avanti* **2/2 vincoli, porte e
  finestre e opzioni**, da cui *Calcola* oppure *Indietro* (le misure scritte
  restano).

## [0.3.0] - 2026-11-13

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Aggiunto
- **Vincoli** (nuova tabella nella finestra delle misure): uno spigolo con il suo
  angolo (`C`, `90`; per uno spigolo rientrante vale anche `270`) oppure un lato
  orizzontale o verticale (`A-B`, `orizzontale`). Ogni vincolo vale una misura, può
  prendere il posto di una diagonale e riceve il suo scarto, così un angolo creduto
  retto che non lo è viene segnalato. Un lato orizzontale o verticale fissa anche la
  rotazione del vano sul foglio.
- **Porte e finestre** (nuova tabella): lato, tipo (porta, finestra, apertura),
  distanza dal primo spigolo del lato e larghezza. Vengono disegnate sul layer
  RILIEVO_APERTURE: stipiti, anta con l'apertura per le porte, doppia linea per le
  finestre, nome e larghezza. Scrivendo il lato al contrario (`C-B` invece di `B-C`)
  la distanza si misura dall'altro spigolo e la porta si apre dall'altra parte.
- **Porta comune tra due vani**: la stessa porta scritta nei due vani con lo stesso
  nome e lo spessore del muro collega da sola i due vani (stipiti affacciati, muro
  parallelo); non servono altre misure di collegamento. Il muro viene disegnato con
  le spallette.
- **Superficie dei vani sul disegno**, in m², sotto il nome del vano (o al centro se
  il vano è uno solo). Nuova scelta *Unità del disegno* (metri, centimetri,
  millimetri), proposta in base alla grandezza del disegno.
- Libretto delle misure: angoli in gradi, superfici in m², elenco di porte e
  finestre.
- Aiuto: nella sezione *Come misurare* spiegazioni su vincoli, porte e finestre e
  porta comune; la guida è aggiornata.

## [0.2.1] - 2026-11-13

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Corretto
- Le lettere degli spigoli non si sovrappongono più quando gli spigoli sono vicini
  (riseghe, lati corti, smussi): ogni lettera cerca un posto libero dalle altre
  lettere, dalle linee del vano e dagli altri spigoli, e quando deve spostarsi è
  collegata al suo spigolo con una linea di richiamo. Vale sia per le lettere di
  lavoro sia per quelle del vano ricalcolato.
- Rieseguendo *Lettere vertici* le lettere vecchie (e i richiami) vengono
  sostituite invece di aggiungersi.

## [0.2.0] - 2026-11-13

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Versione di prova.

### Aggiunto
- **Vani collegati**: in *Misure e diagonali* si possono cliccare più vani uno dopo
  l'altro (l'ordine dà il numero del vano). I vertici si chiamano 1A, 1B … 2A …;
  tutti i vani vengono ricalcolati insieme in un'unica compensazione ai minimi
  quadrati. Ogni vano collegato vuole almeno 3 misure di collegamento; i vani non
  collegati vengono calcolati ciascuno per conto suo. Vani disegnati a contatto
  (senza muro) condividono i vertici.
- **Spessori dei muri**: nella tabella *Diagonali, collegamenti e spessori* un lato
  (es. `1B-1C`) e un vertice dell'altro vano (es. `2A`) danno la distanza del vertice
  dalla parete. Anche le **misure attraverso le porte** (es. `1B` → `2D`) si scrivono
  lì. Nel disegno compaiono come linee con la loro misura.
- **Libretto delle misure** (`RV_LIBRETTO`, pulsante *Libretto misure*): salva
  l'ultimo calcolo in un file CSV per Excel o LibreOffice Calc, con misure,
  calcolate, scarti, σ₀, coordinate dei vertici, superfici e perimetri dei vani. Si
  può anche spuntare *Salva anche il libretto* nella finestra delle misure.
- **Aiuto in ogni finestra**: la finestra delle misure ha una sezione apribile
  *Come misurare* (lati, diagonali, controllo, vani collegati, spessori, unità) e
  una spiegazione sopra ogni tabella e opzione; i messaggi della riga di comando
  guidano passo passo nella scelta dei vani; gli errori dicono cosa correggere e la
  finestra si riapre con le misure già scritte.

### Modificato
- La guida (`RV_GUIDE`) comprende vani collegati e libretto, con l'avvertenza
  sull'uso hobbistico.

## [0.1.0] - 2026-11-13

> **Progetto hobbistico, realizzato per divertimento e non adatto a un uso
> professionale.** Fornito così com'è, senza garanzie: l'autore non si assume alcuna
> responsabilità per il suo uso né per i risultati. Prima versione di prova.

### Aggiunto
- **Misure e diagonali** (`RV_VANO`): si seleziona un vano disegnato a occhio come
  polilinea chiusa, si inseriscono lati e diagonali misurati e il vano viene
  ricalcolato con la forma vera, compensando ai minimi quadrati le misure in più,
  con scarti e σ₀.
- Diagonali proposte per ogni vano (n − 3, tutte interne, anche per vani a L).
- Risultato sul layer RILIEVO_CORRETTO; lettere, quote dei lati (stile RILIEVO),
  diagonali e tabella misure/scarti sul layer RILIEVO_QUOTE.
- **Lettere vertici** (`RV_ETICHETTE`) per preparare l'eidotipo.
- Guida (`RV_GUIDE`), interfaccia in italiano e inglese (`RV_LANG`).
