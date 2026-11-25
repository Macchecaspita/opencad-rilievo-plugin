"""Manual texts — Magyar (gépi fordítás)."""

OUT = "manual_hu.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — felmérési kézikönyv",
    "foot": "Room Survey — felmérési kézikönyv · hobbiprojekt, nem professzionális használatra",
    "subtitle": "Lakás felmérése közvetlen mérésekkel<br/>és újraépítése az Open CAD Studióban",
    "warn": "<b>Felelősségkizárás.</b> A Room Survey hobbiprojekt, szórakozásból készült, és <b>nem alkalmas "
            "professzionális használatra</b>. Úgy, ahogy van, mindenféle garancia nélkül érhető el; a szerző nem vállal "
            "felelősséget a használatáért vagy az eredményekért. Az eredményeket mindig ellenőrizze további mérésekkel.",

    "d_room": "{r}. helyiség",
    "d_sketch": "vázlat: számozott helyiségek, betűzött sarkok (a 3. Helyiségek gomb írja)",
    "d_check": "B–D ellenőrzés",
    "d_out1": "C–E: kívül fut,",
    "d_out2": "nem mérhető",
    "d_from1b": "távolság 1B-től",
    "d_width": "0,90 szélesség",
    "d_thick": "vastagság 0,30 (a kávában)",
    "d_from2d": "1,10 távolság 2D-től",
    "d_furn": "bútor",
    "d_blocked": "A–C elzárva",

    "h1": "1. A módszer röviden",
    "p1": "Az alaprajzot <b>szemre</b> rajzolja meg, a helyszínen megméri minden helyiség oldalait és néhány átlóját, "
          "valamint néhány, a helyiségeket összekötő méretet, a bővítmény pedig a mérések legkisebb négyzetes "
          "kiegyenlítésével újraépíti a <b>valódi</b> alaprajzot. A maradékok azonnal megmutatják, ha egy méret hibás.",
    "t1": [["", "Hol", "Mit csinál"],
           ["1", "az asztalnál", "vázlat: minden helyiséget zárt vonalláncként rajzol meg (szemre) és felírja a betűket"],
           ["2", "a helyszínen", "oldalak, átlók, falvastagságok, ajtók és ablakok mérése, felírva a vázlatra"],
           ["3", "az asztalnál", "méretek bevitele, számítás, maradékok olvasása, a rajz befejezése"]],
    "h_tools": "Felszerelés",
    "tools": [
        "lézeres távolságmérő (jobb, ha a készülék hátuljától mér) és 5 m-es mérőszalag a rövid méretekhez "
        "(vastagságok, kávák, fülkék);",
        "a kinyomtatott vázlat, ceruza és radír; írótábla;",
        "colstok vagy derékszög a szögek ellenőrzéséhez, ha az átlók el vannak zárva;",
        "fényképező vagy telefon: helyiségenként egy kép, a sarokbetűkkel a vázlaton jelölve.",
    ],

    "h2": "2. A helyszíni bejárás előtt: a vázlat a CAD-ben",
    "p2a": "Rajzoljon meg <b>minden helyiséget</b> <b>zárt vonalláncként</b> (PLINE … Zár) a falak <b>belső síkja</b> "
           "mentén. A méretek nem számítanak: az a fontos, hogy minden valódi sarok a vonallánc csúcsa legyen, helyes "
           "sorrendben. A beugrók, fülkék, kiálló pillérek és letörések is csúcsok (a példa 3. helyisége). A folyosó is "
           "olyan helyiség, mint a többi.",
    "p2b": "Nyomja meg a <b>3. Helyiségek: számok, betűk és méretek</b> gombot, és jelölje ki a helyiségeket — egyenként a számok kívánt sorrendjében, vagy mindet egyszerre ablakkal (számozás fentről lefelé, balról jobbra) — majd nyomjon Entert: minden helyiség félkövéren mutatja a számát, a sarkok az A, B, C … betűket; a 2. helyiség B sarka 2B. A mérési ablak is megnyílik: egyelőre zárja be a <b>Bezárás</b> gombbal, és válassza a betűk <b>megtartását</b>. Nyomtassa ki az alaprajzot: ez a vázlata.",
    "t2": [["Tipp", "Miért"],
           ["Olyan sorrendben számozza a helyiségeket, amelyet a helyszínen is követ", "Gyorsabban ír, és ritkábban téveszt helyiséget."],
           ["Vonallánc a belső síkon", "A méréseket a helyiségeken belül veszi; a külső falak később a vastagságokból adódnak."],
           ["Nincsenek fölösleges csúcsok", "Egyenes oldalon lévő csúcs (180°-os szög) fölösleges méréseket kér."]],

    "h3": "3. A helyszínen: egy helyiség mérése",
    "p3a": "Egy <b>n</b> sarkú helyiségnek 2n − 3 ismeretlenje van: <b>minden oldal</b> és <b>n − 3 átló</b> határozza "
           "meg. Egy téglalaphoz 1 átló kell, egy L alakú helyiséghez 3, a példa beugrós fürdőszobájához 3.",
    "p3b": "Az oldalakat <b>azonos magasságban</b> mérje (például 1 m-re a padlótól, a szegélylécek és alacsony bútorok "
           "fölött), saroktól sarokig. Utána az átlókat. A bővítmény által javasolt átlók mind a helyiségen <b>belül</b> "
           "futnak: L alakú helyiségben soha nincs falat keresztező átló.",
    "cap3": "Bal oldalt: egy nem derékszögű helyiség oldalai (kék) és átlója (narancs); a második átló (zöld) az "
            "ellenőrző mérés. Jobb oldalt: L alakú helyiség, a három hasznos átló és egy kerülendő.",
    "h_check": "Az ellenőrző mérés",
    "p_check": "<b>Mindig mérjen eggyel több átlót</b> a minimumnál. Fölös mérések nélkül az alak pontos, bármit is ír be, és "
               "egy félreolvasott szám (6,58 helyett 6,85) észrevétlen marad. Egy fölös méréssel a bővítmény kiszámítja a "
               "<b>maradékokat</b> és σ₀-t, és pirosra színezi a tűrésen (általában 1–2 cm) felüli méréseket.",
    "h_obst": "Ha bútor zár el egy átlót",
    "obst": [
        "Először próbáljon másik átlót (A–C helyett B–D), vagy emelje a bútor fölé, a mérőt vízszintesen tartva.",
        "Ha ez nem megy, használjon <b>feltételt</b> (a helyiség oldalán): sarok és szög, például <b>A, 90</b>, miután "
        "derékszöggel ellenőrizte; vagy <b>vízszintes</b> vagy <b>függőleges</b> oldalt. Minden feltétel egy mérésnek "
        "számít, és maradékot is kap.",
        "Ha még mindig hiányzik valami, az <b>alak kiegészítése a rajz szögeivel</b> opció (általában bekapcsolva) a "
        "hiányzót a rajzból veszi, a táblázat pedig pirossal jelöli. Ott az eredmény csak annyira jó, mint a szemre "
        "készült rajza: jegyezze fel.",
    ],

    "h4": "4. A helyiségek összekötése",
    "p4a": "Minden külön mért helyiségnek helyes az alakja, de nem tudja, hol van a többihez képest. Az összekötéshez "
           "<b>legalább 3 mérés</b> kell a szomszéd helyiséghez, vagy egy <b>közös ajtó</b>. Az ajtó a legkényelmesebb.",
    "cap4": "Ajtó az 1. és a 2. helyiség között: a saroktól való távolság és a szélesség mindkét oldalon mérve, a "
            "falvastagság a kávában mérve.",
    "p4b": "A mérési ablakban, a helyiségek alatt nyomja meg az <b>＋ Ajtó vagy ablak hozzáadása</b> gombot, és <b>kattintson a rajzra</b>, a két helyiség közötti falra, az ajtó közelében. A bővítmény maga találja meg mindkét oldalt (zölddel kiemeli), és ezt kérdezi:",
    "t4": [["Mező", "Példa"],
           ["Távolság az 1B saroktól a kávaig (az 1. helyiségben mérve)", "1,00"],
           ["Szélesség", "0,90"],
           ["Falvastagság, az ajtóban mérve", "0,30"],
           ["Távolság a 2D saroktól a kávaig (a 2. helyiségben mérve)", "1,10"]],
    "p4c": "A távolság a saroktól a kávaig tart; a bővítmény a kattintott ponthoz legközelebbi sarkot javasolja, és "
           "választhatja a másikat. Ennyi: a két helyiség össze van kötve (szemközti kávák, párhuzamos fal, ismert "
           "vastagság). Egy fölös mérés az ajtón át (például 1B-től 2D-ig) ellenőrzésként szolgál.",
    "p4d": "Ajtó nélkül (vagy ellenőrzésként) használja az ablak alján az <b>Egyéb méretek a helyiségek között</b> szakaszt:",
    "links": [
        "<b>falvastagság</b>: Honnan = egy oldal (1B-1C), Hová = a másik helyiség sarka a fal mögött (2A), Mért = az "
        "ajtóban merőlegesen mért vastagság;",
        "<b>mérések az ajtón át</b>: Honnan = 1B, Hová = 2D, a két helyiség ajtón át látható sarkai között; vegyen kettőt "
        "különböző sarkokhoz.",
    ],
    "h_order": "Javasolt sorrend a példalakáshoz",
    "p_order": "Kezdje a <b>folyosóval</b> (4. helyiség), amely szinte az összes többivel érintkezik, és kösse minden "
               "helyiséget az ajtajával a folyosóhoz: nappali és hálószoba balra, konyha és 2. hálószoba jobbra; a "
               "fürdőszoba a hálószobához kapcsolódik. Így minden helyiség rövid láncon függ, és a hiba helyi marad.",

    "h_fix": "Mérőállomással felmért rögzített kontúr",
    "p_fix": "Ha az épület külső kontúrját mérőállomással mérték fel (vagy egyébként pontos), a bővítmény rögzített referenciaként használja: soha nem mozdul el, és a helyiségek benne helyezkednek el.",
    "fix": ["rajzolja meg a kontúrt <b>zárt vonalláncként</b> valódi koordinátákban, a helyiségeket pedig szemre benne;", "nyomja meg az <b>1. Külső kontúr</b> gombot, kattintson a vonalláncra, és nyomjon Entert (Enter kattintás nélkül = nincs kontúr);", "nyomja meg a <b>2. Külső falvastagságok</b> gombot, és kattintson egyenként a kontúr oldalaira: mindegyikhez írja be a falvastagságot (az ablakoknál mérve), és válassza az irányt, <i>a középpont felé</i> normál külső falhoz (<i>kifelé</i> csak megrajzolja a vonalat, amely kimarad a számításból). Amint OK-t nyom, az oldal párhuzamosa és értéke megjelenik a rajzon, a RILIEVO_PERIMETRO fólián: a vonal nélküli oldal elfelejtett oldal. Enter a befejezéshez: az összegzés felsorolja a vastagság nélküli oldalakat;", "a számításban a kontúr menti helyiségoldalak azzal párhuzamosak maradnak ezen a távolságon: egy sarokhelyiségnek, amelynek két oldala a kontúron van, nem kell átló a helyzetéhez; egy csak egy oldallal a kontúron lévő helyiséget mégis össze kell kötni a többivel (például ajtóval)."],
    "h_hatch": "Tömör falak a helyiségek között: a sraffozás",
    "p_hatch": "Tömör falon át nem lehet mérni, de a vastagsága az ajtóknál mérhető. Ha a rajzon <b>sraffozást</b> tesz a helyiségek közötti falakba, a bővítmény magától megtalálja őket:",
    "hatch": ["megtalálja a különböző helyiségek két egymással szemközti oldalát, közöttük a sraffozással, és a mérési ablak <b>Tömör falak a helyiségek között</b> szakaszában javasolja őket;", "írja be az <b>ajtónál mért vastagságot</b>; ha ugyanazon a falon már van közös ajtó vastagsággal, a mező üresen maradhat;", "a számításban a két oldal ezen a távolságon párhuzamos marad, maradékával a táblázatban."],
    "p_hatch2": "A tömör fal rögzíti két helyiség távolságát és elfordulását, de nem a fal menti eltolódást: továbbra is kell egy ajtó ugyanabban a falban, egy második, nem párhuzamos fal ugyanahhoz a helyiséghez, vagy a rögzített kontúr.",

    "h5": "5. Ajtók, ablakok és külső falak",
    "h_dw": "Ajtók és ablakok",
    "p_dw": "Az ablakok és a külső ajtók is az <b>Ajtó vagy ablak hozzáadása</b> gombbal és a falra kattintással kerülnek be: "
            "típus (<b>ajtó</b>, <b>ablak</b>, <b>nyílás</b>), távolság a saroktól a kávaig és szabad szélesség. Az ajtó "
            "annak a saroknak az oldalán van felfüggesztve, ahonnan mér. A magasságokat (könyöklő, áthidaló) a vázlatra "
            "írja: a bővítmény alaprajzban dolgozik.",
    "h_ext": "Külső falak",
    "p_ext": "A bővítmény a helyiségeket a belső síkon építi fel. A külső kontúrhoz mérje meg a <b>falvastagságot az "
             "ablakoknál</b> (kávával együtt), homlokzatonként legalább egyet; majd a CAD-ben rajzolja meg a külső síkot "
             "OFFSET-tel a helyiségek oldalaitól a mért távolságra, és kösse össze a sarkokat. Ahol lehet, ellenőrizze "
             "kívülről vett méréssel (egy homlokzat hossza).",

    "h6": "6. Az asztalnál: számítás és ellenőrzés",
    "calc": ["Nyomja meg a <b>3. Helyiségek: számok, betűk és méretek</b> gombot, jelölje ki az összes helyiséget (a vázlattal azonos sorrendben), és nyomjon Entert. <b>Egyetlen ablak</b> nyílik meg az összes helyiséggel egymás alatt, az 1.-től az utolsóig, mindegyik címmel és állapottal: görgesse a jobb oldali sávval, és írja be az oldalakat és átlókat. A helyiségek alatt ajtók és ablakok, tömör falak, egyéb méretek a helyiségek között és beállítások következnek.", "Nyomja meg a <b>▶ Számítás</b> gombot: a helyiségek vonalláncai a számított alakba kerülnek, és <b>a rajz magától frissül</b> (Ctrl+Z visszavonja). Ha a mérések nem elegendők vagy nem illenek össze, az ablak újra megnyílik az okkal és a már bevitt mérésekkel.", "A méretezések, átlók és a táblázat a <b>RILIEVO_QUOTE</b> fóliára kerülnek; az ajtók és ablakok a <b>RILIEVO_APERTURE</b> fóliára. A <b>Számítás eredménye</b> ablak összegzi a területeket, a fölös méréseket és σ₀-t.", "Olvassa el a maradékok táblázatát: a piros maradék az újraellenőrzendő mérést jelöli. Csak egy fölös méréssel a hiba több mérésre oszlik el: egy további mérés segít megtalálni.", "A <b>Mérési napló</b> mindent CSV-ként ment Excelhez vagy Calchoz (mérések, maradékok, koordináták, területek)."],
    "h_vals": "Jellemző értékek",
    "t6": [["Mennyiség", "Jellemző érték", "Ha nagyobb"],
           ["oldal vagy átló maradéka", "1 cm-en belül", "mérje újra; ellenőrizze a felcserélt számjegyeket és a helyes sarkot"],
           ["egy helyiség σ₀-ja", "0,3–0,8 cm", "eltérő magasságban vett mérések, nem éles sarkok"],
           ["90°-ra kötött szög", "maradék 0,2°-on belül", "a szög nem derékszög: törölje a feltételt és mérjen átlót"]],
    "h_list": "Ellenőrzőlista a helyszín elhagyása előtt",
    "list": [
        "minden helyiségnek megvan minden oldala és legalább n − 3 átlója (vagy ellenőrzött feltétele), plusz egy ellenőrző mérés;",
        "minden helyiség össze van kötve: közös ajtó vastagsággal, vagy 3 mérés egy már összekötött helyiséghez;",
        "a külső falvastagságok az ablakoknál mérve, homlokzatonként legalább egy;",
        "ajtók és ablakok: saroktól mért távolság, szélesség, magasságok feljegyezve;",
        "helyiségenként egy fénykép.",
    ],
    "ex_h": "7. Teljes példa lépésről lépésre",
    "ex_p": "Hét helyiségből álló lakás egy ferde bal homlokzatú épületben, egy letört sarkú szárnnyal. A külső kontúrt mérőállomással mérték fel; a belső falak tömörek, vastagságuk csak az ajtóknál mérhető.",
    "ex_fig": "E1–E7 kontúr felmérve, 1–7. helyiség szemre rajzolva, tömör falak sraffozva",
    "ex_cap": "A kiinduló rajz: rögzített E1–E7 kontúr, számozott helyiségek sarokbetűkkel, sraffozással kitöltött falak, P1–P6 ajtók és ablakok (kékkel).",
    "ex_ties": [["Helyiség", "Oldalak a kontúron", "Tömör falak (sraffozás)", "Ajtók"], ["1", "C–D, D–A", "A–B a 4-gyel; B–C a 2-vel", "P5"], ["2", "C–D", "D–A az 1-gyel; A–B az 5-tel; B–C a 3-mal", "P2"], ["3", "B–C, C–D, D–E", "A–B a 6-tal; E–A a 2-vel", "P6"], ["4", "A–B, D–A", "C–D az 1-gyel; B–C az 5-tel", "P1, P5"], ["5", "A–B", "D–A a 4-gyel; C–D a 2-vel; B–C a 6-tal és 7-tel", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A az 5-tel; A–B a 7-tel; C–D a 3-mal", "P3, P6"], ["7", "A–B, B–C", "C–D a 6-tal; D–A az 5-tel", "P4"]],
    "ex_ties_note": "A hét helyiségből hat két nem párhuzamos oldallal érinti a kontúrt: már a kontúr helyre teszi őket. A 2. helyiségnek csak egy oldala van a kontúron, de az 1. helyiség felé eső (nem párhuzamos) tömör fal rögzíti. A tömör falak és az ajtók így ellenőrzésként is szolgálnak.",
    "ex_steps": [["A kontúr a mérőállomásról", "Mérje fel mérőállomással az épület E1…E7 külső sarkait, és kösse össze őket zárt vonalláncba valódi koordinátákban. Ez a rajz egyetlen pontos eleme: a bővítmény soha nem mozdítja el."], ["A helyiségek szemre", "A kontúron belül rajzoljon meg minden helyiséget zárt vonalláncként a falak belső síkján. Csak a sarkok száma és sorrendje számít: a 3. helyiségnek öt sarka van, mert a külső fal letört, az 1. és 4. helyiség a ferde homlokzatot követi."], ["A sraffozás a tömör falakban", "A sraffozás paranccsal (HATCH) töltse ki a kontúr és a helyiségek közötti teret, vagyis az összes falat. A sraffozás jelzi a bővítménynek, mely helyiségeket választ el tömör fal: nem kell pontosnak lennie, csak a két oldal között kell lennie."], ["A betűk", "Nyomja meg a <b>3. Helyiségek: számok, betűk és méretek</b> gombot, kattintson a helyiségekre az ábra 1…7 sorrendjében, és nyomjon Entert. A számok és betűk azonnal megjelennek, és megnyílik a mérési ablak: zárja be, és tartsa meg a betűket. Nyomtassa ki a rajzot: ez a helyszíni vázlat."], ["A helyszínen: a helyiségek", "Minden helyiségben mérje meg az oldalakat azonos magasságban és legalább egy átlót. A két oldallal a kontúron lévő helyiségeknek nem kell átló az alakjukhoz, de egy fölös átló az egyetlen módja egy félreolvasott szám észrevételének. Például az 1. helyiségre:"], ["A helyszínen: vastagságok és ajtók", "Mérje meg a külső falak vastagságát az ablakoknál (itt 0,40) és a belső falakét az ajtóknál (itt 0,30). Minden ajtónál írja fel mindkét oldalon a saroktól a kávaig mért távolságot és a szélességet:"], ["1. és 2. gomb: kontúr és vastagságok", "Az asztalnál nyomja meg az <b>1. Külső kontúr</b> gombot, kattintson az E1…E7 vonalláncra, és nyomjon Entert. Ezután nyomja meg a <b>2. Külső falvastagságok</b> gombot, kattintson az első oldalra, írja be: 0,40, hagyja az irányt <i>a középpont felé</i>, és jelölje be a négyzetet, hogy minden oldalra alkalmazza: OK után az összes oldal párhuzamosa egyszerre megjelenik. Enter a befejezéshez."], ["3. gomb: a helyiségek és a mérési ablak", "Nyomja meg a <b>3. Helyiségek: számok, betűk és méretek</b> gombot, kattintson a helyiségekre ugyanabban az 1…7 sorrendben, és nyomjon Entert. Az ablak az 1–7. helyiséggel egymás alatt nyílik meg; lejjebb a <b>Tömör falak a helyiségek között (a sraffozásból)</b> szakasz már javasolt sorokkal: írjon 0,30-at oda, ahol nincs közös ajtó; ahol van, a vastagságot az ajtóból veszi."], ["Oldalak, átlók és ajtók", "Görgesse az ablakot, és írja be minden helyiség oldalait és átlóit. Ezután nyomja meg az <b>＋ Ajtó vagy ablak hozzáadása</b> gombot, és kattintson minden P1…P6 ajtó falára: a bővítmény megtalálja mindkét oldalt, megkérdezi a távolságokat, a szélességet és a vastagságot, majd visszatér az ablakba. Az ablakokat ugyanígy, a külső falon adja hozzá."], ["Számítás", "Ha minden helyiség ✔-t mutat, és a helyiségek össze vannak kötve, nyomja meg a <b>▶ Számítás</b> gombot: a helyiségek vonalláncai felveszik a számított alakot, és a rajz magától frissül. A <b>Számítás eredménye</b> ablak összegzi a fölös méréseket és σ₀-t; a táblázatban a piros maradékok jelölik az újraellenőrzendő méréseket. A <b>Mérési napló</b> mindent CSV-ként ment."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Mérés (1. helyiség)", "Érték", "Megjegyzés"], ["1A–1B", "4,10", "oldal"], ["1B–1C", "8,30", "oldal"], ["1C–1D", "4,58", "oldal a kontúron"], ["1D–1A", "8,31", "oldal a kontúron (ferde)"], ["1A–1C", "9,26", "ellenőrző átló"], ["1B–1D", "9,48", "második átló (nem kötelező)"]],
    "ex_doors": [["Ajtó", "Első helyiség: oldal, távolság", "Második helyiség: oldal, távolság", "Szélesség", "Vastagság"], ["P1", "4B–4C, 4B-től 1,60", "5D–5A, 5A-tól 1,60", "0,90", "0,30"], ["P2", "5C–5D, 5D-től 1,30", "2A–2B, 2A-tól 1,30", "0,90", "0,30"], ["P3", "5B–5C, 5B-től 5,00", "6D–6A, 6A-tól 1,10", "0,90", "0,30"], ["P4", "5B–5C, 5B-től 1,00", "7D–7A, 7A-tól 1,00", "0,90", "0,30"], ["P5", "4C–4D, 4C-től 1,10", "1A–1B, 1B-től 1,10", "0,90", "0,30"], ["P6", "6C–6D, 6D-től 1,30", "3A–3B, 3A-tól 1,30", "0,80", "0,30"]],
    "ex_end": "Ha a kontúrt egyszer felmérték, egyetlen mérés sem halad át a falakon: az oldalakat és átlókat a helyiségekben, a vastagságokat az ajtóknál és ablakoknál mérik.",
    "end": "A Room Survey hobbiprojekt, nem alkalmas professzionális használatra: a szerző nem vállal felelősséget a "
           "használatáért. Kód és frissítések: github.com/Macchecaspita/opencad-rilievo-plugin",
    "h_holes": 'Ajtók és ablakok a rajzon: az «Ajtók és ablakok» gomb',
    "p_holes": 'A számítás után az <b>Ajtók és ablakok</b> gomb kivágja a nyílásokat a falakban: megszakítja a fal vonalait, megrajzolja a kávákat, kiüríti a sraffozást, és megrajzolja a nyílászárót a méretével.',
    "cap_holes": '120 széles és 140 cm magas ablak, az 1A saroktól 1,50-re: tok a fal belső síkján; az épületen kívül a szürke zárójel és a szélesség a magasság fölött, centiméterben.',
    "holes": ['nyomja meg az <b>Ajtók és ablakok</b> gombot, és válassza az <b>ajtó</b> vagy <b>ablak</b> lehetőséget, <b>szélességgel</b> és <b>magassággal</b> (az értékeket a következő nyílásnál újra felajánlja);', 'kattintson arra a <b>sarokra</b>, ahonnan mért: a külső kontúr vagy egy helyiség sarkára;', 'kattintson arra az <b>oldalra</b>, amelyen a nyílás van, és írja be a <b>távolságot</b> a saroktól a káváig;', 'a bővítmény megszakítja a kattintott oldalt és a fal másik oldalán lévő vonalat (akkor is, ha nem párhuzamos), megrajzolja a kávákat és kiüríti a sraffozást a nyílásban;', 'a <b>RILIEVO_APERTURE</b> rétegen megjelenik a nyílászáró és a szélesség/magasság centiméterben, az épület kontúrján kívül.'],
    "p_holes2": 'Az ablak tokja a fal belső síkján van; az ajtó szabadon hagyja a nyílást. A parancs csak a rajzon dolgozik: a megszakított helyiség már nem zárt vonallánc, és nem vesz részt a számításban, ezért a nyílásokat a számítás <b>után</b> vágja ki. A Ctrl+Z visszavonja az utolsó nyílást.',
}
