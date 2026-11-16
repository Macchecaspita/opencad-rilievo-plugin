"""Textos del manual — español."""

OUT = "manual_es.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — manual de levantamiento",
    "foot": "Room Survey — manual de levantamiento · proyecto aficionado, no para uso profesional",
    "subtitle": "Cómo levantar una vivienda con medidas directas<br/>y reconstruirla en Open CAD Studio",
    "warn": "<b>Aviso.</b> Room Survey es un proyecto aficionado, hecho por diversión y <b>no apto para uso "
            "profesional</b>. Se ofrece tal cual, sin ninguna garantía; el autor no asume ninguna responsabilidad por "
            "su uso ni por los resultados. Compruebe siempre los resultados con medidas de control.",

    "d_room": "Estancia {r}",
    "d_sketch": "croquis: estancias numeradas, esquinas con letras (escritas por el botón 3. Estancias)",
    "d_check": "B–D control",
    "d_out1": "C–E: pasa por fuera,",
    "d_out2": "no se puede medir",
    "d_from1b": "distancia desde 1B",
    "d_width": "0,90 anchura",
    "d_thick": "espesor 0,30 (en la jamba)",
    "d_from2d": "1,10 distancia desde 2D",
    "d_furn": "mueble",
    "d_blocked": "A–C bloqueada",

    "h1": "1. El método en breve",
    "p1": "Se dibuja la planta <b>a ojo</b>, se miden en obra los lados y algunas diagonales de cada estancia, más "
          "algunas medidas que enlazan las estancias entre sí, y el plugin reconstruye la planta <b>real</b> "
          "compensando las medidas por mínimos cuadrados. Los residuos dicen enseguida si una medida está mal.",
    "t1": [["", "Dónde", "Qué se hace"],
           ["1", "en el estudio", "croquis: se dibuja cada estancia como polilínea cerrada (a ojo) y se escriben las letras"],
           ["2", "en obra", "se miden lados, diagonales, espesores de muro, puertas y ventanas, anotándolos en el croquis"],
           ["3", "en el estudio", "se introducen las medidas, se calcula, se leen los residuos, se completa el dibujo"]],
    "h_tools": "Equipo",
    "tools": [
        "medidor láser (mejor si mide desde la parte trasera del aparato) y una cinta métrica de 5 m para las "
        "medidas cortas (espesores, mochetas, retranqueos);",
        "el croquis impreso, lápiz y goma; una tablilla rígida;",
        "un metro plegable o una escuadra para comprobar los ángulos cuando las diagonales están impedidas;",
        "cámara o teléfono: una foto por estancia, con las letras de las esquinas escritas a lápiz en el croquis.",
    ],

    "h2": "2. Antes de la visita: el croquis en el CAD",
    "p2a": "Dibuje <b>cada estancia</b> como <b>polilínea cerrada</b> (PLINE … Cerrar), siguiendo la <b>cara "
           "interior</b> de los muros. Las medidas no importan: importa que cada esquina real sea un vértice de la "
           "polilínea, en el orden correcto. Retranqueos, hornacinas, pilares salientes y chaflanes también son "
           "vértices (estancia 3 del ejemplo). El pasillo es una estancia como las demás.",
    "p2b": "Pulse <b>3. Estancias: números, letras y medidas</b> y seleccione las estancias — una a una en el orden que quiera dar a los números, o todas juntas con una ventana (numeradas de arriba abajo, de izquierda a derecha) — y pulse Intro: en cada estancia aparece su número en negrita y en las esquinas las letras A, B, C …; la esquina B de la estancia 2 se llama 2B. Se abre también la ventana de medidas: por ahora ciérrela con <b>Cerrar</b> y responda que quiere <b>conservar</b> las letras. Imprima la planta: es su croquis.",
    "t2": [["Consejo", "Por qué"],
           ["Numere las estancias en un orden que también siga en obra", "Se escribe más rápido y se confunde menos de estancia."],
           ["Polilínea en la cara interior", "Las medidas se toman desde dentro de las estancias; los muros exteriores se obtienen después de los espesores."],
           ["Ningún vértice inútil", "Un vértice en un lado recto (ángulo de 180°) pide medidas inútiles."]],

    "h3": "3. En obra: medir una estancia",
    "p3a": "Una estancia de <b>n</b> esquinas tiene 2n − 3 incógnitas: se cierra con <b>todos los lados</b> más "
           "<b>n − 3 diagonales</b>. Un rectángulo necesita 1 diagonal, una estancia en L 3, el baño con retranqueo "
           "del ejemplo 3.",
    "p3b": "Mida los lados <b>a la misma altura</b> (por ejemplo a 1 m del suelo, por encima de rodapiés y muebles "
           "bajos), de esquina a esquina. Después las diagonales. Las diagonales que propone el plugin pasan todas "
           "<b>por dentro</b> de la estancia: en una estancia en L nunca hay una diagonal que atraviese el muro.",
    "cap3": "Izquierda: lados (azul) y diagonal (naranja) de una estancia fuera de escuadra; la segunda diagonal "
            "(verde) es la medida de control. Derecha: estancia en L, las tres diagonales útiles y una que evitar.",
    "h_check": "La medida de control",
    "p_check": "Tome <b>siempre una diagonal más</b> del mínimo. Sin medidas de más la forma es exacta escriba lo que "
               "escriba, y una cifra mal leída (6,85 en lugar de 6,58) pasa desapercibida. Con una medida de más el "
               "plugin calcula los <b>residuos</b> y σ₀ y colorea en rojo las medidas fuera de tolerancia (normalmente "
               "1–2 cm).",
    "h_obst": "Cuando un mueble impide una diagonal",
    "obst": [
        "Pruebe primero otra diagonal (B–D en lugar de A–C), o súbala por encima del mueble manteniendo el medidor "
        "nivelado.",
        "Si no se puede, use una <b>restricción</b> (en la ficha de la estancia): esquina y ángulo, por ejemplo "
        "<b>A, 90</b>, tras comprobarlo con la escuadra; o un lado <b>horizontal</b> o <b>vertical</b>. Cada "
        "restricción vale una medida y también recibe un residuo.",
        "Si aún falta algo, la opción <b>completar la forma con los ángulos del dibujo</b> (normalmente activa) toma "
        "del dibujo lo que falta, y la tabla lo señala en rojo. En ese punto el resultado vale lo que su dibujo a "
        "ojo: anótelo.",
    ],

    "h4": "4. Enlazar las estancias entre sí",
    "p4a": "Cada estancia, medida sola, tiene la forma correcta pero no sabe dónde está respecto a las demás. Para "
           "enlazarla hacen falta <b>al menos 3 medidas</b> hacia la estancia vecina, o una <b>puerta común</b>. "
           "La forma más cómoda es la puerta.",
    "cap4": "Una puerta entre la estancia 1 y la estancia 2: distancia desde la esquina y anchura medidas desde cada "
            "lado, espesor del muro medido en la jamba.",
    "p4b": "En la ventana de medidas, debajo de las estancias, pulse <b>＋ Añadir puerta o ventana</b> y <b>haga clic en el dibujo</b>, sobre el muro entre las dos estancias, cerca de la puerta. El plugin encuentra solo los dos lados (los resalta en verde) y pide:",
    "t4": [["Casilla", "Ejemplo"],
           ["Distancia de la esquina 1B a la jamba (medida en la estancia 1)", "1,00"],
           ["Anchura", "0,90"],
           ["Espesor del muro, medido en la puerta", "0,30"],
           ["Distancia de la esquina 2D a la jamba (medida en la estancia 2)", "1,10"]],
    "p4c": "La distancia se toma desde la esquina hasta la jamba; el plugin propone la esquina más cercana al punto "
           "pulsado y puede elegir la otra. Con eso basta: las dos estancias quedan enlazadas (jambas enfrentadas, "
           "muro paralelo, espesor conocido). Una medida más a través de la puerta (por ejemplo de 1B a 2D) sirve "
           "de control.",
    "p4d": "Sin puerta (o como control) use la sección <b>Otras medidas entre estancias</b>, al final de la ventana:",
    "links": [
        "<b>espesor de un muro</b>: Desde = un lado (1B-1C), Hasta = la esquina de la otra estancia detrás de ese "
        "muro (2A), Medida = el espesor tomado en escuadra en la puerta;",
        "<b>medidas a través de la puerta</b>: Desde = 1B, Hasta = 2D, entre esquinas de las dos estancias que se "
        "ven desde la puerta; tome dos hacia esquinas distintas.",
    ],
    "h_order": "Orden recomendado para la vivienda del ejemplo",
    "p_order": "Empiece por el <b>pasillo</b> (estancia 4), que toca casi todas las demás, y enlace cada estancia al "
               "pasillo con su puerta: salón y dormitorio a la izquierda, cocina y dormitorio 2 a la derecha; el baño "
               "se enlaza al dormitorio. Así cada estancia queda unida por una cadena corta y un error queda localizado.",

    "h_fix": "Perímetro fijo levantado con la estación total",
    "p_fix": "Si el perímetro exterior del edificio se ha levantado con la estación total (o es exacto de todos modos), el plugin lo usa como referencia fija: nunca se mueve, y las estancias se colocan solas dentro de él.",
    "fix": ["dibuje el perímetro como <b>polilínea cerrada</b>, en coordenadas reales, y las estancias a ojo en su interior;", "pulse <b>1. Perímetro exterior</b>, haga clic en la polilínea y pulse Intro (Intro sin clic = sin perímetro);", "pulse <b>2. Espesores de muros exteriores</b> y haga clic en los lados del perímetro uno a uno: para cada uno escriba el espesor del muro hacia el interior (medido en las ventanas). Los lados con espesor se vuelven verdes; los espesores quedan escritos en el dibujo, en la capa RILIEVO_PERIMETRO. Pulse Intro para terminar;", "en el cálculo los lados de las estancias a lo largo del perímetro quedan paralelos a él, a esa distancia: una estancia de esquina con dos lados en el perímetro no necesita diagonales para su posición; una estancia con un solo lado en el perímetro debe enlazarse aún con las demás (por ejemplo con una puerta)."],
    "h_hatch": "Muros macizos entre estancias: el sombreado",
    "p_hatch": "Un muro macizo no se puede atravesar para medir, pero su espesor se mide en las puertas. Si en el dibujo pone un <b>sombreado</b> en los muros entre las estancias, el plugin los reconoce solo:",
    "hatch": ["encuentra los dos lados de estancias distintas que se miran con el sombreado en medio y los propone en la sección <b>Muros macizos entre estancias</b> de la ventana de medidas;", "escriba el <b>espesor medido en una puerta</b>; si en el mismo muro ya hay una puerta común con su espesor, la casilla puede quedar vacía;", "en el cálculo los dos lados se mantienen paralelos a esa distancia, con su residuo en la tabla."],
    "p_hatch2": "Un muro macizo fija la distancia y el giro entre dos estancias, pero no el deslizamiento a lo largo del muro: aún hace falta una puerta en el mismo muro, un segundo muro no paralelo hacia la misma estancia, o el perímetro fijo.",

    "h5": "5. Puertas, ventanas y muros exteriores",
    "h_dw": "Puertas y ventanas",
    "p_dw": "También las ventanas y las puertas al exterior se añaden con <b>Añadir puerta o ventana</b> y un clic en "
            "el muro: tipo (<b>puerta</b>, <b>ventana</b>, <b>abertura</b>), distancia de la esquina a la mocheta y "
            "anchura de paso. La puerta va abisagrada del lado de la esquina desde la que mide. Anote las alturas "
            "(alféizar, dintel) en el croquis: el plugin trabaja en planta.",
    "h_ext": "Muros exteriores",
    "p_ext": "El plugin reconstruye las estancias en la cara interior. Para el perímetro exterior mida el <b>espesor "
             "del muro en las ventanas</b> (mocheta incluida), al menos uno por fachada; después, en el CAD, obtenga "
             "la cara exterior con DESFASE (OFFSET) desde los lados de las estancias, a la distancia medida, y una las "
             "esquinas. Donde sea posible compruébelo con una medida exterior (la longitud de una fachada) tomada "
             "desde fuera.",

    "h6": "6. En el estudio: cálculo y control",
    "calc": ["Pulse <b>3. Estancias: números, letras y medidas</b>, seleccione todas las estancias (mismo orden que el croquis) y pulse Intro. Se abre <b>una sola ventana</b> con todas las estancias una debajo de otra, de la 1 a la última, cada una con su título y su estado: desplácela con la barra de la derecha y escriba lados y diagonales. Debajo de las estancias están puertas y ventanas, muros macizos, otras medidas entre estancias y opciones.", "Pulse <b>▶ Calcular</b>: las polilíneas de las estancias se llevan a la forma calculada y <b>el dibujo se actualiza solo</b> (Ctrl+Z deshace). Si las medidas no bastan o no cuadran, la ventana se vuelve a abrir con el motivo y las medidas ya escritas.", "Cotas, diagonales y tabla van a <b>RILIEVO_QUOTE</b>; puertas y ventanas a <b>RILIEVO_APERTURE</b>. Una ventana <b>Resultado del cálculo</b> resume superficies, medidas sobrantes y σ₀.", "Lea la tabla de residuos: un residuo en rojo indica la medida que hay que volver a comprobar. Con una sola medida sobrante el error se reparte entre varias medidas: tomar otra ayuda a encontrarlo.", "<b>Libro de medidas</b> guarda todo en un CSV para Excel o Calc (medidas, residuos, coordenadas, superficies)."],
    "h_vals": "Valores orientativos",
    "t6": [["Magnitud", "Valor típico", "Si es mayor"],
           ["residuo de un lado o una diagonal", "hasta 1 cm", "vuelva a medir; compruebe cifras invertidas y la esquina correcta"],
           ["σ₀ de una estancia", "0,3–0,8 cm", "medidas tomadas a alturas distintas, esquinas poco definidas"],
           ["ángulo fijado a 90°", "residuo hasta 0,2°", "el ángulo no es recto: quite la restricción y mida una diagonal"]],
    "h_list": "Lista de comprobación antes de salir de la obra",
    "list": [
        "cada estancia tiene todos los lados y al menos n − 3 diagonales (o restricciones comprobadas), más una medida "
        "de control;",
        "cada estancia está enlazada: puerta común con espesor, o 3 medidas hacia una estancia ya enlazada;",
        "espesores de los muros exteriores tomados en las ventanas, al menos uno por fachada;",
        "puertas y ventanas: distancia desde la esquina, anchura, alturas anotadas;",
        "una foto por estancia.",
    ],
    "ex_h": "7. Ejemplo completo, paso a paso",
    "ex_p": "Una vivienda de siete estancias, en un edificio con la fachada izquierda inclinada y un ala con la esquina achaflanada. El perímetro exterior se ha levantado con la estación total; los muros interiores son macizos y su espesor solo se puede medir en las puertas.",
    "ex_fig": "perímetro E1–E7 levantado, estancias 1–7 dibujadas a ojo, muros macizos con sombreado",
    "ex_cap": "El dibujo de partida: perímetro fijo E1–E7, estancias numeradas con las letras de las esquinas, muros rellenos con el sombreado, puertas P1–P6 y ventanas (en azul).",
    "ex_ties": [["Estancia", "Lados sobre el perímetro", "Muros macizos (sombreado)", "Puertas"], ["1", "C–D, D–A", "A–B con 4; B–C con 2", "P5"], ["2", "C–D", "D–A con 1; A–B con 5; B–C con 3", "P2"], ["3", "B–C, C–D, D–E", "A–B con 6; E–A con 2", "P6"], ["4", "A–B, D–A", "C–D con 1; B–C con 5", "P1, P5"], ["5", "A–B", "D–A con 4; C–D con 2; B–C con 6 y 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A con 5; A–B con 7; C–D con 3", "P3, P6"], ["7", "A–B, B–C", "C–D con 6; D–A con 5", "P4"]],
    "ex_ties_note": "Seis estancias de siete tocan el perímetro con dos lados no paralelos: el perímetro solo ya las coloca en su sitio. La estancia 2 tiene un solo lado sobre el perímetro, pero el muro macizo hacia la estancia 1 (no paralelo) la bloquea. Así, muros macizos y puertas sirven también de control.",
    "ex_steps": [["El perímetro de la estación total", "Levante con la estación total las esquinas exteriores del edificio E1…E7 y únalas en una polilínea cerrada, en coordenadas reales. Es el único elemento exacto del dibujo: el plugin nunca lo moverá."], ["Las estancias a ojo", "Dentro del perímetro dibuje cada estancia como polilínea cerrada en la cara interior de los muros. Solo importan el número y el orden de las esquinas: la estancia 3 tiene cinco esquinas porque el muro exterior está achaflanado, las estancias 1 y 4 siguen la fachada inclinada."], ["El sombreado en los muros macizos", "Con el comando de sombreado (HATCH) rellene el espacio entre el perímetro y las estancias, es decir todos los muros. El sombreado indica al plugin qué estancias están separadas por un muro macizo: no hace falta que sea preciso, basta con que esté entre los dos lados."], ["Las letras", "Pulse <b>3. Estancias: números, letras y medidas</b>, haga clic en las estancias en el orden 1…7 de la figura y pulse Intro. Números y letras aparecen enseguida y se abre la ventana de medidas: ciérrela y elija conservar las letras. Imprima el dibujo: es el croquis para la obra."], ["En obra: las estancias", "En cada estancia mida los lados a la misma altura y al menos una diagonal. A las estancias con dos lados sobre el perímetro las diagonales no les hacen falta para la forma, pero una diagonal más es la única manera de notar una cifra mal leída. Para la estancia 1, por ejemplo:"], ["En obra: espesores y puertas", "Mida el espesor de los muros exteriores en las ventanas (aquí 0,40) y el de los muros interiores en las puertas (aquí 0,30). Para cada puerta anote, por los dos lados, la distancia de la esquina a la jamba y la anchura:"], ["Botones 1 y 2: perímetro y espesores", "En el despacho pulse <b>1. Perímetro exterior</b>, haga clic en la polilínea E1…E7 y pulse Intro. Después pulse <b>2. Espesores de muros exteriores</b>, haga clic en el primer lado, escriba 0,40 y marque la casilla para usarlo en todos los lados: todos se vuelven verdes. Intro para terminar."], ["Botón 3: las estancias y la ventana de medidas", "Pulse <b>3. Estancias: números, letras y medidas</b>, haga clic en las estancias en el mismo orden 1…7 y pulse Intro. Se abre la ventana con las estancias de la 1 a la 7, una debajo de otra; más abajo la sección <b>Muros macizos entre estancias (del sombreado)</b> con las filas ya propuestas: escriba 0,30 donde no hay puerta común, donde la hay lo toma de la puerta."], ["Lados, diagonales y puertas", "Desplace la ventana y escriba lados y diagonales de cada estancia. Después pulse <b>＋ Añadir puerta o ventana</b> y haga clic en el muro de cada puerta P1…P6: el plugin encuentra los dos lados y pide distancias, anchura y espesor, y vuelve a la ventana. Las ventanas se añaden igual, sobre el muro exterior."], ["Calcular", "Cuando cada estancia muestra ✔ y las estancias están enlazadas, pulse <b>▶ Calcular</b>: las polilíneas de las estancias toman la forma calculada y el dibujo se actualiza solo. La ventana <b>Resultado del cálculo</b> resume medidas sobrantes y σ₀; en la tabla los residuos en rojo indican las medidas que hay que comprobar. <b>Libro de medidas</b> guarda todo en un CSV."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Medida (estancia 1)", "Valor", "Nota"], ["1A–1B", "4,10", "lado"], ["1B–1C", "8,30", "lado"], ["1C–1D", "4,58", "lado sobre el perímetro"], ["1D–1A", "8,31", "lado sobre el perímetro (inclinado)"], ["1A–1C", "9,26", "diagonal de control"], ["1B–1D", "9,48", "segunda diagonal (opcional)"]],
    "ex_doors": [["Puerta", "Primera estancia: lado, distancia", "Segunda estancia: lado, distancia", "Anchura", "Espesor"], ["P1", "4B–4C, desde 4B 1,60", "5D–5A, desde 5A 1,60", "0,90", "0,30"], ["P2", "5C–5D, desde 5D 1,30", "2A–2B, desde 2A 1,30", "0,90", "0,30"], ["P3", "5B–5C, desde 5B 5,00", "6D–6A, desde 6A 1,10", "0,90", "0,30"], ["P4", "5B–5C, desde 5B 1,00", "7D–7A, desde 7A 1,00", "0,90", "0,30"], ["P5", "4C–4D, desde 4C 1,10", "1A–1B, desde 1B 1,10", "0,90", "0,30"], ["P6", "6C–6D, desde 6D 1,30", "3A–3B, desde 3A 1,30", "0,80", "0,30"]],
    "ex_end": "Con el perímetro levantado una sola vez, ninguna medida atraviesa los muros: lados y diagonales se toman dentro de las estancias, los espesores en las puertas y ventanas.",
    "end": "Room Survey es un proyecto aficionado, no apto para uso profesional: el autor no asume responsabilidad "
           "por su uso. Código y actualizaciones: github.com/Macchecaspita/opencad-rilievo-plugin",
}
