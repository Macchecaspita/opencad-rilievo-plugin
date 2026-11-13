# opencad-rilievo-plugin — Rilievo Vani per Open CAD Studio

[English](README.md) · **Italiano**

> **Progetto hobbistico, realizzato solo per divertimento e non adatto a un uso
> professionale.** Viene fornito così com'è, senza alcuna garanzia (vedi la licenza
> GPL-3.0): l'autore non si assume alcuna responsabilità per il suo uso né per i
> risultati ottenuti. Controlla sempre i risultati e fai una copia dei tuoi disegni.
> Versione iniziale (0.x): le funzioni potrebbero cambiare.

Plugin per [Open CAD Studio](https://github.com/HakanSeven12/OpenCADStudio) che
aggiunge la scheda **Rilievo** per il rilievo architettonico a misure dirette
(cordella o distanziometro):

1. disegni il vano **a occhio** come polilinea chiusa, con i normali comandi del CAD;
2. il plugin scrive le **lettere dei vertici** (A, B, C …) — utili anche stampate
   come eidotipo da portare in cantiere;
3. inserisci in una tabella le **misure dei lati e delle diagonali**: il plugin
   propone le diagonali che servono (n − 3 per un vano di n vertici, scelte in modo
   che corrano dentro il vano, anche a L);
4. il vano viene **ricalcolato con la forma vera**: esatto con le misure minime,
   **compensato ai minimi quadrati** quando ci sono misure in più, con lo **scarto**
   di ogni misura e lo scarto quadratico medio σ₀ — come una compensazione Pregeo.

Il risultato va sul layer **RILIEVO_CORRETTO** (vertice A fermo, lato A–B nella
direzione disegnata); il disegno originale resta per confronto. Sul layer
**RILIEVO_QUOTE** vanno lettere, quote dei lati (stile di quota **RILIEVO**),
diagonali con la loro misura e una tabella *misurata / calcolata / scarto*, con in
rosso gli scarti oltre la tolleranza.

L'interfaccia è in **inglese**, oppure in **italiano** quando Open CAD Studio è in
italiano. `RV_LANG IT|EN|AUTO` forza una lingua.

## Comandi

| Comando | Pulsante | Cosa fa |
|---|---|---|
| `RV_VANO` | **Misure e diagonali** | seleziona il vano (o clicca dentro), scrive le lettere, apre la tabella delle misure e ricalcola |
| `RV_ETICHETTE` | **Lettere vertici** | scrive solo le lettere A, B, C … sui vertici (layer RILIEVO_ETICHETTE) |
| `RV_GUIDE` | **Guida** | breve guida |
| `RV_LANG IT\|EN\|AUTO` | | lingua dei messaggi |

### Consigli di rilievo

- Un vano di **n** vertici ha **2n − 3** incognite: servono tutti i lati più
  **n − 3 diagonali** (1 per un rettangolo, 3 per una L).
- **Misura sempre almeno una diagonale in più**: senza misure in più la forma è
  esatta ma un errore (una cifra letta male, un vertice sbagliato) non si vede.
  Con una diagonale in più il plugin calcola gli scarti e segnala le misure sospette.
- Le misure vanno nelle unità del disegno (metri, centimetri o millimetri); la
  virgola decimale va bene. Altezza dei testi e tolleranza sono proposte in base
  alla grandezza del vano e si possono cambiare.
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

Compilato contro `ocs_plugin_api` del tag **v2026.45** (Plugin API 16), come
Raster Tools 0.8.0. Per una versione successiva del CAD, aggiorna `rev` in
`Cargo.toml` con il commit del nuovo tag e `api_version` in `plugin.toml`.

## Struttura

```
src/lib.rs       manifest, punto d'ingresso del plugin
src/i18n.rs      scelta inglese / italiano
src/ribbon.rs    scheda Rilievo della barra multifunzione
src/commands.rs  comandi RV_*, layer, quote, tabella
src/tools.rs     selezione del vano, finestra delle misure, guida
src/solve.rs     calcolo puro: compensazione lati + diagonali, diagonali proposte (con test)
scripts/release.ps1  pubblicazione di una nuova versione
icons/           icone SVG della scheda
```

Licenza GPL-3.0-only, come Open CAD Studio.
