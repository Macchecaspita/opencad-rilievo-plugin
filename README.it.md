# opencad-rilievo-plugin — Room Survey per Open CAD Studio

[English](README.md) · **Italiano**

> **Progetto hobbistico, realizzato solo per divertimento e non adatto a un uso
> professionale.** Viene fornito così com'è, senza alcuna garanzia (vedi la licenza
> GPL-3.0): l'autore non si assume alcuna responsabilità per il suo uso né per i
> risultati ottenuti. Controlla sempre i risultati e fai una copia dei tuoi disegni.
> Versione iniziale (0.x): le funzioni potrebbero cambiare.

Plugin per [Open CAD Studio](https://github.com/HakanSeven12/OpenCADStudio) che
aggiunge la scheda **Rilievo** per il rilievo architettonico a misure dirette
(cordella o distanziometro):

1. disegni i vani **a occhio** come polilinee chiuse, con i normali comandi del CAD;
2. *(facoltativo)* **1. Perimetro esterno**: scegli il perimetro dell'edificio
   rilevato con la stazione totale, che resterà fermo;
3. *(facoltativo)* **2. Spessori muri esterni**: clicchi il perimetro lato per
   lato e per ognuno scrivi lo spessore e il verso (verso il centro o verso l'esterno) del muro: viene disegnata la linea parallela al lato;
4. **3. Vani: numeri, lettere e misure**: selezioni i vani e premi Invio; ogni
   vano riceve il suo numero e gli spigoli le lettere (A, B, C … — utili anche
   stampate come eidotipo) e si apre **una sola finestra** con tutti i vani, dal
   vano 1 all'ultimo, da scorrere con la barra a destra; il plugin propone le
   diagonali che servono (n − 3 per un vano di n vertici, scelte in modo che
   corrano dentro il vano, anche a L);
5. premi **Calcola**: i vani vengono **ricalcolati con la forma vera** e il disegno
   si aggiorna da solo — esatti con le misure minime, **compensati ai minimi
   quadrati** quando ci sono misure in più, con lo **scarto** di ogni misura e lo
   scarto quadratico medio σ₀, come una compensazione Pregeo.

Si possono rilevare anche **più vani collegati**: si cliccano uno dopo l'altro, i
vertici diventano 1A, 1B … 2A …, e i vani si legano tra loro con gli **spessori dei
muri** e le **misure attraverso le porte**; tutto viene compensato insieme. L'ultimo
calcolo si salva come **libretto delle misure** (CSV per Excel o Calc).

Al posto di qualche diagonale si possono dare **vincoli** (spigolo retto o con un
angolo noto, lato orizzontale o verticale). Si possono inserire **porte e
finestre** (lato, distanza dallo spigolo, larghezza): una **porta comune** scritta
nei due vani con lo spessore del muro basta a collegarli. Dentro
ogni vano è scritto solo il suo numero; le **superfici in m²** sono nella finestra
del risultato e nel libretto delle misure.

Con Calcola le polilinee dei vani prendono la forma calcolata (Ctrl+Z annulla).
Sul layer
**RILIEVO_QUOTE** vanno lettere, quote dei lati (stile di quota **RILIEVO**),
diagonali con la loro misura e una tabella *misurata / calcolata / scarto*, con in
rosso gli scarti oltre la tolleranza.

L'interfaccia e il manuale PDF sono nelle **stesse 23 lingue di Open CAD Studio**
(arabo, bulgaro, ceco, cinese semplificato e tradizionale, coreano, finlandese,
francese, giapponese, greco, hindi, inglese, italiano, lao, nederlandese,
polacco, portoghese, russo, spagnolo, tedesco, turco, ungherese, vietnamita) e
seguono la lingua del programma. `RV_LANG it|en|de|zh-CN|…|AUTO` forza una lingua.
I testi di riferimento sono italiano e inglese: le altre lingue sono traduzioni
automatiche, non riviste da madrelingua, quindi le correzioni sono benvenute.

## Comandi

| Comando | Pulsante | Cosa fa |
|---|---|---|
| `RV_PERIMETRO` | **1. Perimetro esterno** | facoltativo: clicca la polilinea del perimetro rilevato, Invio; resta ferma nel calcolo |
| `RV_SPESSORI` | **2. Spessori muri esterni** | clicca il perimetro lato per lato e scrivi spessore e verso del muro: nasce la linea parallela al lato (layer RILIEVO_PERIMETRO); verso l'esterno solo disegno, fuori dal calcolo dei vani |
| `RV_VANO` | **3. Vani: numeri, lettere e misure** | seleziona i vani, Invio: numeri e lettere sul disegno e una sola finestra con tutti i vani, porte e finestre, poi Calcola, che aggiorna il disegno |
| `RV_ETICHETTE` | | scrive solo le lettere sui vertici, per l'eidotipo |
| `RV_APPLICA` | | porta le polilinee originali nella forma dell'ultimo calcolo |
| `RV_FORO` | **Porte e finestre** | a disegno finito apre porte e finestre nei muri: tipo, larghezza, altezza, spigolo, tratto e distanza; interrompe le due facce del muro anche non parallele, svuota la campitura e mette la quota larghezza/altezza fuori dal perimetro |
| `RV_LIBRETTO` | **Libretto misure** | salva l'ultimo calcolo in CSV: misure, scarti, σ₀, coordinate, superfici |
| `RV_GUIDE` | **Guida** | breve guida |
| `RV_MANUALE` | **Manuale PDF** | manuale di rilievo di un appartamento, con disegni ([docs/Manuale_rilievo.pdf](docs/Manuale_rilievo.pdf); nelle altre lingue `docs/manual_<lingua>.pdf`) |
| `RV_LANG <codice>\|AUTO` | | lingua dei messaggi e del manuale (`it`, `en`, `de`, `zh-CN`, …) |

### Consigli di rilievo

- Un vano di **n** vertici ha **2n − 3** incognite: servono tutti i lati più
  **n − 3 diagonali** (1 per un rettangolo, 3 per una L).
- **Misura sempre almeno una diagonale in più**: senza misure in più la forma è
  esatta ma un errore (una cifra letta male, un vertice sbagliato) non si vede.
  Con una diagonale in più il plugin calcola gli scarti e segnala le misure sospette.
- Le misure vanno nelle unità del disegno (metri, centimetri o millimetri); la
  virgola decimale va bene. Altezza dei testi e tolleranza sono proposte in base
  alla grandezza del vano e si possono cambiare.
- **Vani collegati**: ogni vano oltre al primo vuole almeno **3 misure di
  collegamento** verso vertici diversi, per esempio lo spessore del muro (lato
  `1B-1C` e vertice `2A`) più due misure attraverso la porta (`1B`→`2D`, `1C`→`2A`).
  Con 4 o più misure anche il collegamento è controllato.
- **Vincoli**: un angolo retto (`C`, `90`) o un lato orizzontale (`A-B`,
  `orizzontale`) vale come una misura e può sostituire una diagonale; anche il
  vincolo riceve lo scarto, quindi un angolo "retto" che non lo è viene segnalato.
- **Porta comune**: scrivi la stessa porta nei due vani con lo stesso nome (es. `P1`)
  e lo spessore del muro; i due vani risultano collegati senza altre misure.
- **Misure mancanti** (mobili, ostacoli): lascia vuoto quello che non hai potuto
  misurare; con l'opzione *completa la forma con gli angoli del disegno* il plugin
  calcola lo stesso e segnala in rosso che quelle parti sono indicative.
- Nella finestra delle misure, la sezione **Come misurare** spiega ogni colonna.
- Se le misure non bastano o non stanno insieme, la finestra si riapre con quello
  che avevi scritto e il motivo in cima (per esempio quali diagonali aggiungere).

## Compilare

Serve Rust (https://rustup.rs). Dalla cartella del progetto:

```
cargo test --release
cargo build --release
```

Il risultato è `target/release/opencad_rilievo_plugin.dll` (Windows),
`libopencad_rilievo_plugin.so` (Linux) o `libopencad_rilievo_plugin.dylib` (macOS).

## Installare

Copia la libreria compilata **e** `plugin.toml` nella cartella

- Windows: `%APPDATA%\OpenCADStudio\plugins\opencad.rilievovani\`
- macOS: `~/Library/Application Support/OpenCADStudio/plugins/opencad.rilievovani/`
- Linux: `~/.config/OpenCADStudio/plugins/opencad.rilievovani/`

(su Windows, la DLL va rinominata `opencad.rilievovani-windows-x86_64.dll`, come
quella di Raster Tools) e riavvia Open CAD Studio: compare la scheda **Rilievo**.

Oppure, senza compilare nulla, dal **Plugin Manager** di Open CAD Studio
(comando `PLUGINS`): **Add repository →** `Macchecaspita/opencad-rilievo-plugin`,
scegli l'ultima versione e premi *Install*.

## Pubblicare una nuova versione

1. Scrivi le novità in [CHANGELOG.md](CHANGELOG.md), sotto **[Non rilasciato]**.
2. In PowerShell, nella cartella del progetto:

   ```
   powershell -ExecutionPolicy Bypass -File scripts\release.ps1 0.2.0
   ```

Lo script aggiorna la versione, compila e prova il plugin, poi crea commit e tag e
li carica su GitHub; il workflow `release.yml` compila per Windows, Linux e macOS
e pubblica la release.

## Compatibilità

Compilato contro `ocs_plugin_api` del tag **v2026.46** (Plugin API 23), come
Raster Tools 0.8.0. Per una versione successiva del CAD, aggiorna `rev` in
`Cargo.toml` con il commit del nuovo tag e `api_version` in `plugin.toml`.

## Struttura

```
src/lib.rs       manifest, punto d'ingresso del plugin
src/i18n.rs      scelta inglese / italiano
src/ribbon.rs    scheda Rilievo della barra multifunzione
src/commands.rs  comandi RV_*, layer, quote, tabella
src/tools.rs     selezione dei vani, finestra delle misure con aiuto, libretto, guida
src/solve.rs     calcolo puro: compensazione a rete (distanze e spessori), diagonali proposte (con test)
src/survey.rs    più vani: nomi dei vertici, lettura delle tabelle (con test)
scripts/release.ps1  pubblicazione di una nuova versione
icons/           icone SVG della scheda
```

Licenza GPL-3.0-only, come Open CAD Studio.
