"""Manual texts — Čeština (strojový překlad)."""

OUT = "manual_cs.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — příručka zaměření",
    "foot": "Room Survey — příručka zaměření · hobby projekt, ne pro profesionální použití",
    "subtitle": "Jak zaměřit byt přímým měřením<br/>a znovu ho sestavit v Open CAD Studio",
    "warn": "<b>Upozornění.</b> Room Survey je hobby projekt, vytvořený pro zábavu a <b>nevhodný pro profesionální "
            "použití</b>. Poskytuje se tak, jak je, bez jakékoli záruky; autor nenese žádnou odpovědnost za jeho "
            "použití ani za výsledky. Výsledky vždy ověřte dalšími měřeními.",

    "d_room": "Místnost {r}",
    "d_sketch": "náčrt: očíslované místnosti, rohy s písmeny (zapíše tlačítko 3. Místnosti)",
    "d_check": "B–D kontrola",
    "d_out1": "C–E: vede venku,",
    "d_out2": "nelze změřit",
    "d_from1b": "vzdálenost od 1B",
    "d_width": "0,90 šířka",
    "d_thick": "tloušťka 0,30 (v ostění)",
    "d_from2d": "1,10 vzdálenost od 2D",
    "d_furn": "nábytek",
    "d_blocked": "A–C zablokovaná",

    "h1": "1. Metoda ve zkratce",
    "p1": "Půdorys nakreslíte <b>od oka</b>, na místě změříte strany a několik úhlopříček každé místnosti a několik měr, "
          "které místnosti propojují, a plugin sestaví <b>skutečný</b> půdorys vyrovnáním měr metodou nejmenších "
          "čtverců. Rezidua hned ukážou, zda je některá míra chybná.",
    "t1": [["", "Kde", "Co děláte"],
           ["1", "u stolu", "náčrt: nakreslete každou místnost jako uzavřenou křivku (od oka) a napište písmena"],
           ["2", "na místě", "změřte strany, úhlopříčky, tloušťky zdí, dveře a okna a zapište je do náčrtu"],
           ["3", "u stolu", "zadejte míry, vypočítejte, přečtěte rezidua, dokončete výkres"]],
    "h_tools": "Vybavení",
    "tools": [
        "laserový dálkoměr (lépe, když měří od zadní hrany) a 5m pásmo pro krátké míry (tloušťky, ostění, výklenky);",
        "vytištěný náčrt, tužka a guma; podložka s klipem;",
        "skládací metr nebo úhelník pro kontrolu úhlů, když jsou úhlopříčky zablokované;",
        "fotoaparát nebo telefon: jedna fotka na místnost, s písmeny rohů vyznačenými v náčrtu.",
    ],

    "h2": "2. Před návštěvou: náčrt v CAD",
    "p2a": "Nakreslete <b>každou místnost</b> jako <b>uzavřenou křivku</b> (PLINE … Uzavřít) podle <b>vnitřního líce</b> "
           "zdí. Rozměry nejsou důležité: důležité je, aby každý skutečný roh byl vrcholem křivky, ve správném pořadí. "
           "Ústupky, výklenky, vystupující pilíře a zkosení jsou také vrcholy (místnost 3 v příkladu). Chodba je "
           "místnost jako ostatní.",
    "p2b": "Stiskněte <b>3. Místnosti: čísla, písmena a míry</b> a vyberte místnosti — jednu po druhé v pořadí, které chcete pro čísla, nebo všechny najednou oknem (číslované shora dolů, zleva doprava) — a stiskněte Enter: každá místnost ukáže tučně své číslo a rohy písmena A, B, C …; roh B místnosti 2 se jmenuje 2B. Otevře se i okno měr: zatím ho zavřete tlačítkem <b>Zavřít</b> a odpovězte, že písmena chcete <b>ponechat</b>. Vytiskněte půdorys: to je váš náčrt.",
    "t2": [["Tip", "Proč"],
           ["Číslujte místnosti v pořadí, které budete dodržovat i na místě", "Píšete rychleji a méně často si spletete místnost."],
           ["Křivka na vnitřním líci", "Měří se uvnitř místností; obvodové zdi přijdou později z tloušťek."],
           ["Žádné zbytečné vrcholy", "Vrchol na rovné straně (úhel 180°) vyžaduje zbytečné míry."]],

    "h3": "3. Na místě: zaměření místnosti",
    "p3a": "Místnost s <b>n</b> rohy má 2n − 3 neznámých: určí ji <b>všechny strany</b> plus <b>n − 3 úhlopříček</b>. "
           "Obdélník potřebuje 1 úhlopříčku, místnost tvaru L 3, koupelna s ústupkem z příkladu 3.",
    "p3b": "Strany měřte <b>ve stejné výšce</b> (například 1 m nad podlahou, nad soklovými lištami a nízkým nábytkem), od "
           "rohu k rohu. Potom úhlopříčky. Úhlopříčky navržené pluginem vedou všechny <b>uvnitř</b> místnosti: v místnosti "
           "tvaru L nikdy není úhlopříčka, která by protínala zeď.",
    "cap3": "Vlevo: strany (modře) a úhlopříčka (oranžově) nepravoúhlé místnosti; druhá úhlopříčka (zeleně) je kontrolní "
            "míra. Vpravo: místnost tvaru L, tři užitečné úhlopříčky a jedna, které se vyhnout.",
    "h_check": "Kontrolní míra",
    "p_check": "<b>Vždy změřte o jednu úhlopříčku víc</b>, než je minimum. Bez nadbytečných měr je tvar přesný, ať zadáte "
               "cokoli, a špatně přečtené číslo (6,85 místo 6,58) zůstane nepovšimnuto. S jednou mírou navíc plugin "
               "spočítá <b>rezidua</b> a σ₀ a míry nad tolerancí (obvykle 1–2 cm) obarví červeně.",
    "h_obst": "Když nábytek blokuje úhlopříčku",
    "obst": [
        "Nejdřív zkuste jinou úhlopříčku (B–D místo A–C) nebo ji změřte nad nábytkem s dálkoměrem ve vodorovné poloze.",
        "Pokud to nejde, použijte <b>podmínku</b> (na stránce místnosti): roh a úhel, například <b>A, 90</b>, po ověření "
        "úhelníkem; nebo <b>vodorovnou</b> či <b>svislou</b> stranu. Každá podmínka se počítá jako jedna míra a má "
        "také reziduum.",
        "Pokud stále něco chybí, volba <b>doplnit tvar úhly výkresu</b> (obvykle zapnutá) vezme chybějící z výkresu a "
        "tabulka to označí červeně. V tom místě je výsledek jen tak dobrý jako vaše kresba od oka: poznamenejte si to.",
    ],

    "h4": "4. Propojení místností",
    "p4a": "Každá místnost, změřená samostatně, má správný tvar, ale neví, kde leží vůči ostatním. K propojení potřebujete "
           "<b>alespoň 3 míry</b> k sousední místnosti nebo <b>společné dveře</b>. Dveře jsou nejpohodlnější.",
    "cap4": "Dveře mezi místností 1 a místností 2: vzdálenost od rohu a šířka změřené z každé strany, tloušťka zdi "
            "změřená v ostění.",
    "p4b": "V okně měr pod místnostmi stiskněte <b>＋ Přidat dveře nebo okno</b> a <b>klikněte do výkresu</b>, na zeď mezi oběma místnostmi, poblíž dveří. Plugin sám najde obě strany (zvýrazní je zeleně) a zeptá se:",
    "t4": [["Pole", "Příklad"],
           ["Vzdálenost od rohu 1B k ostění (měřeno v místnosti 1)", "1,00"],
           ["Šířka", "0,90"],
           ["Tloušťka zdi, změřená ve dveřích", "0,30"],
           ["Vzdálenost od rohu 2D k ostění (měřeno v místnosti 2)", "1,10"]],
    "p4c": "Vzdálenost vede od rohu k ostění; plugin navrhne roh nejbližší kliknutému bodu a můžete zvolit druhý. To je "
           "vše: obě místnosti jsou propojené (ostění proti sobě, rovnoběžná zeď, známá tloušťka). Jedna míra navíc "
           "přes dveře (například od 1B do 2D) slouží jako kontrola.",
    "p4d": "Bez dveří (nebo pro kontrolu) použijte oddíl <b>Další míry mezi místnostmi</b> dole v okně:",
    "links": [
        "<b>tloušťka zdi</b>: Od = strana (1B-1C), Do = roh druhé místnosti za touto zdí (2A), Změřeno = tloušťka "
        "změřená kolmo ve dveřích;",
        "<b>míry přes dveře</b>: Od = 1B, Do = 2D, mezi rohy obou místností viditelnými přes dveře; vezměte dvě k různým rohům.",
    ],
    "h_order": "Doporučené pořadí pro ukázkový byt",
    "p_order": "Začněte <b>chodbou</b> (místnost 4), která se dotýká téměř všech ostatních, a každou místnost propojte s "
               "chodbou jejími dveřmi: obývák a ložnici vlevo, kuchyň a ložnici 2 vpravo; koupelna se propojí s "
               "ložnicí. Každá místnost tak visí na krátkém řetězci a chyba zůstane místní.",

    "h_fix": "Pevný obrys zaměřený totální stanicí",
    "p_fix": "Pokud byl vnější obrys budovy zaměřen totální stanicí (nebo je stejně přesný), plugin ho použije jako pevnou referenci: nikdy se neposune a místnosti se umístí dovnitř.",
    "fix": ["nakreslete obrys jako <b>uzavřenou křivku</b> ve skutečných souřadnicích a místnosti od oka uvnitř;", "stiskněte <b>1. Vnější obrys</b>, klikněte na křivku a stiskněte Enter (Enter bez kliknutí = bez obrysu);", "stiskněte <b>2. Tloušťky obvodových zdí</b> a klikejte na strany obrysu jednu po druhé: pro každou zadejte tloušťku zdi (změřenou v oknech) a zvolte směr, <i>ke středu</i> pro běžnou obvodovou zeď (<i>ven</i> jen nakreslí čáru, která zůstane mimo výpočet). Jakmile stisknete OK, objeví se ve výkresu rovnoběžka této strany a její hodnota, ve hladině RILIEVO_PERIMETRO: strana bez čáry je zapomenutá strana. Enter pro dokončení: souhrn vypíše strany bez tloušťky;", "ve výpočtu zůstanou strany místností podél obrysu s ním rovnoběžné v této vzdálenosti: rohová místnost se dvěma stranami na obrysu nepotřebuje pro svou polohu úhlopříčky; místnost jen s jednou stranou na obrysu je třeba propojit s ostatními (například dveřmi)."],
    "h_hatch": "Plné zdi mezi místnostmi: šrafa",
    "p_hatch": "Přes plnou zeď měřit nelze, ale její tloušťka se měří u dveří. Když ve výkresu dáte do zdí mezi místnostmi <b>šrafu</b>, plugin je najde sám:",
    "hatch": ["najde dvě protilehlé strany různých místností se šrafou mezi nimi a navrhne je v oddílu <b>Plné zdi mezi místnostmi</b> okna měr;", "zadejte <b>tloušťku změřenou u dveří</b>; pokud má stejná zeď již společné dveře s tloušťkou, pole může zůstat prázdné;", "ve výpočtu zůstanou obě strany rovnoběžné v této vzdálenosti, se svým reziduem v tabulce."],
    "p_hatch2": "Plná zeď určí vzdálenost a natočení mezi dvěma místnostmi, ale ne posun podél zdi: stále potřebujete dveře ve stejné zdi, druhou nerovnoběžnou zeď ke stejné místnosti nebo pevný obrys.",

    "h5": "5. Dveře, okna a obvodové zdi",
    "h_dw": "Dveře a okna",
    "p_dw": "Okna a vnější dveře se také přidávají přes <b>Přidat dveře nebo okno</b> a kliknutí na zeď: druh "
            "(<b>dveře</b>, <b>okno</b>, <b>otvor</b>), vzdálenost od rohu k ostění a světlá šířka. Dveře jsou zavěšené "
            "na straně rohu, od kterého měříte. Výšky (parapet, nadpraží) si poznamenejte do náčrtu: plugin pracuje v půdorysu.",
    "h_ext": "Obvodové zdi",
    "p_ext": "Plugin sestaví místnosti na vnitřním líci. Pro vnější obrys změřte <b>tloušťku zdi v oknech</b> (včetně "
             "ostění), alespoň jednu na fasádu; pak v CAD nakreslete vnější líc příkazem OFFSET od stran místností v "
             "naměřené vzdálenosti a spojte rohy. Kde je to možné, ověřte vnější mírou (délka fasády) změřenou zvenku.",

    "h6": "6. U stolu: výpočet a kontrola",
    "calc": ["Stiskněte <b>3. Místnosti: čísla, písmena a míry</b>, vyberte všechny místnosti (ve stejném pořadí jako v náčrtu) a stiskněte Enter. Otevře se <b>jediné okno</b> se všemi místnostmi pod sebou, od místnosti 1 po poslední, každá s nadpisem a stavem: posouvejte ho lištou vpravo a zadávejte strany a úhlopříčky. Pod místnostmi následují dveře a okna, plné zdi, další míry mezi místnostmi a možnosti.", "Stiskněte <b>▶ Vypočítat</b>: křivky místností se přesunou do vypočteného tvaru a <b>výkres se aktualizuje sám</b> (Ctrl+Z to vrátí). Pokud míry nestačí nebo si neodpovídají, okno se znovu otevře s důvodem a již zadanými měrami.", "Kóty, úhlopříčky a tabulka jdou do <b>RILIEVO_QUOTE</b>; dveře a okna do <b>RILIEVO_APERTURE</b>. Okno <b>Výsledek výpočtu</b> shrne plochy, nadbytečné míry a σ₀.", "Přečtěte tabulku reziduí: červené reziduum označuje míru k opětovné kontrole. S jedinou nadbytečnou mírou se chyba rozloží na více měr: další míra ji pomůže najít.", "<b>Zápisník měr</b> uloží vše jako CSV pro Excel nebo Calc (míry, rezidua, souřadnice, plochy)."],
    "h_vals": "Typické hodnoty",
    "t6": [["Veličina", "Typická hodnota", "Pokud je vyšší"],
           ["reziduum strany nebo úhlopříčky", "do 1 cm", "přeměřit; zkontrolovat přehozené číslice a správný roh"],
           ["σ₀ místnosti", "0,3–0,8 cm", "míry v různých výškách, neostré rohy"],
           ["úhel vázaný na 90°", "reziduum do 0,2°", "úhel není pravý: odstraňte podmínku a změřte úhlopříčku"]],
    "h_list": "Kontrolní seznam před odchodem ze stavby",
    "list": [
        "každá místnost má všechny strany a alespoň n − 3 úhlopříčky (nebo ověřené podmínky) a kontrolní míru;",
        "každá místnost je propojená: společné dveře s tloušťkou nebo 3 míry k již propojené místnosti;",
        "tloušťky obvodových zdí změřené v oknech, alespoň jedna na fasádu;",
        "dveře a okna: vzdálenost od rohu, šířka, výšky zapsané;",
        "jedna fotka na místnost.",
    ],
    "ex_h": "7. Úplný příklad krok za krokem",
    "ex_p": "Byt se sedmi místnostmi v budově se šikmou levou fasádou a křídlem se zkoseným rohem. Vnější obrys byl zaměřen totální stanicí; vnitřní zdi jsou plné a jejich tloušťku lze změřit jen u dveří.",
    "ex_fig": "obrys E1–E7 zaměřen, místnosti 1–7 nakresleny od oka, plné zdi šrafované",
    "ex_cap": "Výchozí výkres: pevný obrys E1–E7, očíslované místnosti s písmeny rohů, zdi vyplněné šrafou, dveře P1–P6 a okna (modře).",
    "ex_ties": [["Místnost", "Strany na obrysu", "Plné zdi (šrafa)", "Dveře"], ["1", "C–D, D–A", "A–B s 4; B–C s 2", "P5"], ["2", "C–D", "D–A s 1; A–B s 5; B–C s 3", "P2"], ["3", "B–C, C–D, D–E", "A–B s 6; E–A s 2", "P6"], ["4", "A–B, D–A", "C–D s 1; B–C s 5", "P1, P5"], ["5", "A–B", "D–A s 4; C–D s 2; B–C s 6 a 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A s 5; A–B s 7; C–D s 3", "P3, P6"], ["7", "A–B, B–C", "C–D s 6; D–A s 5", "P4"]],
    "ex_ties_note": "Šest ze sedmi místností se dotýká obrysu dvěma nerovnoběžnými stranami: samotný obrys je usadí na místo. Místnost 2 má na obrysu jen jednu stranu, ale plná zeď k místnosti 1 (nerovnoběžná) ji zafixuje. Plné zdi a dveře tak slouží i jako kontroly.",
    "ex_steps": [["Obrys z totální stanice", "Zaměřte totální stanicí vnější rohy budovy E1…E7 a spojte je do uzavřené křivky ve skutečných souřadnicích. Je to jediný přesný prvek výkresu: plugin ho nikdy neposune."], ["Místnosti od oka", "Uvnitř obrysu nakreslete každou místnost jako uzavřenou křivku na vnitřním líci zdí. Záleží jen na počtu a pořadí rohů: místnost 3 má pět rohů, protože obvodová zeď je zkosená, místnosti 1 a 4 sledují šikmou fasádu."], ["Šrafa v plných zdech", "Příkazem šrafování (HATCH) vyplňte prostor mezi obrysem a místnostmi, tedy všechny zdi. Šrafa říká pluginu, které místnosti odděluje plná zeď: nemusí být přesná, jen musí ležet mezi oběma stranami."], ["Písmena", "Stiskněte <b>3. Místnosti: čísla, písmena a míry</b>, klikněte na místnosti v pořadí 1…7 z obrázku a stiskněte Enter. Čísla a písmena se objeví hned a otevře se okno měr: zavřete ho a písmena ponechte. Vytiskněte výkres: to je náčrt na stavbu."], ["Na místě: místnosti", "V každé místnosti změřte strany ve stejné výšce a alespoň jednu úhlopříčku. Místnosti se dvěma stranami na obrysu nepotřebují pro tvar úhlopříčky, ale úhlopříčka navíc je jediný způsob, jak odhalit špatně přečtené číslo. Například pro místnost 1:"], ["Na místě: tloušťky a dveře", "Změřte tloušťku obvodových zdí v oknech (zde 0,40) a vnitřních zdí ve dveřích (zde 0,30). U každých dveří si z obou stran zapište vzdálenost od rohu k ostění a šířku:"], ["Tlačítka 1 a 2: obrys a tloušťky", "U stolu stiskněte <b>1. Vnější obrys</b>, klikněte na křivku E1…E7 a stiskněte Enter. Pak stiskněte <b>2. Tloušťky obvodových zdí</b>, klikněte na první stranu, zadejte 0,40, ponechte směr <i>ke středu</i> a zaškrtněte políčko pro použití na všechny strany: po OK se rovnoběžky všech stran objeví najednou. Enter pro dokončení."], ["Tlačítko 3: místnosti a okno měr", "Stiskněte <b>3. Místnosti: čísla, písmena a míry</b>, klikněte na místnosti ve stejném pořadí 1…7 a stiskněte Enter. Okno se otevře s místnostmi 1 až 7 pod sebou; níže je oddíl <b>Plné zdi mezi místnostmi (podle šrafy)</b> s již navrženými řádky: zadejte 0,30 tam, kde nejsou společné dveře; kde jsou, tloušťka se převezme z dveří."], ["Strany, úhlopříčky a dveře", "Posouvejte okno a zadejte strany a úhlopříčky každé místnosti. Pak stiskněte <b>＋ Přidat dveře nebo okno</b> a klikněte na zeď každých dveří P1…P6: plugin najde obě strany, zeptá se na vzdálenosti, šířku a tloušťku a vrátí se do okna. Okna se přidávají stejně, na obvodové zdi."], ["Výpočet", "Když každá místnost ukazuje ✔ a místnosti jsou propojené, stiskněte <b>▶ Vypočítat</b>: křivky místností převezmou vypočtený tvar a výkres se aktualizuje sám. Okno <b>Výsledek výpočtu</b> shrne nadbytečné míry a σ₀; v tabulce červená rezidua označují míry ke kontrole. <b>Zápisník měr</b> uloží vše jako CSV."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Míra (místnost 1)", "Hodnota", "Poznámka"], ["1A–1B", "4,10", "strana"], ["1B–1C", "8,30", "strana"], ["1C–1D", "4,58", "strana na obrysu"], ["1D–1A", "8,31", "strana na obrysu (šikmá)"], ["1A–1C", "9,26", "kontrolní úhlopříčka"], ["1B–1D", "9,48", "druhá úhlopříčka (volitelná)"]],
    "ex_doors": [["Dveře", "První místnost: strana, vzdálenost", "Druhá místnost: strana, vzdálenost", "Šířka", "Tloušťka"], ["P1", "4B–4C, od 4B 1,60", "5D–5A, od 5A 1,60", "0,90", "0,30"], ["P2", "5C–5D, od 5D 1,30", "2A–2B, od 2A 1,30", "0,90", "0,30"], ["P3", "5B–5C, od 5B 5,00", "6D–6A, od 6A 1,10", "0,90", "0,30"], ["P4", "5B–5C, od 5B 1,00", "7D–7A, od 7A 1,00", "0,90", "0,30"], ["P5", "4C–4D, od 4C 1,10", "1A–1B, od 1B 1,10", "0,90", "0,30"], ["P6", "6C–6D, od 6D 1,30", "3A–3B, od 3A 1,30", "0,80", "0,30"]],
    "ex_end": "Jakmile je obrys jednou zaměřen, žádná míra neprochází zdmi: strany a úhlopříčky se měří uvnitř místností, tloušťky u dveří a oken.",
    "end": "Room Survey je hobby projekt, nevhodný pro profesionální použití: autor nenese žádnou odpovědnost za jeho "
           "použití. Kód a aktualizace: github.com/Macchecaspita/opencad-rilievo-plugin",
    "h_holes": 'Otvory dveří a oken ve výkresu: tlačítko «Dveře a okna»',
    "p_holes": 'Po výpočtu tlačítko <b>Dveře a okna</b> vytvoří otvory ve zdech: přeruší čáry zdi, nakreslí ostění, vyprázdní šrafu a nakreslí výplň otvoru s kótou.',
    "cap_holes": 'Okno široké 120 a vysoké 140 cm, 1,50 od rohu 1A: rám u vnitřního líce zdi; vně budovy šedá závorka a kóta šířka nad výškou v centimetrech.',
    "holes": ['stiskněte <b>Dveře a okna</b> a zvolte <b>dveře</b> nebo <b>okno</b>, se <b>šířkou</b> a <b>výškou</b> (hodnoty se znovu nabídnou pro další otvor);', 'klikněte na <b>roh</b>, od kterého jste měřili: roh vnějšího obrysu nebo místnosti;', 'klikněte na <b>stranu</b>, na které je otvor, a zadejte <b>vzdálenost</b> od rohu k ostění;', 'plugin přeruší kliknutou stranu i čáru na druhé straně zdi (i když není rovnoběžná), nakreslí ostění a vyprázdní šrafu v otvoru;', 'na hladině <b>RILIEVO_APERTURE</b> se objeví výplň otvoru a kóta šířka/výška v centimetrech, vně obrysu budovy.'],
    "p_holes2": 'Okno má rám u vnitřního líce zdi; dveře nechávají otvor volný. Příkaz pracuje jen s výkresem: přerušená místnost už není uzavřená křivka a nevstupuje do výpočtu, proto otvory vytvářejte <b>po</b> výpočtu. Ctrl+Z vrátí poslední otvor.',
}
