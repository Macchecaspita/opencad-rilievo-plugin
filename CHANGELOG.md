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

## [0.1.0] - 2026-11-13

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
