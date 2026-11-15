"""Manual texts — English."""

OUT = "manual_en.pdf"
DEC = "."
S = {
    "doc_title": "Room Survey — surveying manual",
    "foot": "Room Survey — surveying manual · hobby project, not for professional use",
    "subtitle": "How to survey a flat with direct measurements<br/>and rebuild it in Open CAD Studio",
    "warn": "<b>Disclaimer.</b> Room Survey is a hobby project, made for fun and <b>not suitable for professional "
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

    "h_fix": "Fixed perimeter surveyed with a total station",
    "p_fix": "If the outer perimeter of the building was surveyed with a total station (or is exact anyway), the plugin uses it as a fixed reference: it is never moved, and the rooms place themselves inside it.",
    "fix": ["draw the perimeter as a <b>closed polyline</b>, in true coordinates, and the rooms by eye inside it;", "when <b>Sides and diagonals</b> starts, a window asks for the <b>fixed perimeter</b> (optional), before the rooms are chosen: choose to click it on the drawing, or go on without it. It can also be added later, from the list in the measures window;", "on the <b>Outer walls</b> page type the thickness of the outer walls (measured at the windows) and check the room sides proposed along the perimeter: each stays parallel to the perimeter, at that distance;", "a corner room with two sides on the perimeter needs no diagonals for its position; a room with only one side on the perimeter must still be tied to the others (for example with a door)."],
    "h_hatch": "Solid walls between rooms: the hatch",
    "p_hatch": "A solid wall cannot be crossed to measure, but its thickness is measured at the doors. If you put a <b>hatch</b> in the walls between the rooms in the drawing, the plugin finds them by itself:",
    "hatch": ["it finds the two sides of different rooms facing each other with the hatch between them and proposes them on the <b>Solid walls between rooms</b> page;", "type the <b>thickness measured at a door</b>; if the same wall already has a shared door with its thickness, the box can stay empty;", "in the calculation the two sides stay parallel at that distance, with their residual in the table."],
    "p_hatch2": "A solid wall fixes the distance and the turn between two rooms, but not the shift along the wall: you still need a door in the same wall, a second non-parallel wall towards the same room, or the fixed perimeter.",

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
    "ex_h": "7. A complete example, step by step",
    "ex_p": "A flat of seven rooms, in a building with a slanted left façade and a wing with a chamfered corner. The outer perimeter was surveyed with a total station; the inner walls are solid and their thickness can only be measured at the doors.",
    "ex_fig": "perimeter E1–E7 surveyed, rooms 1–7 drawn by eye, solid walls hatched",
    "ex_cap": "The starting drawing: fixed perimeter E1–E7, numbered rooms with the corner letters, walls filled with the hatch, doors P1–P6 and windows (in blue).",
    "ex_ties": [["Room", "Sides on the perimeter", "Solid walls (hatch)", "Doors"], ["1", "C–D, D–A", "A–B with 4; B–C with 2", "P5"], ["2", "C–D", "D–A with 1; A–B with 5; B–C with 3", "P2"], ["3", "B–C, C–D, D–E", "A–B with 6; E–A with 2", "P6"], ["4", "A–B, D–A", "C–D with 1; B–C with 5", "P1, P5"], ["5", "A–B", "D–A with 4; C–D with 2; B–C with 6 and 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A with 5; A–B with 7; C–D with 3", "P3, P6"], ["7", "A–B, B–C", "C–D with 6; D–A with 5", "P4"]],
    "ex_ties_note": "Six rooms out of seven touch the perimeter with two non-parallel sides: the perimeter alone puts them in place. Room 2 has only one side on the perimeter, but the solid wall towards room 1 (not parallel) locks it. Solid walls and doors thus also act as checks.",
    "ex_steps": [["The perimeter from the total station", "Survey the outer corners of the building E1…E7 with the total station and join them into a closed polyline, in true coordinates. It is the only exact element of the drawing: the plugin will never move it."], ["The rooms by eye", "Inside the perimeter draw each room as a closed polyline on the inner face of the walls. Only the number and order of the corners matter: room 3 has five corners because the outer wall is chamfered, rooms 1 and 4 follow the slanted façade."], ["The hatch in the solid walls", "With the hatch command (HATCH) fill the space between the perimeter and the rooms, that is all the walls. The hatch tells the plugin which rooms are separated by a solid wall: it need not be precise, it only has to lie between the two sides."], ["The letters", "Start <b>Corner letters</b>, click the rooms in the order 1…7 of the figure and press Enter. The letters appear at once; in the small window you can change their height. Print the drawing: it is the sketch to take on site."], ["On site: the rooms", "In every room measure the sides at the same height and at least one diagonal. Rooms with two sides on the perimeter do not need diagonals for their shape, but an extra diagonal is the only way to notice a misread figure. For room 1, for example:"], ["On site: thicknesses and doors", "Measure the thickness of the outer walls at the windows (here 0.40) and that of the inner walls at the doors (here 0.30). For every door note, on both sides, the distance from the corner to the jamb and the width:"], ["Sides and diagonals: perimeter and rooms", "Start <b>Sides and diagonals</b>. The first window asks for the fixed perimeter: choose <b>Choose the perimeter</b> and click the polyline E1…E7. Then click the rooms in the same order 1…7 and press Enter."], ["Outer walls and solid walls", "The measures window opens on the <b>Outer walls</b> page: type 0.40 as thickness and check the proposed sides (second column of the table). Then open <b>Solid walls between rooms, from the hatch</b>: the rows are already proposed; type 0.30 where there is no shared door, where there is one it is taken from the door."], ["Sides, diagonals and doors", "Open one room at a time from the list and type sides and diagonals. Then choose <b>Add a door or window</b> and click on the wall of each door P1…P6: the plugin finds the two sides and asks for distances, width and thickness. Windows are added the same way, on the outer wall."], ["Calculation and check", "When the list shows all the rooms complete and tied, press <b>Compute</b>. The <b>Result of the calculation</b> window sums up extra measures and σ₀; in the table the red residuals mark the measures to check again. If all is well, <b>Apply to drawing</b> moves the room polylines to the computed shape and <b>Measures book</b> saves everything in a CSV."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Measure (room 1)", "Value", "Note"], ["1A–1B", "4.10", "side"], ["1B–1C", "8.30", "side"], ["1C–1D", "4.58", "side on the perimeter"], ["1D–1A", "8.31", "side on the perimeter (slanted)"], ["1A–1C", "9.26", "check diagonal"], ["1B–1D", "9.48", "second diagonal (optional)"]],
    "ex_doors": [["Door", "First room: side, distance", "Second room: side, distance", "Width", "Thickness"], ["P1", "4B–4C, from 4B 1.60", "5D–5A, from 5A 1.60", "0.90", "0.30"], ["P2", "5C–5D, from 5D 1.30", "2A–2B, from 2A 1.30", "0.90", "0.30"], ["P3", "5B–5C, from 5B 5.00", "6D–6A, from 6A 1.10", "0.90", "0.30"], ["P4", "5B–5C, from 5B 1.00", "7D–7A, from 7A 1.00", "0.90", "0.30"], ["P5", "4C–4D, from 4C 1.10", "1A–1B, from 1B 1.10", "0.90", "0.30"], ["P6", "6C–6D, from 6D 1.30", "3A–3B, from 3A 1.30", "0.80", "0.30"]],
    "ex_end": "With the perimeter surveyed once, no measure crosses the walls: sides and diagonals are taken inside the rooms, thicknesses at the doors and windows.",
    "end": "Room Survey is a hobby project, not suitable for professional use: the author accepts no responsibility "
           "for its use. Code and updates: github.com/Macchecaspita/opencad-rilievo-plugin",
}
