"""Manual texts — English."""

OUT = "manual_en.pdf"
DEC = "."
S = {
    "doc_title": "Rilievo Vani — surveying manual",
    "foot": "Rilievo Vani — surveying manual · hobby project, not for professional use",
    "subtitle": "How to survey a flat with direct measurements<br/>and rebuild it in Open CAD Studio",
    "warn": "<b>Disclaimer.</b> Rilievo Vani is a hobby project, made for fun and <b>not suitable for professional "
            "use</b>. It is provided as is, without any warranty; the author accepts no responsibility for its use "
            "or for the results. Always check the results with extra measurements.",

    "d_room": "Room {r}",
    "d_sketch": "sketch: numbered rooms, corners with letters (written by Corner letters)",
    "d_check": "B–D check",
    "d_out1": "C–E: runs outside,",
    "d_out2": "cannot be measured",
    "d_from1b": "distance from 1B",
    "d_width": "0.90 width",
    "d_thick": "thickness 0.30 (in the jamb)",
    "d_from2d": "1.10 distance from 2D",
    "d_furn": "furniture",
    "d_blocked": "A–C blocked",

    "h1": "1. The method in short",
    "p1": "You draw the plan <b>by eye</b>, measure on site the sides and a few diagonals of every room, plus a few "
          "measures that tie the rooms together, and the plugin rebuilds the <b>true</b> plan by adjusting the "
          "measures with least squares. The residuals show at once if a measure is wrong.",
    "t1": [["", "Where", "What you do"],
           ["1", "at the desk", "sketch: draw each room as a closed polyline (by eye) and write the letters"],
           ["2", "on site", "measure sides, diagonals, wall thicknesses, doors and windows, noting them on the sketch"],
           ["3", "at the desk", "type in the measures, compute, read the residuals, finish the drawing"]],
    "h_tools": "Equipment",
    "tools": [
        "laser distance meter (better if it measures from the back of the device) and a 5 m tape for short "
        "measures (thicknesses, reveals, recesses);",
        "the printed sketch, pencil and eraser; a clipboard;",
        "a folding rule or a set square to check the angles when the diagonals are blocked;",
        "camera or phone: one photo per room, with the corner letters pencilled on the sketch.",
    ],

    "h2": "2. Before the visit: the sketch in the CAD",
    "p2a": "Draw <b>every room</b> as a <b>closed polyline</b> (PLINE … Close), following the <b>inner face</b> of "
           "the walls. The sizes do not matter: what matters is that every real corner is a vertex of the polyline, "
           "in the right order. Recesses, niches, protruding pillars and chamfers are vertices too (room 3 of the "
           "example). The corridor is a room like the others.",
    "p2b": "With <b>Corner letters</b> select the rooms — one at a time in the order you want for the numbers, or all "
           "together with a window (numbered top to bottom, left to right) — and press Enter: the corners get the "
           "letters A, B, C … and every room its number and area; corner B of room 2 is called 2B. A small window "
           "asks for the height of the letters: make them smaller where corners are close together. Print the plan: "
           "it is your sketch.",
    "t2": [["Tip", "Why"],
           ["Number the rooms in an order you also follow on site", "You write faster and pick the wrong room less often."],
           ["Polyline on the inner face", "Measures are taken from inside the rooms; outer walls come later from the thicknesses."],
           ["No useless vertices", "A vertex on a straight side (180° angle) asks for useless measures."]],

    "h3": "3. On site: measuring a room",
    "p3a": "A room with <b>n</b> corners has 2n − 3 unknowns: it is closed by <b>all the sides</b> plus <b>n − 3 "
           "diagonals</b>. A rectangle needs 1 diagonal, an L-shaped room 3, the bathroom with a recess in the example 3.",
    "p3b": "Measure the sides <b>at the same height</b> (for example 1 m above the floor, above skirting boards and low "
           "furniture), from corner to corner. Then the diagonals. The diagonals suggested by the plugin all run "
           "<b>inside</b> the room: in an L-shaped room there is never a diagonal that crosses the wall.",
    "cap3": "Left: sides (blue) and diagonal (orange) of an out-of-square room; the second diagonal (green) is the check "
            "measure. Right: L-shaped room, the three useful diagonals and one to avoid.",
    "h_check": "The check measure",
    "p_check": "<b>Always take one diagonal more</b> than the minimum. Without extra measures the shape is exact whatever "
               "you type, and a misread figure (6.85 instead of 6.58) goes unnoticed. With one extra measure the plugin "
               "computes the <b>residuals</b> and σ₀ and colours in red the measures beyond the tolerance (usually 1–2 cm).",
    "h_obst": "When furniture blocks a diagonal",
    "obst": [
        "First try a different diagonal (B–D instead of A–C), or raise it above the furniture keeping the "
        "distance meter level.",
        "If that is not possible, use a <b>constraint</b> (in the room's page): corner and angle, for example "
        "<b>A, 90</b>, after checking it with the set square; or a <b>horizontal</b> or <b>vertical</b> side. "
        "Each constraint counts as one measure and also gets a residual.",
        "If something is still missing, the option <b>finish the shape with the drawing's angles</b> (usually on) "
        "takes what is missing from the drawing, and the table marks it in red. At that spot the result is only as "
        "good as your drawing by eye: make a note of it.",
    ],

    "h4": "4. Tying the rooms together",
    "p4a": "Each room, measured on its own, has the right shape but does not know where it is with respect to the "
           "others. To tie it you need <b>at least 3 measures</b> towards the next room, or a <b>shared door</b>. "
           "The door is the handiest way.",
    "cap4": "A door between room 1 and room 2: distance from the corner and width measured on each side, wall "
            "thickness measured in the jamb.",
    "p4b": "From the window's list choose <b>Add a door or window</b> and <b>click on the drawing</b>, on the wall "
           "between the two rooms, near the door. The plugin finds the two sides by itself (it highlights them in "
           "green) and asks for:",
    "t4": [["Box", "Example"],
           ["Distance from corner 1B to the jamb (measured in room 1)", "1.00"],
           ["Width", "0.90"],
           ["Wall thickness, measured in the door", "0.30"],
           ["Distance from corner 2D to the jamb (measured in room 2)", "1.10"]],
    "p4c": "The distance runs from the corner to the jamb; the plugin suggests the corner nearest to the point you "
           "clicked and you can pick the other one. That is all: the two rooms are tied (jambs facing each other, "
           "parallel wall, known thickness). One extra measure through the door (for example from 1B to 2D) acts "
           "as a check.",
    "p4d": "Without a door (or as a check) choose <b>Other measures between rooms</b> from the list:",
    "links": [
        "<b>wall thickness</b>: From = a side (1B-1C), To = the corner of the other room behind that wall (2A), "
        "Measured = the thickness taken square in the door;",
        "<b>measures through the door</b>: From = 1B, To = 2D, between corners of the two rooms that can be seen "
        "through the door; take two towards different corners.",
    ],
    "h_order": "Suggested order for the example flat",
    "p_order": "Start from the <b>corridor</b> (room 4), which touches almost all the others, and tie every room to the "
               "corridor with its door: living room and bedroom on the left, kitchen and bedroom 2 on the right; the "
               "bathroom is tied to the bedroom. This way every room hangs on a short chain and an error stays local.",

    "h5": "5. Doors, windows and outer walls",
    "h_dw": "Doors and windows",
    "p_dw": "Windows and outside doors are also added with <b>Add a door or window</b> and a click on the wall: kind "
            "(<b>door</b>, <b>window</b>, <b>opening</b>), distance from the corner to the reveal and clear width. "
            "The door is hinged on the side of the corner you measure from. Note the heights (sill, lintel) on the "
            "sketch: the plugin works in plan.",
    "h_ext": "Outer walls",
    "p_ext": "The plugin rebuilds the rooms on the inner face. For the outer perimeter measure the <b>wall thickness "
             "at the windows</b> (reveal included), at least one per façade; then in the CAD draw the outer face with "
             "OFFSET from the rooms' sides, at the measured distance, and join the corners. Where possible check with "
             "an outside measure (the length of a façade) taken from outdoors.",

    "h6": "6. At the desk: computing and checking",
    "calc": [
        "<b>Sides and diagonals</b>: click all the rooms (same order as the sketch) and press Enter. The list of rooms "
        "opens with their state: open one room at a time and type sides and diagonals, add the doors with a click on "
        "the drawing; when everything is complete the list offers <b>Compute</b>.",
        "If the measures are not enough or do not agree, the window opens again with the reason and with the measures "
        "already typed.",
        "The result goes on layer <b>RILIEVO_CORRETTO</b>, with the area of every room; dimensions, diagonals and table "
        "on <b>RILIEVO_QUOTE</b>; doors and windows on <b>RILIEVO_APERTURE</b>. The drawing by eye stays for comparison. "
        "<b>Apply to drawing</b> then moves the original polylines to the computed shape.",
        "Read the table of residuals: a red residual marks the measure to check again. With only one extra measure "
        "the error is spread over several measures: taking another one helps to find it.",
        "<b>Measures book</b> saves everything in a CSV for Excel or Calc (measures, residuals, coordinates, areas).",
    ],
    "h_vals": "Typical values",
    "t6": [["Quantity", "Typical value", "If it is higher"],
           ["residual of a side or a diagonal", "within 1 cm", "measure again; check swapped digits and the right corner"],
           ["σ₀ of a room", "0.3–0.8 cm", "measures taken at different heights, corners not sharp"],
           ["angle constrained to 90°", "residual within 0.2°", "the angle is not square: remove the constraint and measure a diagonal"]],
    "h_list": "Checklist before leaving the site",
    "list": [
        "every room has all its sides and at least n − 3 diagonals (or checked constraints), plus a check measure;",
        "every room is tied: shared door with thickness, or 3 measures towards a room already tied;",
        "outer wall thicknesses taken at the windows, at least one per façade;",
        "doors and windows: distance from the corner, width, heights noted;",
        "one photo per room.",
    ],
    "end": "Rilievo Vani is a hobby project, not suitable for professional use: the author accepts no responsibility "
           "for its use. Code and updates: github.com/Macchecaspita/opencad-rilievo-plugin",
}
