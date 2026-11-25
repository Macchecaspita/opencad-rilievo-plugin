"""Manual texts — Deutsch (maschinell übersetzt)."""

OUT = "manual_de.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — Aufmaß-Handbuch",
    "foot": "Room Survey — Aufmaß-Handbuch · Hobbyprojekt, nicht für den professionellen Einsatz",
    "subtitle": "Eine Wohnung mit direkten Messungen aufnehmen<br/>und in Open CAD Studio rekonstruieren",
    "warn": "<b>Haftungsausschluss.</b> Room Survey ist ein Hobbyprojekt, aus Spaß entstanden und <b>nicht für den "
            "professionellen Einsatz geeignet</b>. Es wird ohne jede Gewähr bereitgestellt; der Autor übernimmt keine "
            "Verantwortung für die Nutzung oder die Ergebnisse. Prüfen Sie die Ergebnisse immer mit zusätzlichen Messungen.",

    "d_room": "Raum {r}",
    "d_sketch": "Skizze: nummerierte Räume, Ecken mit Buchstaben (geschrieben von Schaltfläche 3. Räume)",
    "d_check": "B–D Kontrolle",
    "d_out1": "C–E: verläuft außen,",
    "d_out2": "nicht messbar",
    "d_from1b": "Abstand von 1B",
    "d_width": "0,90 Breite",
    "d_thick": "Stärke 0,30 (in der Leibung)",
    "d_from2d": "1,10 Abstand von 2D",
    "d_furn": "Möbel",
    "d_blocked": "A–C versperrt",

    "h1": "1. Die Methode in Kürze",
    "p1": "Sie zeichnen den Grundriss <b>nach Augenmaß</b>, messen vor Ort die Seiten und einige Diagonalen jedes Raums "
          "sowie einige Maße, die die Räume verbinden, und das Plugin rekonstruiert den <b>wahren</b> Grundriss, indem "
          "es die Maße nach der Methode der kleinsten Quadrate ausgleicht. Die Residuen zeigen sofort, ob ein Maß falsch ist.",
    "t1": [["", "Wo", "Was Sie tun"],
           ["1", "am Schreibtisch", "Skizze: jeden Raum als geschlossene Polylinie zeichnen (nach Augenmaß) und die Buchstaben schreiben"],
           ["2", "vor Ort", "Seiten, Diagonalen, Wandstärken, Türen und Fenster messen und auf der Skizze notieren"],
           ["3", "am Schreibtisch", "Maße eingeben, berechnen, Residuen lesen, Zeichnung fertigstellen"]],
    "h_tools": "Ausrüstung",
    "tools": [
        "Laser-Entfernungsmesser (besser, wenn er von der Geräterückseite misst) und ein 5-m-Maßband für kurze Maße "
        "(Stärken, Leibungen, Nischen);",
        "die gedruckte Skizze, Bleistift und Radiergummi; ein Klemmbrett;",
        "ein Zollstock oder Winkel, um die Winkel zu prüfen, wenn die Diagonalen versperrt sind;",
        "Kamera oder Handy: ein Foto pro Raum, mit den Eckbuchstaben auf der Skizze notiert.",
    ],

    "h2": "2. Vor dem Termin: die Skizze im CAD",
    "p2a": "Zeichnen Sie <b>jeden Raum</b> als <b>geschlossene Polylinie</b> (PLINE … Schließen) entlang der "
           "<b>Innenseite</b> der Wände. Die Maße sind egal: wichtig ist, dass jede echte Ecke ein Scheitelpunkt der "
           "Polylinie ist, in der richtigen Reihenfolge. Rücksprünge, Nischen, vorstehende Pfeiler und Fasen sind "
           "ebenfalls Scheitelpunkte (Raum 3 im Beispiel). Der Flur ist ein Raum wie die anderen.",
    "p2b": "Drücken Sie <b>3. Räume: Nummern, Buchstaben und Maße</b> und wählen Sie die Räume — einzeln in der gewünschten Reihenfolge der Nummern oder alle zusammen mit einem Fenster (nummeriert von oben nach unten, links nach rechts) — und drücken Sie Enter: jeder Raum zeigt seine Nummer fett, die Ecken die Buchstaben A, B, C …; Ecke B von Raum 2 heißt 2B. Auch das Maßfenster öffnet sich: schließen Sie es vorerst mit <b>Schließen</b> und antworten Sie, die Buchstaben zu <b>behalten</b>. Drucken Sie den Plan: das ist Ihre Skizze.",
    "t2": [["Tipp", "Warum"],
           ["Nummerieren Sie die Räume in einer Reihenfolge, der Sie auch vor Ort folgen", "Sie schreiben schneller und verwechseln seltener den Raum."],
           ["Polylinie auf der Innenseite", "Gemessen wird in den Räumen; die Außenwände folgen später aus den Stärken."],
           ["Keine unnötigen Scheitelpunkte", "Ein Scheitelpunkt auf einer geraden Seite (180°-Winkel) verlangt unnötige Maße."]],

    "h3": "3. Vor Ort: einen Raum messen",
    "p3a": "Ein Raum mit <b>n</b> Ecken hat 2n − 3 Unbekannte: er wird bestimmt durch <b>alle Seiten</b> plus <b>n − 3 "
           "Diagonalen</b>. Ein Rechteck braucht 1 Diagonale, ein L-förmiger Raum 3, das Bad mit Rücksprung im Beispiel 3.",
    "p3b": "Messen Sie die Seiten <b>auf gleicher Höhe</b> (z. B. 1 m über dem Boden, über Sockelleisten und niedrigen "
           "Möbeln), von Ecke zu Ecke. Dann die Diagonalen. Die vom Plugin vorgeschlagenen Diagonalen verlaufen alle "
           "<b>innerhalb</b> des Raums: in einem L-förmigen Raum gibt es nie eine Diagonale durch die Wand.",
    "cap3": "Links: Seiten (blau) und Diagonale (orange) eines nicht rechtwinkligen Raums; die zweite Diagonale (grün) ist "
            "das Kontrollmaß. Rechts: L-förmiger Raum, die drei nützlichen Diagonalen und eine zu vermeidende.",
    "h_check": "Das Kontrollmaß",
    "p_check": "<b>Nehmen Sie immer eine Diagonale mehr</b> als das Minimum. Ohne zusätzliche Maße ist die Form exakt, was "
               "auch immer Sie eingeben, und eine falsch gelesene Zahl (6,85 statt 6,58) bleibt unbemerkt. Mit einem "
               "zusätzlichen Maß berechnet das Plugin die <b>Residuen</b> und σ₀ und färbt Maße über der Toleranz "
               "(meist 1–2 cm) rot.",
    "h_obst": "Wenn Möbel eine Diagonale versperren",
    "obst": [
        "Versuchen Sie zuerst eine andere Diagonale (B–D statt A–C) oder messen Sie über den Möbeln, mit waagerecht "
        "gehaltenem Entfernungsmesser.",
        "Wenn das nicht geht, verwenden Sie eine <b>Bedingung</b> (auf der Seite des Raums): Ecke und Winkel, z. B. "
        "<b>A, 90</b>, nachdem Sie ihn mit dem Winkel geprüft haben; oder eine <b>horizontale</b> bzw. <b>vertikale</b> "
        "Seite. Jede Bedingung zählt als ein Maß und hat ebenfalls ein Residuum.",
        "Fehlt dann noch etwas, nimmt die Option <b>Form mit den Winkeln der Zeichnung vervollständigen</b> (meist "
        "aktiv) das Fehlende aus der Zeichnung, und die Tabelle markiert es rot. Dort ist das Ergebnis nur so gut wie "
        "Ihre Zeichnung nach Augenmaß: notieren Sie es.",
    ],

    "h4": "4. Die Räume verbinden",
    "p4a": "Jeder Raum hat, für sich gemessen, die richtige Form, weiß aber nicht, wo er gegenüber den anderen liegt. "
           "Zum Verbinden brauchen Sie <b>mindestens 3 Maße</b> zum Nachbarraum oder eine <b>gemeinsame Tür</b>. "
           "Die Tür ist der bequemste Weg.",
    "cap4": "Eine Tür zwischen Raum 1 und Raum 2: Abstand von der Ecke und Breite auf jeder Seite gemessen, "
            "Wandstärke in der Leibung gemessen.",
    "p4b": "Drücken Sie im Maßfenster unter den Räumen <b>＋ Tür oder Fenster hinzufügen</b> und <b>klicken Sie in die Zeichnung</b>, auf die Wand zwischen den beiden Räumen, nahe der Tür. Das Plugin findet beide Seiten selbst (grün hervorgehoben) und fragt:",
    "t4": [["Feld", "Beispiel"],
           ["Abstand von Ecke 1B bis zur Leibung (gemessen in Raum 1)", "1,00"],
           ["Breite", "0,90"],
           ["Wandstärke, in der Tür gemessen", "0,30"],
           ["Abstand von Ecke 2D bis zur Leibung (gemessen in Raum 2)", "1,10"]],
    "p4c": "Der Abstand reicht von der Ecke bis zur Leibung; das Plugin schlägt die Ecke vor, die dem Klickpunkt am "
           "nächsten liegt, und Sie können die andere wählen. Das ist alles: die beiden Räume sind verbunden "
           "(gegenüberliegende Leibungen, parallele Wand, bekannte Stärke). Ein zusätzliches Maß durch die Tür "
           "(z. B. von 1B nach 2D) dient als Kontrolle.",
    "p4d": "Ohne Tür (oder zur Kontrolle) verwenden Sie den Abschnitt <b>Weitere Maße zwischen Räumen</b> unten im Fenster:",
    "links": [
        "<b>Wandstärke</b>: Von = eine Seite (1B-1C), Bis = die Ecke des anderen Raums hinter dieser Wand (2A), "
        "Gemessen = die rechtwinklig in der Tür gemessene Stärke;",
        "<b>Maße durch die Tür</b>: Von = 1B, Bis = 2D, zwischen Ecken der beiden Räume, die man durch die Tür sieht; "
        "nehmen Sie zwei zu verschiedenen Ecken.",
    ],
    "h_order": "Empfohlene Reihenfolge für die Beispielwohnung",
    "p_order": "Beginnen Sie mit dem <b>Flur</b> (Raum 4), der fast alle anderen berührt, und verbinden Sie jeden Raum über "
               "seine Tür mit dem Flur: Wohnzimmer und Schlafzimmer links, Küche und Schlafzimmer 2 rechts; das Bad "
               "wird mit dem Schlafzimmer verbunden. So hängt jeder Raum an einer kurzen Kette, und ein Fehler bleibt lokal.",

    "h_fix": "Fester Umriss, mit dem Tachymeter aufgenommen",
    "p_fix": "Wurde der Außenumriss des Gebäudes mit einem Tachymeter aufgenommen (oder ist er ohnehin exakt), nutzt das Plugin ihn als feste Referenz: er wird nie verschoben, und die Räume platzieren sich darin.",
    "fix": ["zeichnen Sie den Umriss als <b>geschlossene Polylinie</b> in wahren Koordinaten und die Räume nach Augenmaß darin;", "drücken Sie <b>1. Außenumriss</b>, klicken Sie die Polylinie an und drücken Sie Enter (Enter ohne Klick = kein Umriss);", "drücken Sie <b>2. Außenwandstärken</b> und klicken Sie die Umrissseiten nacheinander an: geben Sie für jede die Wandstärke ein (an den Fenstern gemessen) und wählen Sie die Richtung, <i>zur Mitte</i> für eine normale Außenwand (<i>nach außen</i> zeichnet nur die Linie, die nicht in die Berechnung eingeht). Sobald Sie OK drücken, erscheinen die Parallele dieser Seite und ihr Wert in der Zeichnung, auf dem Layer RILIEVO_PERIMETRO: eine Seite ohne Linie ist eine vergessene Seite. Enter beendet: die Zusammenfassung listet die Seiten ohne Stärke;", "in der Berechnung bleiben die Raumseiten entlang des Umrisses parallel dazu, in diesem Abstand: ein Eckraum mit zwei Seiten am Umriss braucht für seine Lage keine Diagonalen; ein Raum mit nur einer Seite am Umriss muss trotzdem mit den anderen verbunden werden (z. B. über eine Tür)."],
    "h_hatch": "Massive Wände zwischen Räumen: die Schraffur",
    "p_hatch": "Durch eine massive Wand kann man nicht messen, aber ihre Stärke wird an den Türen gemessen. Wenn Sie in der Zeichnung eine <b>Schraffur</b> in die Wände zwischen den Räumen legen, findet das Plugin sie selbst:",
    "hatch": ["es findet die zwei gegenüberliegenden Seiten verschiedener Räume mit der Schraffur dazwischen und schlägt sie im Abschnitt <b>Massive Wände zwischen Räumen</b> des Maßfensters vor;", "geben Sie die <b>an einer Tür gemessene Stärke</b> ein; hat dieselbe Wand schon eine gemeinsame Tür mit Stärke, kann das Feld leer bleiben;", "in der Berechnung bleiben die beiden Seiten in diesem Abstand parallel, mit ihrem Residuum in der Tabelle."],
    "p_hatch2": "Eine massive Wand legt Abstand und Drehung zwischen zwei Räumen fest, aber nicht die Verschiebung entlang der Wand: Sie brauchen weiterhin eine Tür in derselben Wand, eine zweite, nicht parallele Wand zum selben Raum oder den festen Umriss.",

    "h5": "5. Türen, Fenster und Außenwände",
    "h_dw": "Türen und Fenster",
    "p_dw": "Auch Fenster und Außentüren werden mit <b>Tür oder Fenster hinzufügen</b> und einem Klick auf die Wand "
            "erfasst: Art (<b>Tür</b>, <b>Fenster</b>, <b>Öffnung</b>), Abstand von der Ecke bis zur Leibung und "
            "lichte Breite. Die Tür ist auf der Seite der Ecke angeschlagen, von der Sie messen. Notieren Sie die Höhen "
            "(Brüstung, Sturz) auf der Skizze: das Plugin arbeitet im Grundriss.",
    "h_ext": "Außenwände",
    "p_ext": "Das Plugin rekonstruiert die Räume auf der Innenseite. Für den Außenumriss messen Sie die <b>Wandstärke an "
             "den Fenstern</b> (inklusive Leibung), mindestens eine pro Fassade; zeichnen Sie dann im CAD die "
             "Außenseite mit VERSATZ von den Raumseiten im gemessenen Abstand und verbinden Sie die Ecken. Prüfen Sie "
             "wenn möglich mit einem Außenmaß (Länge einer Fassade), das von draußen genommen wird.",

    "h6": "6. Am Schreibtisch: berechnen und prüfen",
    "calc": ["Drücken Sie <b>3. Räume: Nummern, Buchstaben und Maße</b>, wählen Sie alle Räume (gleiche Reihenfolge wie auf der Skizze) und drücken Sie Enter. Es öffnet sich <b>ein einziges Fenster</b> mit allen Räumen untereinander, von Raum 1 bis zum letzten, jeder mit Titel und Zustand: scrollen Sie mit der Leiste rechts und geben Sie Seiten und Diagonalen ein. Unter den Räumen folgen Türen und Fenster, massive Wände, weitere Maße zwischen Räumen und Optionen.", "Drücken Sie <b>▶ Berechnen</b>: die Polylinien der Räume werden in die berechnete Form gebracht und <b>die Zeichnung aktualisiert sich selbst</b> (Strg+Z macht es rückgängig). Reichen die Maße nicht oder passen sie nicht zusammen, öffnet sich das Fenster erneut mit dem Grund und den bereits eingegebenen Maßen.", "Bemaßungen, Diagonalen und Tabelle kommen auf <b>RILIEVO_QUOTE</b>; Türen und Fenster auf <b>RILIEVO_APERTURE</b>. Ein Fenster <b>Ergebnis der Berechnung</b> fasst Flächen, überzählige Maße und σ₀ zusammen.", "Lesen Sie die Residuentabelle: ein rotes Residuum markiert das erneut zu prüfende Maß. Mit nur einem überzähligen Maß verteilt sich der Fehler auf mehrere Maße: ein weiteres Maß hilft, ihn zu finden.", "<b>Messbuch</b> speichert alles als CSV für Excel oder Calc (Maße, Residuen, Koordinaten, Flächen)."],
    "h_vals": "Typische Werte",
    "t6": [["Größe", "Typischer Wert", "Wenn höher"],
           ["Residuum einer Seite oder Diagonale", "unter 1 cm", "neu messen; vertauschte Ziffern und die richtige Ecke prüfen"],
           ["σ₀ eines Raums", "0,3–0,8 cm", "Maße auf verschiedenen Höhen, Ecken nicht scharf"],
           ["Winkel auf 90° festgelegt", "Residuum unter 0,2°", "der Winkel ist nicht rechtwinklig: Bedingung entfernen und eine Diagonale messen"]],
    "h_list": "Checkliste vor dem Verlassen der Baustelle",
    "list": [
        "jeder Raum hat alle Seiten und mindestens n − 3 Diagonalen (oder geprüfte Bedingungen), plus ein Kontrollmaß;",
        "jeder Raum ist verbunden: gemeinsame Tür mit Stärke oder 3 Maße zu einem bereits verbundenen Raum;",
        "Außenwandstärken an den Fenstern gemessen, mindestens eine pro Fassade;",
        "Türen und Fenster: Abstand von der Ecke, Breite, Höhen notiert;",
        "ein Foto pro Raum.",
    ],
    "ex_h": "7. Ein vollständiges Beispiel, Schritt für Schritt",
    "ex_p": "Eine Wohnung mit sieben Räumen in einem Gebäude mit schräger linker Fassade und einem Flügel mit abgeschrägter Ecke. Der Außenumriss wurde mit einem Tachymeter aufgenommen; die Innenwände sind massiv, ihre Stärke kann nur an den Türen gemessen werden.",
    "ex_fig": "Umriss E1–E7 aufgenommen, Räume 1–7 nach Augenmaß, massive Wände schraffiert",
    "ex_cap": "Die Ausgangszeichnung: fester Umriss E1–E7, nummerierte Räume mit Eckbuchstaben, Wände mit Schraffur gefüllt, Türen P1–P6 und Fenster (blau).",
    "ex_ties": [["Raum", "Seiten am Umriss", "Massive Wände (Schraffur)", "Türen"], ["1", "C–D, D–A", "A–B mit 4; B–C mit 2", "P5"], ["2", "C–D", "D–A mit 1; A–B mit 5; B–C mit 3", "P2"], ["3", "B–C, C–D, D–E", "A–B mit 6; E–A mit 2", "P6"], ["4", "A–B, D–A", "C–D mit 1; B–C mit 5", "P1, P5"], ["5", "A–B", "D–A mit 4; C–D mit 2; B–C mit 6 und 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A mit 5; A–B mit 7; C–D mit 3", "P3, P6"], ["7", "A–B, B–C", "C–D mit 6; D–A mit 5", "P4"]],
    "ex_ties_note": "Sechs von sieben Räumen berühren den Umriss mit zwei nicht parallelen Seiten: der Umriss allein bringt sie an ihren Platz. Raum 2 hat nur eine Seite am Umriss, aber die massive Wand zu Raum 1 (nicht parallel) fixiert ihn. Massive Wände und Türen wirken so auch als Kontrollen.",
    "ex_steps": [["Der Umriss vom Tachymeter", "Nehmen Sie die Außenecken des Gebäudes E1…E7 mit dem Tachymeter auf und verbinden Sie sie zu einer geschlossenen Polylinie in wahren Koordinaten. Sie ist das einzige exakte Element der Zeichnung: das Plugin verschiebt sie nie."], ["Die Räume nach Augenmaß", "Zeichnen Sie innerhalb des Umrisses jeden Raum als geschlossene Polylinie auf der Innenseite der Wände. Nur Anzahl und Reihenfolge der Ecken zählen: Raum 3 hat fünf Ecken, weil die Außenwand abgeschrägt ist, Räume 1 und 4 folgen der schrägen Fassade."], ["Die Schraffur in den massiven Wänden", "Füllen Sie mit dem Schraffurbefehl (HATCH) den Raum zwischen Umriss und Räumen, also alle Wände. Die Schraffur sagt dem Plugin, welche Räume durch eine massive Wand getrennt sind: sie muss nicht genau sein, nur zwischen den beiden Seiten liegen."], ["Die Buchstaben", "Drücken Sie <b>3. Räume: Nummern, Buchstaben und Maße</b>, klicken Sie die Räume in der Reihenfolge 1…7 der Abbildung an und drücken Sie Enter. Nummern und Buchstaben erscheinen sofort, und das Maßfenster öffnet sich: schließen Sie es und behalten Sie die Buchstaben. Drucken Sie die Zeichnung: das ist die Skizze für die Baustelle."], ["Vor Ort: die Räume", "Messen Sie in jedem Raum die Seiten auf gleicher Höhe und mindestens eine Diagonale. Räume mit zwei Seiten am Umriss brauchen für ihre Form keine Diagonalen, aber eine zusätzliche Diagonale ist der einzige Weg, eine falsch gelesene Zahl zu bemerken. Für Raum 1 zum Beispiel:"], ["Vor Ort: Stärken und Türen", "Messen Sie die Stärke der Außenwände an den Fenstern (hier 0,40) und die der Innenwände an den Türen (hier 0,30). Notieren Sie für jede Tür auf beiden Seiten den Abstand von der Ecke bis zur Leibung und die Breite:"], ["Schaltflächen 1 und 2: Umriss und Stärken", "Drücken Sie am Schreibtisch <b>1. Außenumriss</b>, klicken Sie die Polylinie E1…E7 an und drücken Sie Enter. Drücken Sie dann <b>2. Außenwandstärken</b>, klicken Sie die erste Seite an, geben Sie 0,40 ein, lassen Sie die Richtung <i>zur Mitte</i> und haken Sie das Feld an, um sie auf alle Seiten anzuwenden: nach OK erscheinen die Parallelen aller Seiten auf einmal. Enter zum Beenden."], ["Schaltfläche 3: die Räume und das Maßfenster", "Drücken Sie <b>3. Räume: Nummern, Buchstaben und Maße</b>, klicken Sie die Räume in derselben Reihenfolge 1…7 an und drücken Sie Enter. Das Fenster öffnet sich mit den Räumen 1 bis 7 untereinander; weiter unten der Abschnitt <b>Massive Wände zwischen Räumen (aus der Schraffur)</b> mit bereits vorgeschlagenen Zeilen: geben Sie 0,30 ein, wo keine gemeinsame Tür ist; wo eine ist, wird die Stärke von der Tür übernommen."], ["Seiten, Diagonalen und Türen", "Scrollen Sie im Fenster und geben Sie Seiten und Diagonalen jedes Raums ein. Drücken Sie dann <b>＋ Tür oder Fenster hinzufügen</b> und klicken Sie die Wand jeder Tür P1…P6 an: das Plugin findet beide Seiten und fragt nach Abständen, Breite und Stärke, dann kehrt es zum Fenster zurück. Fenster werden genauso an der Außenwand hinzugefügt."], ["Berechnen", "Wenn jeder Raum ✔ zeigt und die Räume verbunden sind, drücken Sie <b>▶ Berechnen</b>: die Polylinien der Räume nehmen die berechnete Form an und die Zeichnung aktualisiert sich selbst. Das Fenster <b>Ergebnis der Berechnung</b> fasst überzählige Maße und σ₀ zusammen; in der Tabelle markieren rote Residuen die erneut zu prüfenden Maße. <b>Messbuch</b> speichert alles als CSV."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Maß (Raum 1)", "Wert", "Hinweis"], ["1A–1B", "4,10", "Seite"], ["1B–1C", "8,30", "Seite"], ["1C–1D", "4,58", "Seite am Umriss"], ["1D–1A", "8,31", "Seite am Umriss (schräg)"], ["1A–1C", "9,26", "Kontrolldiagonale"], ["1B–1D", "9,48", "zweite Diagonale (optional)"]],
    "ex_doors": [["Tür", "Erster Raum: Seite, Abstand", "Zweiter Raum: Seite, Abstand", "Breite", "Stärke"], ["P1", "4B–4C, von 4B 1,60", "5D–5A, von 5A 1,60", "0,90", "0,30"], ["P2", "5C–5D, von 5D 1,30", "2A–2B, von 2A 1,30", "0,90", "0,30"], ["P3", "5B–5C, von 5B 5,00", "6D–6A, von 6A 1,10", "0,90", "0,30"], ["P4", "5B–5C, von 5B 1,00", "7D–7A, von 7A 1,00", "0,90", "0,30"], ["P5", "4C–4D, von 4C 1,10", "1A–1B, von 1B 1,10", "0,90", "0,30"], ["P6", "6C–6D, von 6D 1,30", "3A–3B, von 3A 1,30", "0,80", "0,30"]],
    "ex_end": "Mit dem einmal aufgenommenen Umriss geht kein Maß durch die Wände: Seiten und Diagonalen werden in den Räumen gemessen, Stärken an Türen und Fenstern.",
    "end": "Room Survey ist ein Hobbyprojekt, nicht für den professionellen Einsatz geeignet: der Autor übernimmt keine "
           "Verantwortung für die Nutzung. Code und Updates: github.com/Macchecaspita/opencad-rilievo-plugin",
    "h_holes": 'Türen und Fenster in der Zeichnung öffnen: die Schaltfläche «Türen und Fenster»',
    "p_holes": 'Nach der Berechnung öffnet die Schaltfläche <b>Türen und Fenster</b> die Öffnungen in den Wänden: sie unterbricht die Wandlinien, zeichnet die Laibungen, leert die Schraffur und zeichnet den Rahmen mit seiner Bemaßung.',
    "cap_holes": 'Ein Fenster 120 breit und 140 cm hoch, 1,50 von der Ecke 1A: Rahmen an der Innenseite der Wand; außerhalb des Gebäudes die graue Klammer und das Maß Breite über Höhe, in Zentimetern.',
    "holes": ['drücken Sie <b>Türen und Fenster</b> und wählen Sie <b>Tür</b> oder <b>Fenster</b>, mit <b>Breite</b> und <b>Höhe</b> (die Werte werden für die nächste Öffnung wieder vorgeschlagen);', 'klicken Sie die <b>Ecke</b> an, von der Sie gemessen haben: eine Ecke des Außenumrisses oder eines Raums;', 'klicken Sie die <b>Seite</b> an, auf der die Öffnung liegt, und geben Sie den <b>Abstand</b> von der Ecke bis zur Laibung ein;', 'das Plugin unterbricht die angeklickte Seite und die Linie auf der anderen Seite der Wand (auch wenn sie nicht parallel ist), zeichnet die Laibungen und leert die Schraffur in der Öffnung;', 'auf dem Layer <b>RILIEVO_APERTURE</b> erscheinen der Rahmen und das Maß Breite/Höhe in Zentimetern, außerhalb des Gebäudeumrisses.'],
    "p_holes2": 'Das Fenster hat seinen Rahmen an der Innenseite der Wand; die Tür lässt die Öffnung frei. Der Befehl arbeitet nur an der Zeichnung: ein geöffneter Raum ist keine geschlossene Polylinie mehr und geht nicht mehr in die Berechnung ein, öffnen Sie die Öffnungen also <b>nach</b> der Berechnung. Strg+Z macht die letzte Öffnung rückgängig.',
}
