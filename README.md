# opencad-rilievo-plugin — Room Survey for Open CAD Studio

**English** · [Italiano](README.it.md)

> **Hobby project, made just for fun and not suitable for professional use.**
> Provided as is, without any warranty (see the GPL-3.0 licence): the author takes
> no responsibility for its use or for the results. Always check the results and
> back up your drawings. Early version (0.x).

An [Open CAD Studio](https://github.com/HakanSeven12/OpenCADStudio) add-on that
adds a **Survey** ribbon tab for architectural surveys measured with a tape or a
laser meter:

1. draw each room **by eye** as a closed polyline with the CAD's own commands;
2. *(optional)* **1. Outer perimeter**: choose the building outline surveyed with
   a total station, which will stay still;
3. *(optional)* **2. Outer wall thicknesses**: click the perimeter side by side and
   type the wall thickness and direction (towards the centre or the outside) for each: the line parallel to the side is drawn;
4. **3. Rooms: numbers, letters and measures**: select the rooms and press Enter;
   each room gets its number and the corners their letters (A, B, C … — printable
   as a field sketch) and **a single window** opens with all the rooms, from room 1
   to the last, scrolled with the bar on the right; the plugin proposes the
   diagonals needed (n − 3 for n corners, all running inside the room);
5. press **Compute**: the rooms are **recomputed with their true shape** and the
   drawing updates by itself — exact with the minimum measures, **least-squares
   adjusted** when there are more, with the **residual** of every measure and the
   standard deviation σ₀.

Several **linked rooms** can be surveyed together (corners 1A, 1B … 2A …), tied by
**wall thicknesses** and **measures through doors** and adjusted at once; the last
calculation can be saved as a **measures book** (CSV for Excel or Calc). Every
dialog has its own help ("How to measure").

Known angles and horizontal / vertical sides can replace diagonals
(**constraints**); **doors and windows** are placed on the sides, and a **shared
door** written in both rooms with the wall thickness links them by itself. Inside
each room only its number is written; the **areas in m²** are in the result
window and in the measures book.

Compute moves the room polylines to the computed shape (Ctrl+Z undoes it). Layer
**RILIEVO_QUOTE** gets
the letters, side dimensions (dimension style **RILIEVO**), diagonals with their
length and a *measured / computed / residual* table, with residuals over the
tolerance in red.

Commands: `RV_PERIMETRO` (outer perimeter), `RV_SPESSORI` (outer wall thicknesses),
`RV_VANO` (rooms: numbers, letters, measures), `RV_ETICHETTE` (corner letters only),
`RV_APPLICA` (move the original rooms to the result), `RV_FORO` (doors and windows cut into the walls of the finished drawing, with a width/height label outside the building), `RV_LIBRETTO` (measures book), `RV_GUIDE`, `RV_MANUALE` (surveying manual, PDF), `RV_LANG <code>|AUTO`.

The interface and the PDF manual are in the same 23 languages as Open CAD
Studio (Arabic, Bulgarian, Chinese simplified and traditional, Czech, Dutch,
English, Finnish, French, German, Greek, Hindi, Hungarian, Italian, Japanese,
Korean, Lao, Polish, Portuguese, Russian, Spanish, Turkish, Vietnamese) and
follow its language; `RV_LANG it|en|de|zh-CN|…|AUTO` forces one. Italian and
English are the reference texts: the other languages are machine translations,
not reviewed by native speakers, so corrections are welcome.

Build with `cargo build --release`; install the library and `plugin.toml` in
`<config>/OpenCADStudio/plugins/opencad.rilievovani/`, or add the repository
`Macchecaspita/opencad-rilievo-plugin` in the Plugin Manager (`PLUGINS`).

Built against `ocs_plugin_api` at tag **v2026.46** (Plugin API 23).
Licence GPL-3.0-only, like Open CAD Studio.
