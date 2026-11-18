"""Textes du manuel — français."""

OUT = "manual_fr.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — manuel de relevé",
    "foot": "Room Survey — manuel de relevé · projet amateur, pas pour un usage professionnel",
    "subtitle": "Comment relever un logement par mesures directes<br/>et le reconstruire dans Open CAD Studio",
    "warn": "<b>Avertissement.</b> Room Survey est un projet amateur, réalisé pour le plaisir et <b>non adapté à un "
            "usage professionnel</b>. Il est fourni tel quel, sans aucune garantie ; l'auteur décline toute "
            "responsabilité quant à son utilisation et aux résultats. Vérifiez toujours les résultats par des "
            "mesures de contrôle.",

    "d_room": "Pièce {r}",
    "d_sketch": "croquis : pièces numérotées, angles avec lettres (écrites par le bouton 3. Pièces)",
    "d_check": "B–D contrôle",
    "d_out1": "C–E : passe dehors,",
    "d_out2": "non mesurable",
    "d_from1b": "distance depuis 1B",
    "d_width": "0,90 largeur",
    "d_thick": "épaisseur 0,30 (dans le tableau)",
    "d_from2d": "1,10 distance depuis 2D",
    "d_furn": "meuble",
    "d_blocked": "A–C bloquée",

    "h1": "1. La méthode en bref",
    "p1": "On dessine le plan <b>à main levée</b>, on mesure sur place les côtés et quelques diagonales de chaque pièce, "
          "plus quelques mesures qui lient les pièces entre elles, et le plugin reconstruit le plan <b>réel</b> en "
          "compensant les mesures par les moindres carrés. Les résidus montrent aussitôt si une mesure est fausse.",
    "t1": [["", "Où", "Ce qu'on fait"],
           ["1", "au bureau", "croquis : on dessine chaque pièce comme une polyligne fermée (à main levée) et on écrit les lettres"],
           ["2", "sur place", "on mesure côtés, diagonales, épaisseurs de mur, portes et fenêtres, en les notant sur le croquis"],
           ["3", "au bureau", "on saisit les mesures, on calcule, on lit les résidus, on complète le dessin"]],
    "h_tools": "Matériel",
    "tools": [
        "télémètre laser (de préférence avec mesure depuis l'arrière de l'appareil) et un mètre ruban de 5 m pour "
        "les mesures courtes (épaisseurs, tableaux, décrochements) ;",
        "le croquis imprimé, crayon et gomme ; une planchette rigide ;",
        "un mètre pliant ou une équerre pour vérifier les angles quand les diagonales sont gênées ;",
        "appareil photo ou téléphone : une photo par pièce, avec les lettres des angles écrites au crayon sur le croquis.",
    ],

    "h2": "2. Avant la visite : le croquis dans la CAO",
    "p2a": "Dessinez <b>chaque pièce</b> comme une <b>polyligne fermée</b> (PLINE … Clore), en suivant la <b>face "
           "intérieure</b> des murs. Les dimensions ne comptent pas : ce qui compte, c'est que chaque angle réel soit "
           "un sommet de la polyligne, dans le bon ordre. Décrochements, niches, poteaux saillants et pans coupés "
           "sont aussi des sommets (pièce 3 de l'exemple). Le couloir est une pièce comme les autres.",
    "p2b": "Appuyez sur <b>3. Pièces : numéros, lettres et mesures</b> et sélectionnez les pièces — une à la fois dans l'ordre voulu pour les numéros, ou toutes ensemble avec une fenêtre (numérotées de haut en bas, de gauche à droite) — et appuyez sur Entrée : dans chaque pièce apparaît son numéro en gras et aux angles les lettres A, B, C … ; l'angle B de la pièce 2 s'appelle 2B. La fenêtre des mesures s'ouvre aussi : pour l'instant fermez-la avec <b>Fermer</b> et répondez de <b>garder</b> les lettres. Imprimez le plan : c'est votre croquis.",
    "t2": [["Conseil", "Pourquoi"],
           ["Numérotez les pièces dans un ordre que vous suivrez aussi sur place", "On écrit plus vite et on se trompe moins de pièce."],
           ["Polyligne sur la face intérieure", "Les mesures se prennent depuis l'intérieur des pièces ; les murs extérieurs se déduisent ensuite des épaisseurs."],
           ["Aucun sommet inutile", "Un sommet sur un côté droit (angle de 180°) demande des mesures inutiles."]],

    "h3": "3. Sur place : mesurer une pièce",
    "p3a": "Une pièce de <b>n</b> angles a 2n − 3 inconnues : elle se ferme avec <b>tous les côtés</b> plus <b>n − 3 "
           "diagonales</b>. Un rectangle demande 1 diagonale, une pièce en L 3, la salle de bains avec décrochement "
           "de l'exemple 3.",
    "p3b": "Mesurez les côtés <b>à la même hauteur</b> (par exemple à 1 m du sol, au-dessus des plinthes et des meubles "
           "bas), d'angle à angle. Puis les diagonales. Les diagonales proposées par le plugin passent toutes <b>à "
           "l'intérieur</b> de la pièce : dans une pièce en L, aucune diagonale ne traverse le mur.",
    "cap3": "À gauche : côtés (bleu) et diagonale (orange) d'une pièce hors d'équerre ; la seconde diagonale (vert) est "
            "la mesure de contrôle. À droite : pièce en L, les trois diagonales utiles et une à éviter.",
    "h_check": "La mesure de contrôle",
    "p_check": "Prenez <b>toujours une diagonale de plus</b> que le minimum. Sans mesures en plus, la forme est exacte "
               "quoi que vous saisissiez, et un chiffre mal lu (6,85 au lieu de 6,58) passe inaperçu. Avec une mesure "
               "en plus, le plugin calcule les <b>résidus</b> et σ₀ et colore en rouge les mesures hors tolérance "
               "(en général 1–2 cm).",
    "h_obst": "Quand un meuble empêche une diagonale",
    "obst": [
        "Essayez d'abord une autre diagonale (B–D au lieu de A–C), ou montez-la au-dessus du meuble en gardant le "
        "télémètre de niveau.",
        "Si ce n'est pas possible, utilisez une <b>contrainte</b> (dans la fiche de la pièce) : angle et valeur, par "
        "exemple <b>A, 90</b>, après l'avoir vérifié à l'équerre ; ou un côté <b>horizontal</b> ou <b>vertical</b>. "
        "Chaque contrainte vaut une mesure et reçoit elle aussi un résidu.",
        "S'il manque encore quelque chose, l'option <b>compléter la forme avec les angles du dessin</b> (activée en "
        "général) prend dans le dessin ce qui manque, et le tableau le signale en rouge. À cet endroit, le résultat "
        "vaut ce que vaut votre dessin à main levée : notez-le.",
    ],

    "h4": "4. Lier les pièces entre elles",
    "p4a": "Chaque pièce, mesurée seule, a la bonne forme mais ne sait pas où elle se trouve par rapport aux autres. "
           "Pour la lier, il faut <b>au moins 3 mesures</b> vers la pièce voisine, ou une <b>porte commune</b>. "
           "La porte est le moyen le plus pratique.",
    "cap4": "Une porte entre la pièce 1 et la pièce 2 : distance depuis l'angle et largeur mesurées de chaque côté, "
            "épaisseur du mur mesurée dans le tableau.",
    "p4b": "Dans la fenêtre des mesures, sous les pièces, appuyez sur <b>＋ Ajouter une porte ou une fenêtre</b> et <b>cliquez sur le dessin</b>, sur le mur entre les deux pièces, près de la porte. Le plugin trouve seul les deux côtés (il les surligne en vert) et demande :",
    "t4": [["Case", "Exemple"],
           ["Distance de l'angle 1B au tableau (mesurée dans la pièce 1)", "1,00"],
           ["Largeur", "0,90"],
           ["Épaisseur du mur, mesurée dans la porte", "0,30"],
           ["Distance de l'angle 2D au tableau (mesurée dans la pièce 2)", "1,10"]],
    "p4c": "La distance se prend de l'angle jusqu'au tableau ; le plugin propose l'angle le plus proche du point "
           "cliqué et vous pouvez choisir l'autre. Cela suffit : les deux pièces sont liées (tableaux en vis-à-vis, "
           "mur parallèle, épaisseur connue). Une mesure de plus à travers la porte (par exemple de 1B à 2D) sert de "
           "contrôle.",
    "p4d": "Sans porte (ou comme contrôle), utilisez la section <b>Autres mesures entre pièces</b>, en bas de la fenêtre :",
    "links": [
        "<b>épaisseur d'un mur</b> : De = un côté (1B-1C), À = l'angle de l'autre pièce derrière ce mur (2A), "
        "Mesurée = l'épaisseur prise d'équerre dans la porte ;",
        "<b>mesures à travers la porte</b> : De = 1B, À = 2D, entre des angles des deux pièces visibles depuis la "
        "porte ; prenez-en deux vers des angles différents.",
    ],
    "h_order": "Ordre conseillé pour le logement de l'exemple",
    "p_order": "Partez du <b>couloir</b> (pièce 4), qui touche presque toutes les autres, et liez chaque pièce au couloir "
               "par sa porte : séjour et chambre à gauche, cuisine et chambre 2 à droite ; la salle de bains se lie à "
               "la chambre. Ainsi chaque pièce tient à une chaîne courte et une erreur reste localisée.",

    "h_fix": "Périmètre fixe relevé à la station totale",
    "p_fix": "Si le périmètre extérieur du bâtiment a été relevé à la station totale (ou s'il est exact de toute façon), le plugin l'utilise comme référence fixe : il n'est jamais déplacé, et les pièces se placent d'elles-mêmes à l'intérieur.",
    "fix": ["dessinez le périmètre comme une <b>polyligne fermée</b>, en coordonnées réelles, et les pièces à main levée à l'intérieur ;", "appuyez sur <b>1. Périmètre extérieur</b>, cliquez la polyligne et appuyez sur Entrée (Entrée sans clic = pas de périmètre) ;", "appuyez sur <b>2. Épaisseurs des murs extérieurs</b> et cliquez les côtés du périmètre un par un : pour chacun saisissez l'épaisseur du mur (mesurée aux fenêtres) et choisissez le sens, <i>vers le centre</i> pour un mur extérieur normal (<i>vers l'extérieur</i> trace seulement la ligne, qui reste hors calcul). Dès que vous appuyez sur OK, la ligne parallèle et la valeur de ce côté apparaissent sur le dessin, sur le calque RILIEVO_PERIMETRO : un côté sans ligne est un côté oublié. Entrée pour finir : le récapitulatif liste les côtés restés sans épaisseur ;", "dans le calcul les côtés des pièces le long du périmètre restent parallèles à lui, à cette distance : une pièce d'angle avec deux côtés sur le périmètre n'a pas besoin de diagonales pour sa position ; une pièce avec un seul côté sur le périmètre doit encore être liée aux autres (par exemple par une porte)."],
    "h_hatch": "Murs pleins entre pièces : la hachure",
    "p_hatch": "Un mur plein ne peut pas être traversé pour mesurer, mais son épaisseur se mesure dans les portes. Si vous mettez une <b>hachure</b> dans les murs entre les pièces du dessin, le plugin les reconnaît seul :",
    "hatch": ["il trouve les deux côtés de pièces différentes qui se font face avec la hachure entre eux et les propose dans la section <b>Murs pleins entre pièces</b> de la fenêtre des mesures ;", "saisissez l'<b>épaisseur mesurée dans une porte</b> ; si le même mur a déjà une porte commune avec son épaisseur, la case peut rester vide ;", "dans le calcul, les deux côtés restent parallèles à cette distance, avec leur résidu dans le tableau."],
    "p_hatch2": "Un mur plein fixe la distance et la rotation entre deux pièces, mais pas le glissement le long du mur : il faut encore une porte dans le même mur, un second mur non parallèle vers la même pièce, ou le périmètre fixe.",

    "h5": "5. Portes, fenêtres et murs extérieurs",
    "h_dw": "Portes et fenêtres",
    "p_dw": "Les fenêtres et les portes vers l'extérieur s'ajoutent aussi avec <b>Ajouter une porte ou une fenêtre</b> "
            "et un clic sur le mur : type (<b>porte</b>, <b>fenêtre</b>, <b>ouverture</b>), distance de l'angle au "
            "tableau et largeur de passage. La porte est ferrée du côté de l'angle depuis lequel vous mesurez. Notez "
            "les hauteurs (allège, linteau) sur le croquis : le plugin travaille en plan.",
    "h_ext": "Murs extérieurs",
    "p_ext": "Le plugin reconstruit les pièces sur la face intérieure. Pour le périmètre extérieur, mesurez "
             "l'<b>épaisseur du mur aux fenêtres</b> (tableau compris), au moins une par façade ; puis, dans la CAO, "
             "tracez la face extérieure avec DECALER (OFFSET) depuis les côtés des pièces, à la distance mesurée, et "
             "raccordez les angles. Si possible, vérifiez avec une mesure extérieure (la longueur d'une façade) prise "
             "du dehors.",

    "h6": "6. Au bureau : calcul et contrôle",
    "calc": ["Appuyez sur <b>3. Pièces : numéros, lettres et mesures</b>, sélectionnez toutes les pièces (même ordre que le croquis) et appuyez sur Entrée. <b>Une seule fenêtre</b> s'ouvre avec toutes les pièces l'une sous l'autre, de la pièce 1 à la dernière, chacune avec son titre et son état : faites-la défiler avec la barre de droite et saisissez côtés et diagonales. Sous les pièces viennent portes et fenêtres, murs pleins, autres mesures entre pièces et options.", "Appuyez sur <b>▶ Calculer</b> : les polylignes des pièces sont amenées à la forme calculée et <b>le dessin se met à jour tout seul</b> (Ctrl+Z annule). Si les mesures ne suffisent pas ou ne concordent pas, la fenêtre se rouvre avec le motif et les mesures déjà saisies.", "Cotes, diagonales et tableau vont sur <b>RILIEVO_QUOTE</b> ; portes et fenêtres sur <b>RILIEVO_APERTURE</b>. Une fenêtre <b>Résultat du calcul</b> résume surfaces, mesures surabondantes et σ₀.", "Lisez le tableau des résidus : un résidu rouge signale la mesure à recontrôler. Avec une seule mesure surabondante l'erreur se répartit sur plusieurs mesures : en prendre une autre aide à la trouver.", "<b>Carnet de mesures</b> enregistre tout dans un CSV pour Excel ou Calc (mesures, résidus, coordonnées, surfaces)."],
    "h_vals": "Valeurs indicatives",
    "t6": [["Grandeur", "Valeur typique", "Si elle est plus élevée"],
           ["résidu d'un côté ou d'une diagonale", "moins de 1 cm", "remesurez ; vérifiez les chiffres inversés et le bon angle"],
           ["σ₀ d'une pièce", "0,3–0,8 cm", "mesures prises à des hauteurs différentes, angles peu nets"],
           ["angle contraint à 90°", "résidu inférieur à 0,2°", "l'angle n'est pas droit : retirez la contrainte et mesurez une diagonale"]],
    "h_list": "Liste de contrôle avant de quitter le chantier",
    "list": [
        "chaque pièce a tous ses côtés et au moins n − 3 diagonales (ou contraintes vérifiées), plus une mesure de contrôle ;",
        "chaque pièce est liée : porte commune avec épaisseur, ou 3 mesures vers une pièce déjà liée ;",
        "épaisseurs des murs extérieurs prises aux fenêtres, au moins une par façade ;",
        "portes et fenêtres : distance depuis l'angle, largeur, hauteurs notées ;",
        "une photo par pièce.",
    ],
    "ex_h": "7. Exemple complet, pas à pas",
    "ex_p": "Un logement de sept pièces, dans un bâtiment à la façade gauche inclinée, avec une aile à l'angle coupé. Le périmètre extérieur a été relevé à la station totale ; les murs intérieurs sont pleins et leur épaisseur ne se mesure que dans les portes.",
    "ex_fig": "périmètre E1–E7 relevé, pièces 1–7 dessinées à main levée, murs pleins hachurés",
    "ex_cap": "Le dessin de départ : périmètre fixe E1–E7, pièces numérotées avec les lettres des angles, murs remplis par la hachure, portes P1–P6 et fenêtres (en bleu).",
    "ex_ties": [["Pièce", "Côtés sur le périmètre", "Murs pleins (hachure)", "Portes"], ["1", "C–D, D–A", "A–B avec 4 ; B–C avec 2", "P5"], ["2", "C–D", "D–A avec 1 ; A–B avec 5 ; B–C avec 3", "P2"], ["3", "B–C, C–D, D–E", "A–B avec 6 ; E–A avec 2", "P6"], ["4", "A–B, D–A", "C–D avec 1 ; B–C avec 5", "P1, P5"], ["5", "A–B", "D–A avec 4 ; C–D avec 2 ; B–C avec 6 et 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A avec 5 ; A–B avec 7 ; C–D avec 3", "P3, P6"], ["7", "A–B, B–C", "C–D avec 6 ; D–A avec 5", "P4"]],
    "ex_ties_note": "Six pièces sur sept touchent le périmètre par deux côtés non parallèles : le périmètre à lui seul les met en place. La pièce 2 n'a qu'un côté sur le périmètre, mais le mur plein vers la pièce 1 (non parallèle) la bloque. Murs pleins et portes servent ainsi aussi de contrôle.",
    "ex_steps": [["Le périmètre à la station totale", "Relevez à la station totale les angles extérieurs du bâtiment E1…E7 et reliez-les en une polyligne fermée, en coordonnées réelles. C'est le seul élément exact du dessin : le plugin ne le déplacera jamais."], ["Les pièces à main levée", "À l'intérieur du périmètre, dessinez chaque pièce comme une polyligne fermée sur la face intérieure des murs. Seuls le nombre et l'ordre des angles comptent : la pièce 3 a cinq angles parce que le mur extérieur est coupé, les pièces 1 et 4 suivent la façade inclinée."], ["La hachure dans les murs pleins", "Avec la commande de hachure (HATCH), remplissez l'espace entre le périmètre et les pièces, c'est-à-dire tous les murs. La hachure indique au plugin quelles pièces sont séparées par un mur plein : inutile qu'elle soit précise, il suffit qu'elle soit entre les deux côtés."], ["Les lettres", "Appuyez sur <b>3. Pièces : numéros, lettres et mesures</b>, cliquez les pièces dans l'ordre 1…7 de la figure et appuyez sur Entrée. Numéros et lettres apparaissent aussitôt et la fenêtre des mesures s'ouvre : fermez-la et choisissez de garder les lettres. Imprimez le dessin : c'est le croquis à emporter sur place."], ["Sur place : les pièces", "Dans chaque pièce, mesurez les côtés à la même hauteur et au moins une diagonale. Les pièces avec deux côtés sur le périmètre n'ont pas besoin de diagonales pour leur forme, mais une diagonale de plus est le seul moyen de repérer un chiffre mal lu. Pour la pièce 1, par exemple :"], ["Sur place : épaisseurs et portes", "Mesurez l'épaisseur des murs extérieurs aux fenêtres (ici 0,40) et celle des murs intérieurs dans les portes (ici 0,30). Pour chaque porte, notez des deux côtés la distance de l'angle au tableau et la largeur :"], ["Boutons 1 et 2 : périmètre et épaisseurs", "Au bureau appuyez sur <b>1. Périmètre extérieur</b>, cliquez la polyligne E1…E7 et appuyez sur Entrée. Puis appuyez sur <b>2. Épaisseurs des murs extérieurs</b>, cliquez le premier côté, saisissez 0,40, laissez le sens <i>vers le centre</i> et cochez la case pour l'utiliser sur tous les côtés : après OK les lignes parallèles de tous les côtés apparaissent aussitôt. Entrée pour finir."], ["Bouton 3 : les pièces et la fenêtre des mesures", "Appuyez sur <b>3. Pièces : numéros, lettres et mesures</b>, cliquez les pièces dans le même ordre 1…7 et appuyez sur Entrée. La fenêtre s'ouvre avec les pièces 1 à 7, l'une sous l'autre ; plus bas la section <b>Murs pleins entre pièces (de la hachure)</b> avec les lignes déjà proposées : saisissez 0,30 là où il n'y a pas de porte commune, là où il y en a une, elle est prise de la porte."], ["Côtés, diagonales et portes", "Faites défiler la fenêtre et saisissez côtés et diagonales de chaque pièce. Puis appuyez sur <b>＋ Ajouter une porte ou une fenêtre</b> et cliquez le mur de chaque porte P1…P6 : le plugin trouve les deux côtés et demande distances, largeur et épaisseur, puis revient à la fenêtre. Les fenêtres s'ajoutent de la même façon, sur le mur extérieur."], ["Calculer", "Quand chaque pièce affiche ✔ et que les pièces sont liées, appuyez sur <b>▶ Calculer</b> : les polylignes des pièces prennent la forme calculée et le dessin se met à jour tout seul. La fenêtre <b>Résultat du calcul</b> résume mesures surabondantes et σ₀ ; dans le tableau les résidus rouges signalent les mesures à recontrôler. <b>Carnet de mesures</b> enregistre tout dans un CSV."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Mesure (pièce 1)", "Valeur", "Note"], ["1A–1B", "4,10", "côté"], ["1B–1C", "8,30", "côté"], ["1C–1D", "4,58", "côté sur le périmètre"], ["1D–1A", "8,31", "côté sur le périmètre (incliné)"], ["1A–1C", "9,26", "diagonale de contrôle"], ["1B–1D", "9,48", "seconde diagonale (facultative)"]],
    "ex_doors": [["Porte", "Première pièce : côté, distance", "Seconde pièce : côté, distance", "Largeur", "Épaisseur"], ["P1", "4B–4C, depuis 4B 1,60", "5D–5A, depuis 5A 1,60", "0,90", "0,30"], ["P2", "5C–5D, depuis 5D 1,30", "2A–2B, depuis 2A 1,30", "0,90", "0,30"], ["P3", "5B–5C, depuis 5B 5,00", "6D–6A, depuis 6A 1,10", "0,90", "0,30"], ["P4", "5B–5C, depuis 5B 1,00", "7D–7A, depuis 7A 1,00", "0,90", "0,30"], ["P5", "4C–4D, depuis 4C 1,10", "1A–1B, depuis 1B 1,10", "0,90", "0,30"], ["P6", "6C–6D, depuis 6D 1,30", "3A–3B, depuis 3A 1,30", "0,80", "0,30"]],
    "ex_end": "Avec le périmètre relevé une seule fois, aucune mesure ne traverse les murs : côtés et diagonales se prennent dans les pièces, les épaisseurs dans les portes et les fenêtres.",
    "end": "Room Survey est un projet amateur, non adapté à un usage professionnel : l'auteur décline toute "
           "responsabilité quant à son utilisation. Code et mises à jour : github.com/Macchecaspita/opencad-rilievo-plugin",
}
