"""Textos do manual — português."""

OUT = "manual_pt.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — manual de levantamento",
    "foot": "Room Survey — manual de levantamento · projeto amador, não para uso profissional",
    "subtitle": "Como levantar uma habitação com medidas diretas<br/>e reconstruí-la no Open CAD Studio",
    "warn": "<b>Aviso.</b> O Room Survey é um projeto amador, feito por diversão e <b>não adequado a uso "
            "profissional</b>. É fornecido tal como está, sem qualquer garantia; o autor não assume qualquer "
            "responsabilidade pelo seu uso nem pelos resultados. Verifique sempre os resultados com medidas de controlo.",

    "d_room": "Compartimento {r}",
    "d_sketch": "croqui: compartimentos numerados, cantos com letras (escritas por Letras dos cantos)",
    "d_check": "B–D controlo",
    "d_out1": "C–E: passa por fora,",
    "d_out2": "não mensurável",
    "d_from1b": "distância a partir de 1B",
    "d_width": "0,90 largura",
    "d_thick": "espessura 0,30 (na ombreira)",
    "d_from2d": "1,10 distância a partir de 2D",
    "d_furn": "móvel",
    "d_blocked": "A–C bloqueada",

    "h1": "1. O método em resumo",
    "p1": "Desenha-se a planta <b>à mão</b>, medem-se na obra os lados e algumas diagonais de cada compartimento, mais "
          "algumas medidas que ligam os compartimentos entre si, e o plugin reconstrói a planta <b>real</b> "
          "compensando as medidas pelos mínimos quadrados. Os resíduos dizem logo se uma medida está errada.",
    "t1": [["", "Onde", "O que se faz"],
           ["1", "no escritório", "croqui: desenha-se cada compartimento como polilinha fechada (à mão) e escrevem-se as letras"],
           ["2", "na obra", "medem-se lados, diagonais, espessuras de paredes, portas e janelas, anotando-os no croqui"],
           ["3", "no escritório", "introduzem-se as medidas, calcula-se, leem-se os resíduos, completa-se o desenho"]],
    "h_tools": "Equipamento",
    "tools": [
        "medidor laser (de preferência com medição a partir da traseira do aparelho) e uma fita métrica de 5 m para "
        "as medidas curtas (espessuras, ombreiras, reentrâncias);",
        "o croqui impresso, lápis e borracha; uma prancheta rígida;",
        "um metro articulado ou um esquadro para verificar os ângulos quando as diagonais estão impedidas;",
        "máquina fotográfica ou telemóvel: uma foto por compartimento, com as letras dos cantos escritas a lápis no croqui.",
    ],

    "h2": "2. Antes da visita: o croqui no CAD",
    "p2a": "Desenhe <b>cada compartimento</b> como <b>polilinha fechada</b> (PLINE … Fechar), seguindo a <b>face "
           "interior</b> das paredes. As medidas não importam: importa que cada canto real seja um vértice da "
           "polilinha, pela ordem certa. Reentrâncias, nichos, pilares salientes e chanfros também são vértices "
           "(compartimento 3 do exemplo). O corredor é um compartimento como os outros.",
    "p2b": "Com <b>Letras dos cantos</b> selecione os compartimentos — um de cada vez pela ordem que quer dar aos "
           "números, ou todos juntos com uma janela (numerados de cima para baixo, da esquerda para a direita) — e "
           "prima Enter: nos cantos aparecem as letras A, B, C … e em cada compartimento o seu número e a sua área; o "
           "canto B do compartimento 2 chama-se 2B. Uma janelinha pede a altura das letras: reduza-a onde os cantos "
           "estão próximos. Imprima a planta: é o seu croqui.",
    "t2": [["Conselho", "Porquê"],
           ["Numere os compartimentos por uma ordem que também siga na obra", "Escreve-se mais depressa e troca-se menos de compartimento."],
           ["Polilinha na face interior", "As medidas tiram-se de dentro dos compartimentos; as paredes exteriores obtêm-se depois das espessuras."],
           ["Nenhum vértice inútil", "Um vértice num lado reto (ângulo de 180°) pede medidas inúteis."]],

    "h3": "3. Na obra: medir um compartimento",
    "p3a": "Um compartimento de <b>n</b> cantos tem 2n − 3 incógnitas: fecha-se com <b>todos os lados</b> mais <b>n − 3 "
           "diagonais</b>. Um retângulo precisa de 1 diagonal, um compartimento em L de 3, a casa de banho com "
           "reentrância do exemplo de 3.",
    "p3b": "Meça os lados <b>à mesma altura</b> (por exemplo a 1 m do chão, acima dos rodapés e dos móveis baixos), de "
           "canto a canto. Depois as diagonais. As diagonais propostas pelo plugin passam todas <b>por dentro</b> do "
           "compartimento: num compartimento em L nunca há uma diagonal que atravesse a parede.",
    "cap3": "À esquerda: lados (azul) e diagonal (laranja) de um compartimento fora de esquadria; a segunda diagonal "
            "(verde) é a medida de controlo. À direita: compartimento em L, as três diagonais úteis e uma a evitar.",
    "h_check": "A medida de controlo",
    "p_check": "Tire <b>sempre uma diagonal a mais</b> do que o mínimo. Sem medidas a mais a forma é exata seja o que "
               "for que escreva, e um algarismo mal lido (6,85 em vez de 6,58) passa despercebido. Com uma medida a "
               "mais o plugin calcula os <b>resíduos</b> e σ₀ e pinta de vermelho as medidas fora da tolerância "
               "(normalmente 1–2 cm).",
    "h_obst": "Quando um móvel impede uma diagonal",
    "obst": [
        "Experimente primeiro outra diagonal (B–D em vez de A–C), ou suba-a por cima do móvel mantendo o medidor nivelado.",
        "Se não for possível, use uma <b>restrição</b> (na ficha do compartimento): canto e ângulo, por exemplo "
        "<b>A, 90</b>, depois de o verificar com o esquadro; ou um lado <b>horizontal</b> ou <b>vertical</b>. Cada "
        "restrição vale uma medida e também recebe um resíduo.",
        "Se ainda faltar alguma coisa, a opção <b>completar a forma com os ângulos do desenho</b> (normalmente ativa) "
        "tira do desenho o que falta, e a tabela assinala-o a vermelho. Nesse ponto o resultado vale o que vale o seu "
        "desenho à mão: anote-o.",
    ],

    "h4": "4. Ligar os compartimentos entre si",
    "p4a": "Cada compartimento, medido sozinho, tem a forma certa mas não sabe onde está em relação aos outros. Para o "
           "ligar são precisas <b>pelo menos 3 medidas</b> para o compartimento vizinho, ou uma <b>porta comum</b>. "
           "A forma mais cómoda é a porta.",
    "cap4": "Uma porta entre o compartimento 1 e o compartimento 2: distância a partir do canto e largura medidas de "
            "cada lado, espessura da parede medida na ombreira.",
    "p4b": "Na lista da janela escolha <b>Acrescentar porta ou janela</b> e <b>clique no desenho</b>, sobre a parede "
           "entre os dois compartimentos, perto da porta. O plugin reconhece sozinho os dois lados (realça-os a verde) "
           "e pede:",
    "t4": [["Caixa", "Exemplo"],
           ["Distância do canto 1B à ombreira (medida no compartimento 1)", "1,00"],
           ["Largura", "0,90"],
           ["Espessura da parede, medida na porta", "0,30"],
           ["Distância do canto 2D à ombreira (medida no compartimento 2)", "1,10"]],
    "p4c": "A distância toma-se do canto até à ombreira; o plugin propõe o canto mais próximo do ponto clicado e pode "
           "escolher o outro. Basta isto: os dois compartimentos ficam ligados (ombreiras frente a frente, parede "
           "paralela, espessura conhecida). Uma medida a mais através da porta (por exemplo de 1B a 2D) serve de controlo.",
    "p4d": "Sem porta (ou como controlo) escolha na lista <b>Outras medidas entre compartimentos</b>:",
    "links": [
        "<b>espessura de uma parede</b>: De = um lado (1B-1C), Até = o canto do outro compartimento atrás dessa parede "
        "(2A), Medida = a espessura tirada em esquadria na porta;",
        "<b>medidas através da porta</b>: De = 1B, Até = 2D, entre cantos dos dois compartimentos que se veem da porta; "
        "tire duas para cantos diferentes.",
    ],
    "h_order": "Ordem recomendada para a habitação do exemplo",
    "p_order": "Comece pelo <b>corredor</b> (compartimento 4), que toca quase todos os outros, e ligue cada compartimento "
               "ao corredor pela sua porta: sala e quarto à esquerda, cozinha e quarto 2 à direita; a casa de banho "
               "liga-se ao quarto. Assim cada compartimento fica preso a uma cadeia curta e um erro fica localizado.",

    "h_fix": "Perímetro fixo levantado com a estação total",
    "p_fix": "Se o perímetro exterior do edifício foi levantado com a estação total (ou é exato de qualquer modo), o plugin usa-o como referência fixa: nunca é movido, e os compartimentos colocam-se sozinhos dentro dele.",
    "fix": ["desenhe o perímetro como <b>polilinha fechada</b>, em coordenadas reais, e os compartimentos à mão dentro dele;", "depois de escolher os compartimentos em <b>Lados e diagonais</b>, a linha de comandos pede o <b>perímetro fixo</b> (opcional): clique nele, ou prima Enter para continuar sem ele. Também se pode acrescentar depois, a partir da lista da janela;", "na página <b>Paredes exteriores</b> escreva a espessura das paredes exteriores (medida nas janelas) e verifique os lados dos compartimentos propostos ao longo do perímetro: cada um fica paralelo ao perímetro, a essa distância;", "um compartimento de canto com dois lados no perímetro não precisa de diagonais para a sua posição; um compartimento com um só lado no perímetro ainda tem de ser ligado aos outros (por exemplo com uma porta)."],
    "h_hatch": "Paredes cheias entre compartimentos: a trama",
    "p_hatch": "Uma parede cheia não se pode atravessar para medir, mas a sua espessura mede-se nas portas. Se puser uma <b>trama</b> nas paredes entre os compartimentos do desenho, o plugin reconhece-as sozinho:",
    "hatch": ["encontra os dois lados de compartimentos diferentes que se defrontam com a trama no meio e propõe-nos na página <b>Paredes cheias entre compartimentos</b>;", "escreva a <b>espessura medida numa porta</b>; se na mesma parede já houver uma porta comum com a espessura, a caixa pode ficar vazia;", "no cálculo os dois lados ficam paralelos a essa distância, com o seu resíduo na tabela."],
    "p_hatch2": "Uma parede cheia fixa a distância e a rotação entre dois compartimentos, mas não o deslizamento ao longo da parede: ainda é precisa uma porta na mesma parede, uma segunda parede não paralela para o mesmo compartimento, ou o perímetro fixo.",

    "h5": "5. Portas, janelas e paredes exteriores",
    "h_dw": "Portas e janelas",
    "p_dw": "Também as janelas e as portas para o exterior se acrescentam com <b>Acrescentar porta ou janela</b> e um "
            "clique na parede: tipo (<b>porta</b>, <b>janela</b>, <b>abertura</b>), distância do canto à ombreira e "
            "largura livre. A porta é articulada do lado do canto a partir do qual mede. Anote as alturas (peitoril, "
            "padieira) no croqui: o plugin trabalha em planta.",
    "h_ext": "Paredes exteriores",
    "p_ext": "O plugin reconstrói os compartimentos na face interior. Para o perímetro exterior meça a <b>espessura da "
             "parede nas janelas</b> (ombreira incluída), pelo menos uma por fachada; depois, no CAD, obtenha a face "
             "exterior com OFFSET a partir dos lados dos compartimentos, à distância medida, e una os cantos. Onde for "
             "possível verifique com uma medida exterior (o comprimento de uma fachada) tirada de fora.",

    "h6": "6. No escritório: cálculo e controlo",
    "calc": [
        "<b>Lados e diagonais</b>: clique em todos os compartimentos (mesma ordem do croqui) e prima Enter. Abre-se a "
        "lista dos compartimentos com o seu estado: abra um compartimento de cada vez e escreva lados e diagonais, "
        "acrescente as portas com um clique no desenho; quando tudo está completo a lista propõe <b>Calcular</b>.",
        "Se as medidas não chegam ou não batem certo, a janela volta a abrir-se com o motivo e com as medidas já escritas.",
        "O resultado vai para a camada <b>RILIEVO_CORRETTO</b>, com a área de cada compartimento; cotas, diagonais e "
        "tabela em <b>RILIEVO_QUOTE</b>; portas e janelas em <b>RILIEVO_APERTURE</b>. O desenho à mão fica para "
        "comparação. <b>Aplicar ao desenho</b> leva depois as polilinhas originais à forma calculada.",
        "Leia a tabela dos resíduos: um resíduo vermelho indica a medida a verificar de novo. Com uma só medida a mais "
        "o erro reparte-se por várias medidas: tirar outra ajuda a encontrá-lo.",
        "<b>Caderno de medidas</b> guarda tudo num CSV para Excel ou Calc (medidas, resíduos, coordenadas, áreas).",
    ],
    "h_vals": "Valores indicativos",
    "t6": [["Grandeza", "Valor típico", "Se for maior"],
           ["resíduo de um lado ou de uma diagonal", "até 1 cm", "volte a medir; verifique algarismos trocados e o canto certo"],
           ["σ₀ de um compartimento", "0,3–0,8 cm", "medidas tiradas a alturas diferentes, cantos pouco nítidos"],
           ["ângulo fixado a 90°", "resíduo até 0,2°", "o ângulo não é reto: retire a restrição e meça uma diagonal"]],
    "h_list": "Lista de verificação antes de sair da obra",
    "list": [
        "cada compartimento tem todos os lados e pelo menos n − 3 diagonais (ou restrições verificadas), mais uma "
        "medida de controlo;",
        "cada compartimento está ligado: porta comum com espessura, ou 3 medidas para um compartimento já ligado;",
        "espessuras das paredes exteriores tiradas nas janelas, pelo menos uma por fachada;",
        "portas e janelas: distância a partir do canto, largura, alturas anotadas;",
        "uma foto por compartimento.",
    ],
    "end": "O Room Survey é um projeto amador, não adequado a uso profissional: o autor não assume responsabilidade "
           "pelo seu uso. Código e atualizações: github.com/Macchecaspita/opencad-rilievo-plugin",
}
