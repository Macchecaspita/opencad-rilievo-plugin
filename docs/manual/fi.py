"""Manual texts — Suomi (konekäännös)."""

OUT = "manual_fi.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — mittausopas",
    "foot": "Room Survey — mittausopas · harrasteprojekti, ei ammattikäyttöön",
    "subtitle": "Asunnon mittaaminen suorin mittauksin<br/>ja sen rakentaminen uudelleen Open CAD Studiossa",
    "warn": "<b>Vastuuvapaus.</b> Room Survey on harrasteprojekti, tehty huvin vuoksi eikä <b>sovellu ammattikäyttöön</b>. "
            "Se toimitetaan sellaisenaan ilman mitään takuuta; tekijä ei ota vastuuta sen käytöstä eikä tuloksista. "
            "Tarkista tulokset aina lisämittauksilla.",

    "d_room": "Huone {r}",
    "d_sketch": "luonnos: numeroidut huoneet, kulmat kirjaimin (kirjoittaa painike 3. Huoneet)",
    "d_check": "B–D tarkistus",
    "d_out1": "C–E: kulkee ulkopuolella,",
    "d_out2": "ei voi mitata",
    "d_from1b": "etäisyys 1B:stä",
    "d_width": "0,90 leveys",
    "d_thick": "paksuus 0,30 (pielessä)",
    "d_from2d": "1,10 etäisyys 2D:stä",
    "d_furn": "huonekalu",
    "d_blocked": "A–C estetty",

    "h1": "1. Menetelmä lyhyesti",
    "p1": "Piirrät pohjan <b>silmämääräisesti</b>, mittaat paikan päällä jokaisen huoneen sivut ja muutaman lävistäjän sekä "
          "muutaman huoneita yhdistävän mitan, ja liitännäinen rakentaa <b>todellisen</b> pohjan tasoittamalla mitat "
          "pienimmän neliösumman menetelmällä. Jäännökset näyttävät heti, jos jokin mitta on väärin.",
    "t1": [["", "Missä", "Mitä teet"],
           ["1", "pöydän ääressä", "luonnos: piirrä jokainen huone suljettuna moniviivana (silmämääräisesti) ja kirjoita kirjaimet"],
           ["2", "paikan päällä", "mittaa sivut, lävistäjät, seinäpaksuudet, ovet ja ikkunat ja merkitse ne luonnokseen"],
           ["3", "pöydän ääressä", "syötä mitat, laske, lue jäännökset, viimeistele piirustus"]],
    "h_tools": "Välineet",
    "tools": [
        "laseretäisyysmittari (parempi, jos se mittaa laitteen takareunasta) ja 5 m mittanauha lyhyisiin mittoihin "
        "(paksuudet, pielet, syvennykset);",
        "tulostettu luonnos, lyijykynä ja pyyhekumi; kirjoitusalusta;",
        "taittomitta tai suorakulma kulmien tarkistamiseen, kun lävistäjät ovat estettyjä;",
        "kamera tai puhelin: yksi kuva huonetta kohden, kulmakirjaimet merkittyinä luonnokseen.",
    ],

    "h2": "2. Ennen käyntiä: luonnos CAD-ohjelmassa",
    "p2a": "Piirrä <b>jokainen huone</b> <b>suljettuna moniviivana</b> (PLINE … Sulje) seinien <b>sisäpintaa</b> pitkin. "
           "Mitoilla ei ole väliä: tärkeää on, että jokainen todellinen kulma on moniviivan kärkipiste oikeassa "
           "järjestyksessä. Sisäänvedot, syvennykset, esiin tulevat pilarit ja viisteet ovat myös kärkipisteitä "
           "(esimerkin huone 3). Käytävä on huone siinä missä muutkin.",
    "p2b": "Paina <b>3. Huoneet: numerot, kirjaimet ja mitat</b> ja valitse huoneet — yksi kerrallaan haluamassasi numerojärjestyksessä tai kaikki kerralla ikkunalla (numerointi ylhäältä alas, vasemmalta oikealle) — ja paina Enter: jokainen huone näyttää numeronsa lihavoituna ja kulmat kirjaimet A, B, C …; huoneen 2 kulma B on 2B. Myös mittaikkuna avautuu: sulje se toistaiseksi painikkeella <b>Sulje</b> ja vastaa, että kirjaimet <b>säilytetään</b>. Tulosta pohja: se on luonnoksesi.",
    "t2": [["Vinkki", "Miksi"],
           ["Numeroi huoneet järjestyksessä, jota seuraat myös paikan päällä", "Kirjoitat nopeammin ja sekoitat huoneet harvemmin."],
           ["Moniviiva sisäpinnalle", "Mittaukset tehdään huoneiden sisällä; ulkoseinät saadaan myöhemmin paksuuksista."],
           ["Ei turhia kärkipisteitä", "Kärkipiste suoralla sivulla (180° kulma) vaatii turhia mittoja."]],

    "h3": "3. Paikan päällä: huoneen mittaus",
    "p3a": "Huoneessa, jossa on <b>n</b> kulmaa, on 2n − 3 tuntematonta: sen määrittävät <b>kaikki sivut</b> ja <b>n − 3 "
           "lävistäjää</b>. Suorakulmio tarvitsee 1 lävistäjän, L-muotoinen huone 3, esimerkin sisäänvedollinen kylpyhuone 3.",
    "p3b": "Mittaa sivut <b>samalla korkeudella</b> (esimerkiksi 1 m lattiasta, jalkalistojen ja matalien huonekalujen "
           "yläpuolella), kulmasta kulmaan. Sitten lävistäjät. Liitännäisen ehdottamat lävistäjät kulkevat kaikki huoneen "
           "<b>sisällä</b>: L-muotoisessa huoneessa ei koskaan ole lävistäjää, joka leikkaa seinän.",
    "cap3": "Vasemmalla: suorakulmattoman huoneen sivut (sininen) ja lävistäjä (oranssi); toinen lävistäjä (vihreä) on "
            "tarkistusmitta. Oikealla: L-muotoinen huone, kolme hyödyllistä lävistäjää ja yksi vältettävä.",
    "h_check": "Tarkistusmitta",
    "p_check": "<b>Ota aina yksi lävistäjä enemmän</b> kuin minimi. Ilman ylimääräisiä mittoja muoto on tarkka mitä tahansa "
               "syötätkin, eikä väärin luettu luku (6,85 eikä 6,58) huomaa. Yhdellä lisämitalla liitännäinen laskee "
               "<b>jäännökset</b> ja σ₀:n ja värittää toleranssin (yleensä 1–2 cm) ylittävät mitat punaisiksi.",
    "h_obst": "Kun huonekalu estää lävistäjän",
    "obst": [
        "Kokeile ensin toista lävistäjää (B–D eikä A–C) tai nosta se huonekalun yläpuolelle pitäen mittarin vaakasuorassa.",
        "Jos se ei onnistu, käytä <b>ehtoa</b> (huoneen sivulla): kulma ja sen arvo, esimerkiksi <b>A, 90</b>, kun olet "
        "tarkistanut sen suorakulmalla; tai <b>vaakasuora</b> tai <b>pystysuora</b> sivu. Jokainen ehto lasketaan "
        "yhdeksi mitaksi ja saa myös jäännöksen.",
        "Jos jotain puuttuu yhä, valinta <b>täydennä muoto piirustuksen kulmilla</b> (yleensä päällä) ottaa puuttuvan "
        "piirustuksesta, ja taulukko merkitsee sen punaisella. Siinä kohdassa tulos on vain yhtä hyvä kuin "
        "silmämääräinen piirroksesi: kirjaa se muistiin.",
    ],

    "h4": "4. Huoneiden yhdistäminen",
    "p4a": "Jokaisella erikseen mitatulla huoneella on oikea muoto, mutta se ei tiedä, missä se on muihin nähden. "
           "Yhdistämiseen tarvitset <b>vähintään 3 mittaa</b> seuraavaan huoneeseen tai <b>yhteisen oven</b>. Ovi on kätevin.",
    "cap4": "Ovi huoneen 1 ja huoneen 2 välillä: etäisyys kulmasta ja leveys mitattuina kummaltakin puolelta, "
            "seinäpaksuus mitattuna pielestä.",
    "p4b": "Paina mittaikkunassa huoneiden alla <b>＋ Lisää ovi tai ikkuna</b> ja <b>napsauta piirustusta</b>, kahden huoneen välistä seinää oven läheltä. Liitännäinen löytää molemmat sivut itse (korostaa ne vihreällä) ja kysyy:",
    "t4": [["Kenttä", "Esimerkki"],
           ["Etäisyys kulmasta 1B pieleen (mitattu huoneessa 1)", "1,00"],
           ["Leveys", "0,90"],
           ["Seinäpaksuus, mitattu ovesta", "0,30"],
           ["Etäisyys kulmasta 2D pieleen (mitattu huoneessa 2)", "1,10"]],
    "p4c": "Etäisyys on kulmasta pieleen; liitännäinen ehdottaa napsautettua pistettä lähintä kulmaa, ja voit valita "
           "toisen. Siinä kaikki: huoneet on yhdistetty (pielet vastakkain, yhdensuuntainen seinä, tunnettu paksuus). "
           "Yksi lisämitta oven läpi (esimerkiksi 1B:stä 2D:hen) toimii tarkistuksena.",
    "p4d": "Ilman ovea (tai tarkistukseksi) käytä ikkunan alaosan osiota <b>Muut mitat huoneiden välillä</b>:",
    "links": [
        "<b>seinäpaksuus</b>: Mistä = sivu (1B-1C), Mihin = toisen huoneen kulma seinän takana (2A), Mitattu = ovesta "
        "kohtisuoraan mitattu paksuus;",
        "<b>mitat oven läpi</b>: Mistä = 1B, Mihin = 2D, kahden huoneen kulmien välillä, jotka näkyvät oven läpi; ota "
        "kaksi eri kulmiin.",
    ],
    "h_order": "Suositeltu järjestys esimerkkiasunnolle",
    "p_order": "Aloita <b>käytävästä</b> (huone 4), joka koskettaa lähes kaikkia muita, ja yhdistä jokainen huone käytävään "
               "sen ovella: olohuone ja makuuhuone vasemmalla, keittiö ja makuuhuone 2 oikealla; kylpyhuone yhdistetään "
               "makuuhuoneeseen. Näin jokainen huone riippuu lyhyessä ketjussa ja virhe pysyy paikallisena.",

    "h_fix": "Takymetrillä mitattu kiinteä ulkoreuna",
    "p_fix": "Jos rakennuksen ulkoreuna on mitattu takymetrillä (tai on muuten tarkka), liitännäinen käyttää sitä kiinteänä viitteenä: sitä ei koskaan siirretä, ja huoneet asettuvat sen sisälle.",
    "fix": ["piirrä ulkoreuna <b>suljettuna moniviivana</b> todellisissa koordinaateissa ja huoneet silmämääräisesti sen sisälle;", "paina <b>1. Ulkoreuna</b>, napsauta moniviivaa ja paina Enter (Enter ilman napsautusta = ei ulkoreunaa);", "paina <b>2. Ulkoseinien paksuudet</b> ja napsauta ulkoreunan sivuja yksi kerrallaan: syötä kullekin seinäpaksuus (mitattu ikkunoista) ja valitse suunta, <i>keskustaa kohti</i> tavalliselle ulkoseinälle (<i>ulospäin</i> piirtää vain viivan, joka jää laskennan ulkopuolelle). Heti kun painat OK, sivun suuntaisviiva ja arvo näkyvät piirustuksessa tasolla RILIEVO_PERIMETRO: sivu ilman viivaa on unohtunut sivu. Enter lopettaa: yhteenveto luettelee sivut ilman paksuutta;", "laskennassa ulkoreunan suuntaiset huoneiden sivut pysyvät sen suuntaisina tällä etäisyydellä: kulmahuone, jolla on kaksi sivua ulkoreunalla, ei tarvitse lävistäjiä sijaintiinsa; huone, jolla on vain yksi sivu ulkoreunalla, on silti yhdistettävä muihin (esimerkiksi ovella)."],
    "h_hatch": "Täysseinät huoneiden välillä: viivoitus",
    "p_hatch": "Täysseinän läpi ei voi mitata, mutta sen paksuus mitataan ovista. Jos lisäät piirustuksessa huoneiden välisiin seiniin <b>viivoituksen</b>, liitännäinen löytää ne itse:",
    "hatch": ["se löytää kahden eri huoneen vastakkaiset sivut, joiden välissä on viivoitus, ja ehdottaa niitä mittaikkunan osiossa <b>Täysseinät huoneiden välillä</b>;", "syötä <b>ovesta mitattu paksuus</b>; jos samassa seinässä on jo yhteinen ovi paksuuksineen, kentän voi jättää tyhjäksi;", "laskennassa molemmat sivut pysyvät yhdensuuntaisina tällä etäisyydellä, jäännöksineen taulukossa."],
    "p_hatch2": "Täysseinä kiinnittää kahden huoneen välisen etäisyyden ja kierron, mutta ei siirtymää seinän suunnassa: tarvitset yhä oven samassa seinässä, toisen ei-yhdensuuntaisen seinän samaan huoneeseen tai kiinteän ulkoreunan.",

    "h5": "5. Ovet, ikkunat ja ulkoseinät",
    "h_dw": "Ovet ja ikkunat",
    "p_dw": "Myös ikkunat ja ulko-ovet lisätään toiminnolla <b>Lisää ovi tai ikkuna</b> ja napsautuksella seinään: tyyppi "
            "(<b>ovi</b>, <b>ikkuna</b>, <b>aukko</b>), etäisyys kulmasta pieleen ja vapaa leveys. Ovi on saranoitu sen "
            "kulman puolelle, josta mittaat. Kirjaa korkeudet (ikkunalauta, kamana) luonnokseen: liitännäinen toimii pohjassa.",
    "h_ext": "Ulkoseinät",
    "p_ext": "Liitännäinen rakentaa huoneet sisäpinnalle. Mittaa ulkoreunaa varten <b>seinäpaksuus ikkunoista</b> (pieli "
             "mukaan lukien), vähintään yksi julkisivua kohden; piirrä sitten CAD:ssa ulkopinta OFFSET-komennolla "
             "huoneiden sivuista mitatulle etäisyydelle ja yhdistä kulmat. Tarkista mahdollisuuksien mukaan ulkoa "
             "otetulla mitalla (julkisivun pituus).",

    "h6": "6. Pöydän ääressä: laskenta ja tarkistus",
    "calc": ["Paina <b>3. Huoneet: numerot, kirjaimet ja mitat</b>, valitse kaikki huoneet (samassa järjestyksessä kuin luonnoksessa) ja paina Enter. Avautuu <b>yksi ikkuna</b>, jossa kaikki huoneet ovat allekkain huoneesta 1 viimeiseen, kukin otsikkoineen ja tiloineen: vieritä oikean reunan palkilla ja syötä sivut ja lävistäjät. Huoneiden alla ovat ovet ja ikkunat, täysseinät, muut mitat huoneiden välillä ja asetukset.", "Paina <b>▶ Laske</b>: huoneiden moniviivat siirretään laskettuun muotoon ja <b>piirustus päivittyy itsestään</b> (Ctrl+Z kumoaa). Jos mitat eivät riitä tai eivät sovi yhteen, ikkuna avautuu uudelleen syyn ja jo syötettyjen mittojen kanssa.", "Mitoitukset, lävistäjät ja taulukko menevät tasolle <b>RILIEVO_QUOTE</b>; ovet ja ikkunat tasolle <b>RILIEVO_APERTURE</b>. Ikkuna <b>Laskennan tulos</b> kokoaa pinta-alat, ylimääräiset mitat ja σ₀:n.", "Lue jäännöstaulukko: punainen jäännös merkitsee uudelleen tarkistettavan mitan. Vain yhdellä ylimääräisellä mitalla virhe jakautuu usealle mitalle: vielä yksi mitta auttaa löytämään sen.", "<b>Mittakirja</b> tallentaa kaiken CSV-tiedostona Exceliä tai Calcia varten (mitat, jäännökset, koordinaatit, pinta-alat)."],
    "h_vals": "Tyypilliset arvot",
    "t6": [["Suure", "Tyypillinen arvo", "Jos suurempi"],
           ["sivun tai lävistäjän jäännös", "alle 1 cm", "mittaa uudelleen; tarkista vaihtuneet numerot ja oikea kulma"],
           ["huoneen σ₀", "0,3–0,8 cm", "mitat eri korkeuksilla, kulmat eivät terävät"],
           ["90°:een sidottu kulma", "jäännös alle 0,2°", "kulma ei ole suora: poista ehto ja mittaa lävistäjä"]],
    "h_list": "Tarkistuslista ennen työmaalta lähtöä",
    "list": [
        "jokaisella huoneella on kaikki sivut ja vähintään n − 3 lävistäjää (tai tarkistetut ehdot) sekä tarkistusmitta;",
        "jokainen huone on yhdistetty: yhteinen ovi paksuuksineen tai 3 mittaa jo yhdistettyyn huoneeseen;",
        "ulkoseinien paksuudet mitattu ikkunoista, vähintään yksi julkisivua kohden;",
        "ovet ja ikkunat: etäisyys kulmasta, leveys, korkeudet kirjattu;",
        "yksi kuva huonetta kohden.",
    ],
    "ex_h": "7. Täydellinen esimerkki vaihe vaiheelta",
    "ex_p": "Seitsemän huoneen asunto rakennuksessa, jossa on vino vasen julkisivu ja siipi viistetyllä kulmalla. Ulkoreuna on mitattu takymetrillä; sisäseinät ovat täysseiniä, ja niiden paksuuden voi mitata vain ovista.",
    "ex_fig": "ulkoreuna E1–E7 mitattu, huoneet 1–7 piirretty silmämääräisesti, täysseinät viivoitettu",
    "ex_cap": "Lähtöpiirustus: kiinteä ulkoreuna E1–E7, numeroidut huoneet kulmakirjaimineen, seinät täytetty viivoituksella, ovet P1–P6 ja ikkunat (sinisellä).",
    "ex_ties": [["Huone", "Sivut ulkoreunalla", "Täysseinät (viivoitus)", "Ovet"], ["1", "C–D, D–A", "A–B ja 4; B–C ja 2", "P5"], ["2", "C–D", "D–A ja 1; A–B ja 5; B–C ja 3", "P2"], ["3", "B–C, C–D, D–E", "A–B ja 6; E–A ja 2", "P6"], ["4", "A–B, D–A", "C–D ja 1; B–C ja 5", "P1, P5"], ["5", "A–B", "D–A ja 4; C–D ja 2; B–C ja 6 sekä 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A ja 5; A–B ja 7; C–D ja 3", "P3, P6"], ["7", "A–B, B–C", "C–D ja 6; D–A ja 5", "P4"]],
    "ex_ties_note": "Seitsemästä huoneesta kuusi koskettaa ulkoreunaa kahdella ei-yhdensuuntaisella sivulla: pelkkä ulkoreuna asettaa ne paikoilleen. Huoneella 2 on vain yksi sivu ulkoreunalla, mutta täysseinä huoneeseen 1 (ei yhdensuuntainen) lukitsee sen. Täysseinät ja ovet toimivat näin myös tarkistuksina.",
    "ex_steps": [["Ulkoreuna takymetristä", "Mittaa rakennuksen ulkokulmat E1…E7 takymetrillä ja yhdistä ne suljetuksi moniviivaksi todellisissa koordinaateissa. Se on piirustuksen ainoa tarkka osa: liitännäinen ei koskaan siirrä sitä."], ["Huoneet silmämääräisesti", "Piirrä ulkoreunan sisälle jokainen huone suljettuna moniviivana seinien sisäpintaa pitkin. Vain kulmien määrä ja järjestys ratkaisevat: huoneessa 3 on viisi kulmaa, koska ulkoseinä on viistetty, huoneet 1 ja 4 seuraavat vinoa julkisivua."], ["Viivoitus täysseiniin", "Täytä viivoituskomennolla (HATCH) ulkoreunan ja huoneiden välinen tila eli kaikki seinät. Viivoitus kertoo liitännäiselle, mitkä huoneet erottaa täysseinä: sen ei tarvitse olla tarkka, kunhan se on kahden sivun välissä."], ["Kirjaimet", "Paina <b>3. Huoneet: numerot, kirjaimet ja mitat</b>, napsauta huoneita kuvan järjestyksessä 1…7 ja paina Enter. Numerot ja kirjaimet ilmestyvät heti ja mittaikkuna avautuu: sulje se ja säilytä kirjaimet. Tulosta piirustus: se on työmaalle vietävä luonnos."], ["Paikan päällä: huoneet", "Mittaa jokaisessa huoneessa sivut samalla korkeudella ja vähintään yksi lävistäjä. Huoneet, joilla on kaksi sivua ulkoreunalla, eivät tarvitse lävistäjiä muotoonsa, mutta ylimääräinen lävistäjä on ainoa tapa huomata väärin luettu luku. Esimerkiksi huoneelle 1:"], ["Paikan päällä: paksuudet ja ovet", "Mittaa ulkoseinien paksuus ikkunoista (tässä 0,40) ja sisäseinien paksuus ovista (tässä 0,30). Kirjaa jokaisesta ovesta molemmilta puolilta etäisyys kulmasta pieleen ja leveys:"], ["Painikkeet 1 ja 2: ulkoreuna ja paksuudet", "Paina pöydän ääressä <b>1. Ulkoreuna</b>, napsauta moniviivaa E1…E7 ja paina Enter. Paina sitten <b>2. Ulkoseinien paksuudet</b>, napsauta ensimmäistä sivua, syötä 0,40, jätä suunnaksi <i>keskustaa kohti</i> ja rastita ruutu käyttääksesi sitä kaikille sivuille: OK:n jälkeen kaikkien sivujen suuntaisviivat ilmestyvät kerralla. Enter lopettaa."], ["Painike 3: huoneet ja mittaikkuna", "Paina <b>3. Huoneet: numerot, kirjaimet ja mitat</b>, napsauta huoneita samassa järjestyksessä 1…7 ja paina Enter. Ikkuna avautuu huoneet 1–7 allekkain; alempana osio <b>Täysseinät huoneiden välillä (viivoituksesta)</b> valmiiksi ehdotettuine riveineen: syötä 0,30, missä ei ole yhteistä ovea; missä on, paksuus otetaan ovesta."], ["Sivut, lävistäjät ja ovet", "Vieritä ikkunaa ja syötä jokaisen huoneen sivut ja lävistäjät. Paina sitten <b>＋ Lisää ovi tai ikkuna</b> ja napsauta kunkin oven P1…P6 seinää: liitännäinen löytää molemmat sivut, kysyy etäisyydet, leveyden ja paksuuden ja palaa ikkunaan. Ikkunat lisätään samoin ulkoseinään."], ["Laskenta", "Kun jokainen huone näyttää ✔ ja huoneet on yhdistetty, paina <b>▶ Laske</b>: huoneiden moniviivat saavat lasketun muodon ja piirustus päivittyy itsestään. Ikkuna <b>Laskennan tulos</b> kokoaa ylimääräiset mitat ja σ₀:n; taulukossa punaiset jäännökset merkitsevät uudelleen tarkistettavat mitat. <b>Mittakirja</b> tallentaa kaiken CSV:nä."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Mitta (huone 1)", "Arvo", "Huomautus"], ["1A–1B", "4,10", "sivu"], ["1B–1C", "8,30", "sivu"], ["1C–1D", "4,58", "sivu ulkoreunalla"], ["1D–1A", "8,31", "sivu ulkoreunalla (vino)"], ["1A–1C", "9,26", "tarkistuslävistäjä"], ["1B–1D", "9,48", "toinen lävistäjä (valinnainen)"]],
    "ex_doors": [["Ovi", "Ensimmäinen huone: sivu, etäisyys", "Toinen huone: sivu, etäisyys", "Leveys", "Paksuus"], ["P1", "4B–4C, 4B:stä 1,60", "5D–5A, 5A:sta 1,60", "0,90", "0,30"], ["P2", "5C–5D, 5D:stä 1,30", "2A–2B, 2A:sta 1,30", "0,90", "0,30"], ["P3", "5B–5C, 5B:stä 5,00", "6D–6A, 6A:sta 1,10", "0,90", "0,30"], ["P4", "5B–5C, 5B:stä 1,00", "7D–7A, 7A:sta 1,00", "0,90", "0,30"], ["P5", "4C–4D, 4C:stä 1,10", "1A–1B, 1B:stä 1,10", "0,90", "0,30"], ["P6", "6C–6D, 6D:stä 1,30", "3A–3B, 3A:sta 1,30", "0,80", "0,30"]],
    "ex_end": "Kun ulkoreuna on kerran mitattu, yksikään mitta ei kulje seinien läpi: sivut ja lävistäjät mitataan huoneiden sisällä, paksuudet ovista ja ikkunoista.",
    "end": "Room Survey on harrasteprojekti, joka ei sovellu ammattikäyttöön: tekijä ei ota vastuuta sen käytöstä. "
           "Koodi ja päivitykset: github.com/Macchecaspita/opencad-rilievo-plugin",
    "h_holes": 'Ovet ja ikkunat piirustukseen: painike «Ovet ja ikkunat»',
    "p_holes": 'Laskennan jälkeen painike <b>Ovet ja ikkunat</b> avaa aukot seiniin: se katkaisee seinän viivat, piirtää pielet, tyhjentää rasterin ja piirtää karmin mittoineen.',
    "cap_holes": 'Ikkuna, leveys 120 ja korkeus 140 cm, 1,50 kulmasta 1A: karmi seinän sisäpinnassa; rakennuksen ulkopuolella harmaa sulje ja mitta leveys korkeuden yläpuolella senttimetreinä.',
    "holes": ['paina <b>Ovet ja ikkunat</b> ja valitse <b>ovi</b> tai <b>ikkuna</b>, <b>leveys</b> ja <b>korkeus</b> (arvot ehdotetaan uudelleen seuraavalle aukolle);', 'napsauta <b>kulmaa</b>, josta mittasit: ulkoreunan tai huoneen kulmaa;', 'napsauta <b>sivua</b>, jolla aukko on, ja kirjoita <b>etäisyys</b> kulmasta pieleen;', 'laajennus katkaisee napsautetun sivun ja seinän toisella puolella olevan viivan (vaikka se ei olisi yhdensuuntainen), piirtää pielet ja tyhjentää rasterin aukosta;', 'tasolle <b>RILIEVO_APERTURE</b> ilmestyvät karmi ja mitta leveys/korkeus senttimetreinä, rakennuksen ulkoreunan ulkopuolelle.'],
    "p_holes2": 'Ikkunan karmi on seinän sisäpinnassa; ovi jättää aukon vapaaksi. Komento toimii vain piirustuksessa: katkaistu huone ei ole enää suljettu moniviiva eikä kuulu laskentaan, joten avaa aukot laskennan <b>jälkeen</b>. Ctrl+Z kumoaa viimeisen aukon.',
}
