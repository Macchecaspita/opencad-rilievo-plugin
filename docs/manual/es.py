"""Textos del manual — español."""

OUT = "manual_es.pdf"
DEC = ","
S = {
    "doc_title": "Rilievo Vani — manual de levantamiento",
    "foot": "Rilievo Vani — manual de levantamiento · proyecto aficionado, no para uso profesional",
    "subtitle": "Cómo levantar una vivienda con medidas directas<br/>y reconstruirla en Open CAD Studio",
    "warn": "<b>Aviso.</b> Rilievo Vani es un proyecto aficionado, hecho por diversión y <b>no apto para uso "
            "profesional</b>. Se ofrece tal cual, sin ninguna garantía; el autor no asume ninguna responsabilidad por "
            "su uso ni por los resultados. Compruebe siempre los resultados con medidas de control.",

    "d_room": "Estancia {r}",
    "d_sketch": "croquis: estancias numeradas, esquinas con letras (escritas por Letras de esquinas)",
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
    "p2b": "Con <b>Letras de esquinas</b> seleccione las estancias — una a una en el orden que quiera dar a los "
           "números, o todas juntas con una ventana (numeradas de arriba abajo, de izquierda a derecha) — y pulse "
           "Intro: en las esquinas aparecen las letras A, B, C … y en cada estancia su número y su superficie; la "
           "esquina B de la estancia 2 se llama 2B. Una ventanita pide la altura de las letras: hágalas más pequeñas "
           "donde las esquinas están juntas. Imprima la planta: es su croquis.",
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
    "p4b": "En la lista de la ventana elija <b>Añadir puerta o ventana</b> y <b>haga clic en el dibujo</b>, sobre el "
           "muro entre las dos estancias, cerca de la puerta. El plugin reconoce por sí solo los dos lados (los "
           "resalta en verde) y pide:",
    "t4": [["Casilla", "Ejemplo"],
           ["Distancia de la esquina 1B a la jamba (medida en la estancia 1)", "1,00"],
           ["Anchura", "0,90"],
           ["Espesor del muro, medido en la puerta", "0,30"],
           ["Distancia de la esquina 2D a la jamba (medida en la estancia 2)", "1,10"]],
    "p4c": "La distancia se toma desde la esquina hasta la jamba; el plugin propone la esquina más cercana al punto "
           "pulsado y puede elegir la otra. Con eso basta: las dos estancias quedan enlazadas (jambas enfrentadas, "
           "muro paralelo, espesor conocido). Una medida más a través de la puerta (por ejemplo de 1B a 2D) sirve "
           "de control.",
    "p4d": "Sin puerta (o como control) elija en la lista <b>Otras medidas entre estancias</b>:",
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
    "calc": [
        "<b>Lados y diagonales</b>: haga clic en todas las estancias (mismo orden que el croquis) y pulse Intro. Se "
        "abre la lista de estancias con su estado: abra una estancia cada vez y escriba lados y diagonales, añada las "
        "puertas con un clic en el dibujo; cuando todo está completo la lista propone <b>Calcular</b>.",
        "Si las medidas no bastan o no encajan, la ventana se vuelve a abrir con el motivo y con las medidas ya escritas.",
        "El resultado va a la capa <b>RILIEVO_CORRETTO</b>, con la superficie de cada estancia; cotas, diagonales y "
        "tabla en <b>RILIEVO_QUOTE</b>; puertas y ventanas en <b>RILIEVO_APERTURE</b>. El dibujo a ojo se queda para "
        "comparar. <b>Aplicar al dibujo</b> lleva después las polilíneas originales a la forma calculada.",
        "Lea la tabla de residuos: un residuo en rojo indica la medida que hay que volver a comprobar. Con una sola "
        "medida de más el error se reparte entre varias medidas: tomar otra ayuda a encontrarlo.",
        "<b>Libro de medidas</b> guarda todo en un CSV para Excel o Calc (medidas, residuos, coordenadas, superficies).",
    ],
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
    "end": "Rilievo Vani es un proyecto aficionado, no apto para uso profesional: el autor no asume responsabilidad "
           "por su uso. Código y actualizaciones: github.com/Macchecaspita/opencad-rilievo-plugin",
}
