# opencad-rilievo-plugin — Rilievo Vani for Open CAD Studio

**English** · [Italiano](README.it.md)

> **Hobby project, made just for fun and not suitable for professional use.**
> Provided as is, without any warranty (see the GPL-3.0 licence): the author takes
> no responsibility for its use or for the results. Always check the results and
> back up your drawings. Early version (0.x).

An [Open CAD Studio](https://github.com/HakanSeven12/OpenCADStudio) add-on that
adds a **Survey** ribbon tab for architectural surveys measured with a tape or a
laser meter:

1. draw each room **by eye** as a closed polyline with the CAD's own commands;
2. the plugin writes **corner letters** (A, B, C …) — printable as a field sketch;
3. type the measured **sides and diagonals** in a table: the plugin proposes the
   diagonals needed (n − 3 for n corners, all running inside the room);
4. the room is **recomputed with its true shape**: exact with the minimum
   measures, **least-squares adjusted** when there are more, with the **residual**
   of every measure and the standard deviation σ₀.

Several **linked rooms** can be surveyed together (corners 1A, 1B … 2A …), tied by
**wall thicknesses** and **measures through doors** and adjusted at once; the last
calculation can be saved as a **measures book** (CSV for Excel or Calc). Every
dialog has its own help ("How to measure").

Known angles and horizontal / vertical sides can replace diagonals
(**constraints**); **doors and windows** are placed on the sides, and a **shared
door** written in both rooms with the wall thickness links them by itself. Each
room shows its **area in m²**.

The result goes on layer **RILIEVO_CORRETTO** (corner A fixed, side A–B in the
drawn direction); the original stays for comparison. Layer **RILIEVO_QUOTE** gets
the letters, side dimensions (dimension style **RILIEVO**), diagonals with their
length and a *measured / computed / residual* table, with residuals over the
tolerance in red.

Commands: `RV_VANO` (sides, diagonals, links), `RV_ETICHETTE` (corner letters),
`RV_LIBRETTO` (measures book), `RV_GUIDE`, `RV_LANG IT|EN|AUTO`.

Build with `cargo build --release`; install the library and `plugin.toml` in
`<config>/OpenCADStudio/plugins/opencad.rilievovani/`, or add the repository
`Macchecaspita/opencad-rilievo-plugin` in the Plugin Manager (`PLUGINS`).

Built against `ocs_plugin_api` at tag **v2026.45** (Plugin API 16).
Licence GPL-3.0-only, like Open CAD Studio.
