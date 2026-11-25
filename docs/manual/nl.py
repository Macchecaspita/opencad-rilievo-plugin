"""Manual texts — Nederlands (machinevertaling)."""

OUT = "manual_nl.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — handleiding inmeten",
    "foot": "Room Survey — handleiding inmeten · hobbyproject, niet voor professioneel gebruik",
    "subtitle": "Een woning inmeten met directe maten<br/>en opnieuw opbouwen in Open CAD Studio",
    "warn": "<b>Disclaimer.</b> Room Survey is een hobbyproject, gemaakt voor de lol en <b>niet geschikt voor professioneel "
            "gebruik</b>. Het wordt geleverd zoals het is, zonder enige garantie; de auteur aanvaardt geen "
            "verantwoordelijkheid voor het gebruik of de resultaten. Controleer de resultaten altijd met extra maten.",

    "d_room": "Ruimte {r}",
    "d_sketch": "schets: genummerde ruimtes, hoeken met letters (geschreven door knop 3. Ruimtes)",
    "d_check": "B–D controle",
    "d_out1": "C–E: loopt buiten,",
    "d_out2": "niet meetbaar",
    "d_from1b": "afstand vanaf 1B",
    "d_width": "0,90 breedte",
    "d_thick": "dikte 0,30 (in de dag)",
    "d_from2d": "1,10 afstand vanaf 2D",
    "d_furn": "meubel",
    "d_blocked": "A–C geblokkeerd",

    "h1": "1. De methode in het kort",
    "p1": "U tekent de plattegrond <b>op het oog</b>, meet ter plaatse de zijden en enkele diagonalen van elke ruimte, plus "
          "enkele maten die de ruimtes verbinden, en de plugin bouwt de <b>echte</b> plattegrond opnieuw op door de maten "
          "te vereffenen met de kleinste-kwadratenmethode. De residuen tonen meteen of een maat fout is.",
    "t1": [["", "Waar", "Wat u doet"],
           ["1", "aan het bureau", "schets: elke ruimte als gesloten polylijn tekenen (op het oog) en de letters schrijven"],
           ["2", "ter plaatse", "zijden, diagonalen, muurdiktes, deuren en ramen meten en op de schets noteren"],
           ["3", "aan het bureau", "maten invoeren, berekenen, residuen lezen, tekening afmaken"]],
    "h_tools": "Uitrusting",
    "tools": [
        "laserafstandsmeter (liefst meetend vanaf de achterkant) en een rolmaat van 5 m voor korte maten "
        "(diktes, dagkanten, nissen);",
        "de geprinte schets, potlood en gum; een klembord;",
        "een duimstok of winkelhaak om hoeken te controleren als de diagonalen geblokkeerd zijn;",
        "camera of telefoon: één foto per ruimte, met de hoekletters op de schets genoteerd.",
    ],

    "h2": "2. Voor het bezoek: de schets in de CAD",
    "p2a": "Teken <b>elke ruimte</b> als <b>gesloten polylijn</b> (PLINE … Sluiten) langs de <b>binnenzijde</b> van de "
           "muren. De maten doen er niet toe: belangrijk is dat elke echte hoek een hoekpunt van de polylijn is, in de "
           "juiste volgorde. Inspringingen, nissen, uitstekende pijlers en afschuiningen zijn ook hoekpunten (ruimte 3 "
           "van het voorbeeld). De gang is een ruimte zoals de andere.",
    "p2b": "Druk op <b>3. Ruimtes: nummers, letters en maten</b> en selecteer de ruimtes — één voor één in de gewenste volgorde voor de nummers, of allemaal tegelijk met een venster (genummerd van boven naar beneden, links naar rechts) — en druk op Enter: elke ruimte toont haar nummer vet en de hoeken de letters A, B, C …; hoek B van ruimte 2 heet 2B. Ook het matenvenster gaat open: sluit het voorlopig met <b>Sluiten</b> en kies de letters te <b>behouden</b>. Print de plattegrond: dat is uw schets.",
    "t2": [["Tip", "Waarom"],
           ["Nummer de ruimtes in een volgorde die u ook ter plaatse volgt", "U schrijft sneller en kiest minder vaak de verkeerde ruimte."],
           ["Polylijn op de binnenzijde", "Maten worden binnen de ruimtes genomen; buitenmuren volgen later uit de diktes."],
           ["Geen nutteloze hoekpunten", "Een hoekpunt op een rechte zijde (hoek van 180°) vraagt nutteloze maten."]],

    "h3": "3. Ter plaatse: een ruimte meten",
    "p3a": "Een ruimte met <b>n</b> hoeken heeft 2n − 3 onbekenden: ze wordt bepaald door <b>alle zijden</b> plus <b>n − 3 "
           "diagonalen</b>. Een rechthoek heeft 1 diagonaal nodig, een L-vormige ruimte 3, de badkamer met inspringing uit het voorbeeld 3.",
    "p3b": "Meet de zijden <b>op dezelfde hoogte</b> (bijvoorbeeld 1 m boven de vloer, boven plinten en lage meubels), van "
           "hoek tot hoek. Daarna de diagonalen. De door de plugin voorgestelde diagonalen lopen allemaal <b>binnen</b> de "
           "ruimte: in een L-vormige ruimte is er nooit een diagonaal die door de muur gaat.",
    "cap3": "Links: zijden (blauw) en diagonaal (oranje) van een niet-haakse ruimte; de tweede diagonaal (groen) is de "
            "controlemaat. Rechts: L-vormige ruimte, de drie bruikbare diagonalen en één om te vermijden.",
    "h_check": "De controlemaat",
    "p_check": "<b>Neem altijd één diagonaal meer</b> dan het minimum. Zonder extra maten is de vorm exact wat u ook invoert, "
               "en een verkeerd gelezen getal (6,85 in plaats van 6,58) valt niet op. Met één extra maat berekent de plugin "
               "de <b>residuen</b> en σ₀ en kleurt maten boven de tolerantie (meestal 1–2 cm) rood.",
    "h_obst": "Als een meubel een diagonaal blokkeert",
    "obst": [
        "Probeer eerst een andere diagonaal (B–D in plaats van A–C), of meet boven het meubel met de afstandsmeter "
        "waterpas.",
        "Lukt dat niet, gebruik dan een <b>voorwaarde</b> (op de pagina van de ruimte): hoek en waarde, bijvoorbeeld "
        "<b>A, 90</b>, na controle met de winkelhaak; of een <b>horizontale</b> of <b>verticale</b> zijde. Elke "
        "voorwaarde telt als één maat en krijgt ook een residu.",
        "Ontbreekt er dan nog iets, dan neemt de optie <b>vorm aanvullen met de hoeken van de tekening</b> (meestal aan) "
        "het ontbrekende uit de tekening, en de tabel markeert het rood. Op die plek is het resultaat maar zo goed als uw "
        "tekening op het oog: noteer het.",
    ],

    "h4": "4. De ruimtes verbinden",
    "p4a": "Elke ruimte heeft, apart gemeten, de juiste vorm maar weet niet waar ze ligt ten opzichte van de andere. Om haar "
           "te verbinden hebt u <b>minstens 3 maten</b> naar de volgende ruimte nodig, of een <b>gedeelde deur</b>. "
           "De deur is het handigst.",
    "cap4": "Een deur tussen ruimte 1 en ruimte 2: afstand vanaf de hoek en breedte aan beide kanten gemeten, muurdikte "
            "in de dag gemeten.",
    "p4b": "Druk in het matenvenster, onder de ruimtes, op <b>＋ Deur of raam toevoegen</b> en <b>klik in de tekening</b>, op de muur tussen de twee ruimtes, bij de deur. De plugin vindt beide zijden zelf (groen gemarkeerd) en vraagt:",
    "t4": [["Veld", "Voorbeeld"],
           ["Afstand van hoek 1B tot de dag (gemeten in ruimte 1)", "1,00"],
           ["Breedte", "0,90"],
           ["Muurdikte, gemeten in de deur", "0,30"],
           ["Afstand van hoek 2D tot de dag (gemeten in ruimte 2)", "1,10"]],
    "p4c": "De afstand loopt van de hoek tot de dag; de plugin stelt de hoek voor die het dichtst bij het aangeklikte punt "
           "ligt en u kunt de andere kiezen. Dat is alles: de twee ruimtes zijn verbonden (dagkanten tegenover elkaar, "
           "evenwijdige muur, bekende dikte). Eén extra maat door de deur (bijvoorbeeld van 1B naar 2D) dient als controle.",
    "p4d": "Zonder deur (of als controle) gebruikt u de sectie <b>Overige maten tussen ruimtes</b> onderaan het venster:",
    "links": [
        "<b>muurdikte</b>: Van = een zijde (1B-1C), Tot = de hoek van de andere ruimte achter die muur (2A), Gemeten = "
        "de haaks in de deur gemeten dikte;",
        "<b>maten door de deur</b>: Van = 1B, Tot = 2D, tussen hoeken van de twee ruimtes die door de deur zichtbaar "
        "zijn; neem er twee naar verschillende hoeken.",
    ],
    "h_order": "Aanbevolen volgorde voor de voorbeeldwoning",
    "p_order": "Begin met de <b>gang</b> (ruimte 4), die bijna alle andere raakt, en verbind elke ruimte via haar deur met "
               "de gang: woonkamer en slaapkamer links, keuken en slaapkamer 2 rechts; de badkamer wordt met de slaapkamer "
               "verbonden. Zo hangt elke ruimte aan een korte keten en blijft een fout lokaal.",

    "h_fix": "Vaste omtrek, ingemeten met een total station",
    "p_fix": "Als de buitenomtrek van het gebouw met een total station is ingemeten (of toch exact is), gebruikt de plugin hem als vaste referentie: hij wordt nooit verplaatst en de ruimtes plaatsen zich erbinnen.",
    "fix": ["teken de omtrek als <b>gesloten polylijn</b> in echte coördinaten, en de ruimtes op het oog erbinnen;", "druk op <b>1. Buitenomtrek</b>, klik op de polylijn en druk op Enter (Enter zonder klik = geen omtrek);", "druk op <b>2. Diktes buitenmuren</b> en klik de zijden van de omtrek één voor één aan: typ voor elke de muurdikte (gemeten bij de ramen) en kies de richting, <i>naar het midden</i> voor een normale buitenmuur (<i>naar buiten</i> tekent alleen de lijn, die buiten de berekening blijft). Zodra u op OK drukt, verschijnen de evenwijdige lijn van die zijde en de waarde in de tekening, op laag RILIEVO_PERIMETRO: een zijde zonder lijn is een vergeten zijde. Enter om af te ronden: de samenvatting toont de zijden zonder dikte;", "in de berekening blijven de zijden van de ruimtes langs de omtrek er evenwijdig aan, op die afstand: een hoekruimte met twee zijden aan de omtrek heeft voor haar plaats geen diagonalen nodig; een ruimte met maar één zijde aan de omtrek moet nog met de andere verbonden worden (bijvoorbeeld met een deur)."],
    "h_hatch": "Massieve muren tussen ruimtes: de arcering",
    "p_hatch": "Door een massieve muur kun je niet meten, maar de dikte wordt bij de deuren gemeten. Als u in de tekening een <b>arcering</b> in de muren tussen de ruimtes zet, vindt de plugin ze zelf:",
    "hatch": ["hij vindt de twee tegenover elkaar liggende zijden van verschillende ruimtes met de arcering ertussen en stelt ze voor in de sectie <b>Massieve muren tussen ruimtes</b> van het matenvenster;", "typ de <b>bij een deur gemeten dikte</b>; heeft dezelfde muur al een gedeelde deur met dikte, dan mag het veld leeg blijven;", "in de berekening blijven de twee zijden evenwijdig op die afstand, met hun residu in de tabel."],
    "p_hatch2": "Een massieve muur legt de afstand en draaiing tussen twee ruimtes vast, maar niet de verschuiving langs de muur: u hebt nog een deur in dezelfde muur nodig, een tweede niet-evenwijdige muur naar dezelfde ruimte, of de vaste omtrek.",

    "h5": "5. Deuren, ramen en buitenmuren",
    "h_dw": "Deuren en ramen",
    "p_dw": "Ook ramen en buitendeuren worden toegevoegd met <b>Deur of raam toevoegen</b> en een klik op de muur: soort "
            "(<b>deur</b>, <b>raam</b>, <b>opening</b>), afstand van de hoek tot de dag en vrije breedte. De deur draait "
            "aan de kant van de hoek vanwaar u meet. Noteer de hoogtes (borstwering, latei) op de schets: de plugin werkt "
            "in plattegrond.",
    "h_ext": "Buitenmuren",
    "p_ext": "De plugin bouwt de ruimtes op de binnenzijde. Meet voor de buitenomtrek de <b>muurdikte bij de ramen</b> "
             "(dagkant inbegrepen), minstens één per gevel; teken daarna in de CAD de buitenzijde met OFFSET vanaf de "
             "zijden van de ruimtes, op de gemeten afstand, en verbind de hoeken. Controleer waar mogelijk met een "
             "buitenmaat (lengte van een gevel) van buitenaf.",

    "h6": "6. Aan het bureau: berekenen en controleren",
    "calc": ["Druk op <b>3. Ruimtes: nummers, letters en maten</b>, selecteer alle ruimtes (zelfde volgorde als de schets) en druk op Enter. Er opent <b>één venster</b> met alle ruimtes onder elkaar, van ruimte 1 tot de laatste, elk met titel en status: scroll met de balk rechts en typ zijden en diagonalen. Onder de ruimtes volgen deuren en ramen, massieve muren, overige maten tussen ruimtes en opties.", "Druk op <b>▶ Berekenen</b>: de polylijnen van de ruimtes krijgen de berekende vorm en <b>de tekening werkt zichzelf bij</b> (Ctrl+Z maakt het ongedaan). Zijn de maten onvoldoende of passen ze niet, dan opent het venster opnieuw met de reden en de al ingevoerde maten.", "Maatvoering, diagonalen en tabel komen op <b>RILIEVO_QUOTE</b>; deuren en ramen op <b>RILIEVO_APERTURE</b>. Een venster <b>Resultaat van de berekening</b> vat oppervlaktes, overtallige maten en σ₀ samen.", "Lees de residuentabel: een rood residu markeert de maat die opnieuw gecontroleerd moet worden. Met maar één overtallige maat verspreidt de fout zich over meerdere maten: nog een maat helpt hem te vinden.", "<b>Maatboek</b> bewaart alles als CSV voor Excel of Calc (maten, residuen, coördinaten, oppervlaktes)."],
    "h_vals": "Typische waarden",
    "t6": [["Grootheid", "Typische waarde", "Als hoger"],
           ["residu van een zijde of diagonaal", "binnen 1 cm", "opnieuw meten; verwisselde cijfers en de juiste hoek controleren"],
           ["σ₀ van een ruimte", "0,3–0,8 cm", "maten op verschillende hoogtes, hoeken niet scherp"],
           ["hoek vastgelegd op 90°", "residu binnen 0,2°", "de hoek is niet haaks: voorwaarde weghalen en een diagonaal meten"]],
    "h_list": "Checklist voor u de bouwplaats verlaat",
    "list": [
        "elke ruimte heeft al haar zijden en minstens n − 3 diagonalen (of gecontroleerde voorwaarden), plus een controlemaat;",
        "elke ruimte is verbonden: gedeelde deur met dikte, of 3 maten naar een al verbonden ruimte;",
        "buitenmuurdiktes bij de ramen gemeten, minstens één per gevel;",
        "deuren en ramen: afstand vanaf de hoek, breedte, hoogtes genoteerd;",
        "één foto per ruimte.",
    ],
    "ex_h": "7. Een volledig voorbeeld, stap voor stap",
    "ex_p": "Een woning met zeven ruimtes, in een gebouw met een schuine linkergevel en een vleugel met afgeschuinde hoek. De buitenomtrek is met een total station ingemeten; de binnenmuren zijn massief en hun dikte kan alleen bij de deuren gemeten worden.",
    "ex_fig": "omtrek E1–E7 ingemeten, ruimtes 1–7 op het oog getekend, massieve muren gearceerd",
    "ex_cap": "De starttekening: vaste omtrek E1–E7, genummerde ruimtes met hoekletters, muren gevuld met arcering, deuren P1–P6 en ramen (blauw).",
    "ex_ties": [["Ruimte", "Zijden aan de omtrek", "Massieve muren (arcering)", "Deuren"], ["1", "C–D, D–A", "A–B met 4; B–C met 2", "P5"], ["2", "C–D", "D–A met 1; A–B met 5; B–C met 3", "P2"], ["3", "B–C, C–D, D–E", "A–B met 6; E–A met 2", "P6"], ["4", "A–B, D–A", "C–D met 1; B–C met 5", "P1, P5"], ["5", "A–B", "D–A met 4; C–D met 2; B–C met 6 en 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A met 5; A–B met 7; C–D met 3", "P3, P6"], ["7", "A–B, B–C", "C–D met 6; D–A met 5", "P4"]],
    "ex_ties_note": "Zes van de zeven ruimtes raken de omtrek met twee niet-evenwijdige zijden: de omtrek alleen zet ze op hun plaats. Ruimte 2 heeft maar één zijde aan de omtrek, maar de massieve muur naar ruimte 1 (niet evenwijdig) zet haar vast. Massieve muren en deuren werken zo ook als controles.",
    "ex_steps": [["De omtrek van het total station", "Meet de buitenhoeken van het gebouw E1…E7 in met het total station en verbind ze tot een gesloten polylijn, in echte coördinaten. Het is het enige exacte element van de tekening: de plugin verplaatst hem nooit."], ["De ruimtes op het oog", "Teken binnen de omtrek elke ruimte als gesloten polylijn op de binnenzijde van de muren. Alleen aantal en volgorde van de hoeken tellen: ruimte 3 heeft vijf hoeken omdat de buitenmuur afgeschuind is, ruimtes 1 en 4 volgen de schuine gevel."], ["De arcering in de massieve muren", "Vul met het arceringscommando (HATCH) de ruimte tussen omtrek en ruimtes, dus alle muren. De arcering vertelt de plugin welke ruimtes door een massieve muur gescheiden zijn: ze hoeft niet precies te zijn, alleen tussen de twee zijden te liggen."], ["De letters", "Druk op <b>3. Ruimtes: nummers, letters en maten</b>, klik de ruimtes aan in de volgorde 1…7 van de figuur en druk op Enter. Nummers en letters verschijnen meteen en het matenvenster gaat open: sluit het en kies de letters te behouden. Print de tekening: dat is de schets voor de bouwplaats."], ["Ter plaatse: de ruimtes", "Meet in elke ruimte de zijden op dezelfde hoogte en minstens één diagonaal. Ruimtes met twee zijden aan de omtrek hebben voor hun vorm geen diagonalen nodig, maar een extra diagonaal is de enige manier om een verkeerd gelezen getal op te merken. Voor ruimte 1 bijvoorbeeld:"], ["Ter plaatse: diktes en deuren", "Meet de dikte van de buitenmuren bij de ramen (hier 0,40) en die van de binnenmuren bij de deuren (hier 0,30). Noteer voor elke deur aan beide kanten de afstand van de hoek tot de dag en de breedte:"], ["Knoppen 1 en 2: omtrek en diktes", "Druk aan het bureau op <b>1. Buitenomtrek</b>, klik op de polylijn E1…E7 en druk op Enter. Druk dan op <b>2. Diktes buitenmuren</b>, klik op de eerste zijde, typ 0,40, laat de richting op <i>naar het midden</i> en vink het vakje aan om haar op alle zijden toe te passen: na OK verschijnen de evenwijdige lijnen van alle zijden in één keer. Enter om af te ronden."], ["Knop 3: de ruimtes en het matenvenster", "Druk op <b>3. Ruimtes: nummers, letters en maten</b>, klik de ruimtes aan in dezelfde volgorde 1…7 en druk op Enter. Het venster opent met de ruimtes 1 tot 7 onder elkaar; verder naar beneden de sectie <b>Massieve muren tussen ruimtes (uit de arcering)</b> met al voorgestelde rijen: typ 0,30 waar geen gedeelde deur is; waar er een is, wordt de dikte van de deur genomen."], ["Zijden, diagonalen en deuren", "Scroll het venster en typ de zijden en diagonalen van elke ruimte. Druk dan op <b>＋ Deur of raam toevoegen</b> en klik op de muur van elke deur P1…P6: de plugin vindt beide zijden en vraagt afstanden, breedte en dikte, en keert dan terug naar het venster. Ramen worden op dezelfde manier toegevoegd, op de buitenmuur."], ["Berekenen", "Als elke ruimte ✔ toont en de ruimtes verbonden zijn, druk op <b>▶ Berekenen</b>: de polylijnen van de ruimtes nemen de berekende vorm aan en de tekening werkt zichzelf bij. Het venster <b>Resultaat van de berekening</b> vat overtallige maten en σ₀ samen; in de tabel markeren rode residuen de opnieuw te controleren maten. <b>Maatboek</b> bewaart alles als CSV."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Maat (ruimte 1)", "Waarde", "Opmerking"], ["1A–1B", "4,10", "zijde"], ["1B–1C", "8,30", "zijde"], ["1C–1D", "4,58", "zijde aan de omtrek"], ["1D–1A", "8,31", "zijde aan de omtrek (schuin)"], ["1A–1C", "9,26", "controlediagonaal"], ["1B–1D", "9,48", "tweede diagonaal (optioneel)"]],
    "ex_doors": [["Deur", "Eerste ruimte: zijde, afstand", "Tweede ruimte: zijde, afstand", "Breedte", "Dikte"], ["P1", "4B–4C, vanaf 4B 1,60", "5D–5A, vanaf 5A 1,60", "0,90", "0,30"], ["P2", "5C–5D, vanaf 5D 1,30", "2A–2B, vanaf 2A 1,30", "0,90", "0,30"], ["P3", "5B–5C, vanaf 5B 5,00", "6D–6A, vanaf 6A 1,10", "0,90", "0,30"], ["P4", "5B–5C, vanaf 5B 1,00", "7D–7A, vanaf 7A 1,00", "0,90", "0,30"], ["P5", "4C–4D, vanaf 4C 1,10", "1A–1B, vanaf 1B 1,10", "0,90", "0,30"], ["P6", "6C–6D, vanaf 6D 1,30", "3A–3B, vanaf 3A 1,30", "0,80", "0,30"]],
    "ex_end": "Met de eenmaal ingemeten omtrek gaat geen enkele maat door de muren: zijden en diagonalen worden binnen de ruimtes genomen, diktes bij deuren en ramen.",
    "end": "Room Survey is een hobbyproject, niet geschikt voor professioneel gebruik: de auteur aanvaardt geen "
           "verantwoordelijkheid voor het gebruik. Code en updates: github.com/Macchecaspita/opencad-rilievo-plugin",
    "h_holes": 'Deuren en ramen openen in de tekening: de knop «Deuren en ramen»',
    "p_holes": 'Na de berekening opent de knop <b>Deuren en ramen</b> de openingen in de muren: hij onderbreekt de muurlijnen, tekent de dagkanten, maakt de arcering leeg en tekent het kozijn met zijn maat.',
    "cap_holes": 'Een raam van 120 breed en 140 cm hoog, op 1,50 van hoek 1A: kozijn aan de binnenzijde van de muur; buiten het gebouw de grijze haak en de maat breedte boven hoogte, in centimeters.',
    "holes": ['druk op <b>Deuren en ramen</b> en kies <b>deur</b> of <b>raam</b>, met <b>breedte</b> en <b>hoogte</b> (de waarden worden voor de volgende opening opnieuw voorgesteld);', 'klik op de <b>hoek</b> vanwaar u hebt gemeten: een hoek van de buitenomtrek of van een ruimte;', 'klik op de <b>zijde</b> waarop de opening ligt en typ de <b>afstand</b> van de hoek tot de dagkant;', 'de plugin onderbreekt de aangeklikte zijde en de lijn aan de andere kant van de muur (ook als die niet evenwijdig is), tekent de dagkanten en maakt de arcering in de opening leeg;', 'op laag <b>RILIEVO_APERTURE</b> verschijnen het kozijn en de maat breedte/hoogte in centimeters, buiten de omtrek van het gebouw.'],
    "p_holes2": 'Het raam heeft zijn kozijn aan de binnenzijde van de muur; de deur laat de opening vrij. De opdracht werkt alleen op de tekening: een onderbroken ruimte is geen gesloten polylijn meer en telt niet meer mee in de berekening, open de openingen dus <b>na</b> het berekenen. Ctrl+Z maakt de laatste opening ongedaan.',
}
