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
    "d_sketch": "croqui: compartimentos numerados, cantos com letras (escritas pelo botão 3. Compartimentos)",
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
    "p2b": "Prima <b>3. Compartimentos: números, letras e medidas</b> e selecione os compartimentos — um de cada vez pela ordem que quer dar aos números, ou todos juntos com uma janela (numerados de cima para baixo, da esquerda para a direita) — e prima Enter: em cada compartimento aparece o seu número a negrito e nos cantos as letras A, B, C …; o canto B do compartimento 2 chama-se 2B. Abre-se também a janela das medidas: por agora feche-a com <b>Fechar</b> e responda que quer <b>manter</b> as letras. Imprima a planta: é o seu croqui.",
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
    "p4b": "Na janela das medidas, por baixo dos compartimentos, prima <b>＋ Acrescentar porta ou janela</b> e <b>clique no desenho</b>, sobre a parede entre os dois compartimentos, perto da porta. O plugin encontra sozinho os dois lados (realça-os a verde) e pede:",
    "t4": [["Caixa", "Exemplo"],
           ["Distância do canto 1B à ombreira (medida no compartimento 1)", "1,00"],
           ["Largura", "0,90"],
           ["Espessura da parede, medida na porta", "0,30"],
           ["Distância do canto 2D à ombreira (medida no compartimento 2)", "1,10"]],
    "p4c": "A distância toma-se do canto até à ombreira; o plugin propõe o canto mais próximo do ponto clicado e pode "
           "escolher o outro. Basta isto: os dois compartimentos ficam ligados (ombreiras frente a frente, parede "
           "paralela, espessura conhecida). Uma medida a mais através da porta (por exemplo de 1B a 2D) serve de controlo.",
    "p4d": "Sem porta (ou como controlo) use a secção <b>Outras medidas entre compartimentos</b>, no fim da janela:",
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
    "fix": ["desenhe o perímetro como <b>polilinha fechada</b>, em coordenadas reais, e os compartimentos à mão no seu interior;", "prima <b>1. Perímetro exterior</b>, clique na polilinha e prima Enter (Enter sem clique = sem perímetro);", "prima <b>2. Espessuras das paredes exteriores</b> e clique nos lados do perímetro um de cada vez: para cada um escreva a espessura da parede (medida nas janelas) e escolha o sentido, <i>para o centro</i> para uma parede exterior normal (<i>para o exterior</i> só desenha a linha, que fica fora do cálculo). Logo que prime OK, a linha paralela e o valor desse lado aparecem no desenho, na camada RILIEVO_PERIMETRO: um lado sem linha é um lado esquecido. Prima Enter para terminar: o resumo lista os lados sem espessura;", "no cálculo os lados dos compartimentos ao longo do perímetro ficam paralelos a ele, a essa distância: um compartimento de canto com dois lados no perímetro não precisa de diagonais para a sua posição; um compartimento com um só lado no perímetro deve ainda ser ligado aos outros (por exemplo com uma porta)."],
    "h_hatch": "Paredes cheias entre compartimentos: a trama",
    "p_hatch": "Uma parede cheia não se pode atravessar para medir, mas a sua espessura mede-se nas portas. Se puser uma <b>trama</b> nas paredes entre os compartimentos do desenho, o plugin reconhece-as sozinho:",
    "hatch": ["encontra os dois lados de compartimentos diferentes que se defrontam com a trama no meio e propõe-nos na secção <b>Paredes cheias entre compartimentos</b> da janela das medidas;", "escreva a <b>espessura medida numa porta</b>; se na mesma parede já houver uma porta comum com a espessura, a caixa pode ficar vazia;", "no cálculo os dois lados ficam paralelos a essa distância, com o seu resíduo na tabela."],
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
    "calc": ["Prima <b>3. Compartimentos: números, letras e medidas</b>, selecione todos os compartimentos (mesma ordem do croqui) e prima Enter. Abre-se <b>uma só janela</b> com todos os compartimentos um debaixo do outro, do 1 ao último, cada um com o seu título e o seu estado: desloque-a com a barra à direita e escreva lados e diagonais. Por baixo dos compartimentos estão portas e janelas, paredes cheias, outras medidas entre compartimentos e opções.", "Prima <b>▶ Calcular</b>: as polilinhas dos compartimentos são levadas à forma calculada e <b>o desenho atualiza-se sozinho</b> (Ctrl+Z anula). Se as medidas não bastarem ou não baterem certo, a janela reabre-se com o motivo e com as medidas já escritas.", "Cotas, diagonais e tabela vão para <b>RILIEVO_QUOTE</b>; portas e janelas para <b>RILIEVO_APERTURE</b>. Uma janela <b>Resultado do cálculo</b> resume áreas, medidas a mais e σ₀.", "Leia a tabela dos resíduos: um resíduo a vermelho indica a medida a verificar de novo. Com uma só medida a mais o erro espalha-se por várias medidas: tirar outra ajuda a encontrá-lo.", "<b>Caderno de medidas</b> guarda tudo num CSV para Excel ou Calc (medidas, resíduos, coordenadas, áreas)."],
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
    "ex_h": "7. Exemplo completo, passo a passo",
    "ex_p": "Uma habitação de sete compartimentos, num edifício com a fachada esquerda inclinada e uma ala com o canto chanfrado. O perímetro exterior foi levantado com a estação total; as paredes interiores são cheias e a sua espessura só se pode medir nas portas.",
    "ex_fig": "perímetro E1–E7 levantado, compartimentos 1–7 desenhados à mão, paredes cheias com trama",
    "ex_cap": "O desenho de partida: perímetro fixo E1–E7, compartimentos numerados com as letras dos cantos, paredes preenchidas com a trama, portas P1–P6 e janelas (a azul).",
    "ex_ties": [["Compart.", "Lados no perímetro", "Paredes cheias (trama)", "Portas"], ["1", "C–D, D–A", "A–B com 4; B–C com 2", "P5"], ["2", "C–D", "D–A com 1; A–B com 5; B–C com 3", "P2"], ["3", "B–C, C–D, D–E", "A–B com 6; E–A com 2", "P6"], ["4", "A–B, D–A", "C–D com 1; B–C com 5", "P1, P5"], ["5", "A–B", "D–A com 4; C–D com 2; B–C com 6 e 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A com 5; A–B com 7; C–D com 3", "P3, P6"], ["7", "A–B, B–C", "C–D com 6; D–A com 5", "P4"]],
    "ex_ties_note": "Seis compartimentos em sete tocam o perímetro com dois lados não paralelos: o perímetro sozinho põe-nos no lugar. O compartimento 2 tem um só lado no perímetro, mas a parede cheia para o compartimento 1 (não paralela) bloqueia-o. Assim, paredes cheias e portas servem também de controlo.",
    "ex_steps": [["O perímetro da estação total", "Levante com a estação total os cantos exteriores do edifício E1…E7 e una-os numa polilinha fechada, em coordenadas reais. É o único elemento exato do desenho: o plugin nunca o moverá."], ["Os compartimentos à mão", "Dentro do perímetro desenhe cada compartimento como polilinha fechada na face interior das paredes. Só importam o número e a ordem dos cantos: o compartimento 3 tem cinco cantos porque a parede exterior é chanfrada, os compartimentos 1 e 4 seguem a fachada inclinada."], ["A trama nas paredes cheias", "Com o comando de trama (HATCH) preencha o espaço entre o perímetro e os compartimentos, ou seja todas as paredes. A trama diz ao plugin que compartimentos estão separados por uma parede cheia: não precisa de ser precisa, basta que esteja entre os dois lados."], ["As letras", "Prima <b>3. Compartimentos: números, letras e medidas</b>, clique nos compartimentos pela ordem 1…7 da figura e prima Enter. Números e letras aparecem logo e abre-se a janela das medidas: feche-a e escolha manter as letras. Imprima o desenho: é o croqui a levar para a obra."], ["Na obra: os compartimentos", "Em cada compartimento meça os lados à mesma altura e pelo menos uma diagonal. Aos compartimentos com dois lados no perímetro as diagonais não fazem falta para a forma, mas uma diagonal a mais é a única maneira de dar por um algarismo mal lido. Para o compartimento 1, por exemplo:"], ["Na obra: espessuras e portas", "Meça a espessura das paredes exteriores nas janelas (aqui 0,40) e a das paredes interiores nas portas (aqui 0,30). Para cada porta anote, dos dois lados, a distância do canto à ombreira e a largura:"], ["Botões 1 e 2: perímetro e espessuras", "No escritório prima <b>1. Perímetro exterior</b>, clique na polilinha E1…E7 e prima Enter. Depois prima <b>2. Espessuras das paredes exteriores</b>, clique no primeiro lado, escreva 0,40, deixe o sentido <i>para o centro</i> e assinale a caixa para a usar em todos os lados: depois de OK aparecem logo as linhas paralelas de todos os lados. Enter para terminar."], ["Botão 3: os compartimentos e a janela das medidas", "Prima <b>3. Compartimentos: números, letras e medidas</b>, clique nos compartimentos pela mesma ordem 1…7 e prima Enter. Abre-se a janela com os compartimentos de 1 a 7, um debaixo do outro; mais abaixo a secção <b>Paredes cheias entre compartimentos (da trama)</b> com as linhas já propostas: escreva 0,30 onde não há porta comum, onde há é tirada da porta."], ["Lados, diagonais e portas", "Desloque a janela e escreva lados e diagonais de cada compartimento. Depois prima <b>＋ Acrescentar porta ou janela</b> e clique na parede de cada porta P1…P6: o plugin encontra os dois lados e pede distâncias, largura e espessura, e volta à janela. As janelas acrescentam-se da mesma forma, na parede exterior."], ["Calcular", "Quando cada compartimento mostra ✔ e os compartimentos estão ligados, prima <b>▶ Calcular</b>: as polilinhas dos compartimentos tomam a forma calculada e o desenho atualiza-se sozinho. A janela <b>Resultado do cálculo</b> resume medidas a mais e σ₀; na tabela os resíduos a vermelho indicam as medidas a verificar. <b>Caderno de medidas</b> guarda tudo num CSV."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Medida (compartimento 1)", "Valor", "Nota"], ["1A–1B", "4,10", "lado"], ["1B–1C", "8,30", "lado"], ["1C–1D", "4,58", "lado no perímetro"], ["1D–1A", "8,31", "lado no perímetro (inclinado)"], ["1A–1C", "9,26", "diagonal de controlo"], ["1B–1D", "9,48", "segunda diagonal (opcional)"]],
    "ex_doors": [["Porta", "Primeiro compart.: lado, distância", "Segundo compart.: lado, distância", "Largura", "Espessura"], ["P1", "4B–4C, a partir de 4B 1,60", "5D–5A, a partir de 5A 1,60", "0,90", "0,30"], ["P2", "5C–5D, a partir de 5D 1,30", "2A–2B, a partir de 2A 1,30", "0,90", "0,30"], ["P3", "5B–5C, a partir de 5B 5,00", "6D–6A, a partir de 6A 1,10", "0,90", "0,30"], ["P4", "5B–5C, a partir de 5B 1,00", "7D–7A, a partir de 7A 1,00", "0,90", "0,30"], ["P5", "4C–4D, a partir de 4C 1,10", "1A–1B, a partir de 1B 1,10", "0,90", "0,30"], ["P6", "6C–6D, a partir de 6D 1,30", "3A–3B, a partir de 3A 1,30", "0,80", "0,30"]],
    "ex_end": "Com o perímetro levantado uma só vez, nenhuma medida atravessa as paredes: lados e diagonais tiram-se dentro dos compartimentos, as espessuras nas portas e janelas.",
    "end": "O Room Survey é um projeto amador, não adequado a uso profissional: o autor não assume responsabilidade "
           "pelo seu uso. Código e atualizações: github.com/Macchecaspita/opencad-rilievo-plugin",
    "h_holes": 'Abrir portas e janelas no desenho: o botão «Portas e janelas»',
    "p_holes": 'Terminado o cálculo, o botão <b>Portas e janelas</b> abre os vãos nas paredes: interrompe as linhas da parede, desenha as ombreiras, esvazia a trama e desenha a caixilharia com a sua cota.',
    "cap_holes": 'Uma janela com 120 de largura e 140 cm de altura, a 1,50 do canto 1A: caixilho na face interior da parede; fora do edifício, o parêntese cinzento e a cota largura sobre altura, em centímetros.',
    "holes": ['prima <b>Portas e janelas</b> e escolha <b>porta</b> ou <b>janela</b>, com <b>largura</b> e <b>altura</b> (os valores voltam a ser propostos para o vão seguinte);', 'clique no <b>canto</b> a partir do qual mediu: um canto do perímetro exterior ou de um compartimento;', 'clique no <b>lado</b> onde está o vão e escreva a <b>distância</b> do canto à ombreira;', 'o plugin interrompe o lado clicado e a linha do outro lado da parede (mesmo que não seja paralela), desenha as ombreiras e esvazia a trama no vão;', 'na camada <b>RILIEVO_APERTURE</b> aparecem a caixilharia e a cota largura/altura em centímetros, fora do perímetro do edifício.'],
    "p_holes2": 'A janela tem o caixilho na face interior da parede; a porta deixa o vão livre. O comando trabalha só no desenho: um compartimento interrompido já não é uma polilinha fechada e já não entra no cálculo, por isso abra os vãos <b>depois</b> de calcular. Ctrl+Z anula o último vão.',
}
