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
