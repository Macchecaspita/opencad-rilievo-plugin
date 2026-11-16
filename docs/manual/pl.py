"""Teksty podręcznika — polski."""

OUT = "manual_pl.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — podręcznik pomiaru",
    "foot": "Room Survey — podręcznik pomiaru · projekt hobbystyczny, nie do użytku zawodowego",
    "subtitle": "Jak zmierzyć mieszkanie pomiarami bezpośrednimi<br/>i odtworzyć je w Open CAD Studio",
    "warn": "<b>Uwaga.</b> Room Survey to projekt hobbystyczny, zrobiony dla zabawy i <b>nieprzeznaczony do użytku "
            "zawodowego</b>. Jest udostępniany w obecnej postaci, bez żadnej gwarancji; autor nie ponosi żadnej "
            "odpowiedzialności za jego użycie ani za wyniki. Zawsze sprawdzaj wyniki pomiarami kontrolnymi.",

    "d_room": "Pomieszczenie {r}",
    "d_sketch": "szkic: ponumerowane pomieszczenia, narożniki z literami (wpisane przyciskiem 3. Pomieszczenia)",
    "d_check": "B–D kontrola",
    "d_out1": "C–E: biegnie na zewnątrz,",
    "d_out2": "nie do zmierzenia",
    "d_from1b": "odległość od 1B",
    "d_width": "0,90 szerokość",
    "d_thick": "grubość 0,30 (w ościeżu)",
    "d_from2d": "1,10 odległość od 2D",
    "d_furn": "mebel",
    "d_blocked": "A–C zablokowana",

    "h1": "1. Metoda w skrócie",
    "p1": "Rzut rysuje się <b>na oko</b>, na miejscu mierzy się boki i kilka przekątnych każdego pomieszczenia oraz kilka "
          "pomiarów wiążących pomieszczenia ze sobą, a wtyczka odtwarza <b>rzeczywisty</b> rzut, wyrównując pomiary "
          "metodą najmniejszych kwadratów. Odchyłki od razu pokazują, czy któryś pomiar jest błędny.",
    "t1": [["", "Gdzie", "Co się robi"],
           ["1", "przy biurku", "szkic: każde pomieszczenie rysuje się jako zamkniętą polilinię (na oko) i wpisuje litery"],
           ["2", "na miejscu", "mierzy się boki, przekątne, grubości ścian, drzwi i okna, zapisując je na szkicu"],
           ["3", "przy biurku", "wprowadza się pomiary, oblicza, odczytuje odchyłki, uzupełnia rysunek"]],
    "h_tools": "Sprzęt",
    "tools": [
        "dalmierz laserowy (najlepiej mierzący od tyłu urządzenia) i taśma miernicza 5 m do krótkich pomiarów "
        "(grubości, ościeża, uskoki);",
        "wydrukowany szkic, ołówek i gumka; sztywna podkładka;",
        "miarka składana lub kątownik do sprawdzania kątów, gdy przekątne są zasłonięte;",
        "aparat lub telefon: jedno zdjęcie na pomieszczenie, z literami narożników wpisanymi ołówkiem na szkicu.",
    ],

    "h2": "2. Przed wizytą: szkic w CAD",
    "p2a": "Narysuj <b>każde pomieszczenie</b> jako <b>zamkniętą polilinię</b> (PLINE … Zamknij), wzdłuż <b>wewnętrznego "
           "lica</b> ścian. Wymiary nie mają znaczenia: liczy się, by każdy rzeczywisty narożnik był wierzchołkiem "
           "polilinii, we właściwej kolejności. Uskoki, wnęki, wystające filary i fazy to również wierzchołki "
           "(pomieszczenie 3 w przykładzie). Korytarz jest pomieszczeniem jak każde inne.",
    "p2b": "Naciśnij <b>3. Pomieszczenia: numery, litery i pomiary</b> i zaznacz pomieszczenia — po jednym w kolejności, jaką chcesz nadać numerom, albo wszystkie naraz oknem (numerowane z góry na dół, od lewej do prawej) — i naciśnij Enter: w każdym pomieszczeniu pojawia się jego numer pogrubiony, a w narożnikach litery A, B, C …; narożnik B pomieszczenia 2 nazywa się 2B. Otwiera się też okno pomiarów: na razie zamknij je przyciskiem <b>Zamknij</b> i odpowiedz, że chcesz <b>zachować</b> litery. Wydrukuj rzut: to twój szkic.",
    "t2": [["Wskazówka", "Dlaczego"],
           ["Numeruj pomieszczenia w kolejności, którą zachowasz też na miejscu", "Pisze się szybciej i rzadziej myli pomieszczenia."],
           ["Polilinia na wewnętrznym licu", "Pomiary bierze się z wnętrza pomieszczeń; ściany zewnętrzne wyznacza się później z grubości."],
           ["Żadnych zbędnych wierzchołków", "Wierzchołek na prostym boku (kąt 180°) wymaga zbędnych pomiarów."]],

    "h3": "3. Na miejscu: pomiar pomieszczenia",
    "p3a": "Pomieszczenie o <b>n</b> narożnikach ma 2n − 3 niewiadomych: zamyka się je <b>wszystkimi bokami</b> oraz "
           "<b>n − 3 przekątnymi</b>. Prostokąt wymaga 1 przekątnej, pomieszczenie w kształcie L — 3, łazienka z "
           "uskokiem z przykładu — 3.",
    "p3b": "Mierz boki <b>na tej samej wysokości</b> (np. 1 m nad podłogą, powyżej listew i niskich mebli), od narożnika "
           "do narożnika. Potem przekątne. Przekątne proponowane przez wtyczkę przebiegają zawsze <b>wewnątrz</b> "
           "pomieszczenia: w pomieszczeniu w kształcie L nigdy nie ma przekątnej przecinającej ścianę.",
    "cap3": "Z lewej: boki (niebieskie) i przekątna (pomarańczowa) pomieszczenia nieprostokątnego; druga przekątna "
            "(zielona) to pomiar kontrolny. Z prawej: pomieszczenie w kształcie L, trzy przydatne przekątne i jedna, "
            "której należy unikać.",
    "h_check": "Pomiar kontrolny",
    "p_check": "<b>Zawsze bierz o jedną przekątną więcej</b> niż minimum. Bez pomiarów nadliczbowych kształt jest dokładny "
               "bez względu na to, co wpiszesz, a źle odczytana cyfra (6,85 zamiast 6,58) przejdzie niezauważona. Z "
               "jednym pomiarem więcej wtyczka oblicza <b>odchyłki</b> i σ₀ i zaznacza na czerwono pomiary poza "
               "tolerancją (zwykle 1–2 cm).",
    "h_obst": "Gdy mebel zasłania przekątną",
    "obst": [
        "Najpierw spróbuj innej przekątnej (B–D zamiast A–C) albo poprowadź ją nad meblem, trzymając dalmierz w poziomie.",
        "Jeśli to niemożliwe, użyj <b>warunku</b> (na karcie pomieszczenia): narożnik i kąt, np. <b>A, 90</b>, po "
        "sprawdzeniu kątownikiem; albo bok <b>poziomy</b> lub <b>pionowy</b>. Każdy warunek liczy się jako pomiar i "
        "także otrzymuje odchyłkę.",
        "Jeśli nadal czegoś brakuje, opcja <b>uzupełnij kształt kątami z rysunku</b> (zwykle włączona) bierze brakujące "
        "dane z rysunku, a tabela zaznacza to na czerwono. W tym miejscu wynik jest tyle wart, co Twój rysunek na oko: "
        "zanotuj to.",
    ],

    "h4": "4. Wiązanie pomieszczeń ze sobą",
    "p4a": "Każde pomieszczenie zmierzone osobno ma właściwy kształt, ale nie wie, gdzie leży względem pozostałych. Aby "
           "je powiązać, potrzeba <b>co najmniej 3 pomiarów</b> do sąsiedniego pomieszczenia albo <b>wspólnych "
           "drzwi</b>. Drzwi to najwygodniejszy sposób.",
    "cap4": "Drzwi między pomieszczeniem 1 a pomieszczeniem 2: odległość od narożnika i szerokość mierzone z każdej "
            "strony, grubość ściany mierzona w ościeżu.",
    "p4b": "W oknie pomiarów, pod pomieszczeniami, naciśnij <b>＋ Dodaj drzwi lub okno</b> i <b>kliknij na rysunku</b>, na ścianie między dwoma pomieszczeniami, blisko drzwi. Wtyczka sama znajduje oba boki (podświetla je na zielono) i pyta o:",
    "t4": [["Pole", "Przykład"],
           ["Odległość od narożnika 1B do ościeża (mierzona w pomieszczeniu 1)", "1,00"],
           ["Szerokość", "0,90"],
           ["Grubość ściany, mierzona w drzwiach", "0,30"],
           ["Odległość od narożnika 2D do ościeża (mierzona w pomieszczeniu 2)", "1,10"]],
    "p4c": "Odległość mierzy się od narożnika do ościeża; wtyczka proponuje narożnik najbliższy klikniętemu punktowi, a "
           "możesz wybrać drugi. To wystarczy: oba pomieszczenia są powiązane (ościeża naprzeciw siebie, ściana "
           "równoległa, znana grubość). Jeden dodatkowy pomiar przez drzwi (np. od 1B do 2D) służy jako kontrola.",
    "p4d": "Bez drzwi (lub dla kontroli) użyj sekcji <b>Inne pomiary między pomieszczeniami</b> na dole okna:",
    "links": [
        "<b>grubość ściany</b>: Od = bok (1B-1C), Do = narożnik drugiego pomieszczenia za tą ścianą (2A), "
        "Zmierzony = grubość wzięta prostopadle w drzwiach;",
        "<b>pomiary przez drzwi</b>: Od = 1B, Do = 2D, między narożnikami obu pomieszczeń widocznymi przez drzwi; "
        "weź dwa do różnych narożników.",
    ],
    "h_order": "Zalecana kolejność dla przykładowego mieszkania",
    "p_order": "Zacznij od <b>korytarza</b> (pomieszczenie 4), który styka się z prawie wszystkimi innymi, i powiąż każde "
               "pomieszczenie z korytarzem przez jego drzwi: salon i sypialnię z lewej, kuchnię i sypialnię 2 z prawej; "
               "łazienkę wiąże się z sypialnią. Dzięki temu każde pomieszczenie wisi na krótkim łańcuchu, a błąd "
               "pozostaje lokalny.",

    "h_fix": "Stały obrys zmierzony tachimetrem",
    "p_fix": "Jeśli obrys zewnętrzny budynku zmierzono tachimetrem (albo i tak jest dokładny), wtyczka używa go jako stałego odniesienia: nigdy nie jest przesuwany, a pomieszczenia same ustawiają się w jego wnętrzu.",
    "fix": ["narysuj obrys jako <b>zamkniętą polilinię</b>, w prawdziwych współrzędnych, a pomieszczenia na oko wewnątrz niego;", "naciśnij <b>1. Obrys zewnętrzny</b>, kliknij polilinię i naciśnij Enter (Enter bez kliknięcia = brak obrysu);", "naciśnij <b>2. Grubości ścian zewnętrznych</b> i klikaj boki obrysu po kolei: dla każdego wpisz grubość ściany do wewnątrz (zmierzoną w oknach). Boki z grubością stają się zielone; grubości zostają zapisane na rysunku, na warstwie RILIEVO_PERIMETRO. Enter, aby zakończyć;", "w obliczeniu boki pomieszczeń wzdłuż obrysu pozostają do niego równoległe, w tej odległości: pomieszczenie narożne z dwoma bokami na obrysie nie potrzebuje przekątnych do swojego położenia; pomieszczenie z jednym bokiem na obrysie trzeba jeszcze powiązać z innymi (na przykład drzwiami)."],
    "h_hatch": "Pełne ściany między pomieszczeniami: kreskowanie",
    "p_hatch": "Przez pełną ścianę nie da się mierzyć, ale jej grubość mierzy się w drzwiach. Jeśli na rysunku wstawisz <b>kreskowanie</b> w ściany między pomieszczeniami, wtyczka sama je rozpozna:",
    "hatch": ["znajduje dwa zwrócone do siebie boki różnych pomieszczeń z kreskowaniem pomiędzy nimi i proponuje je w sekcji <b>Pełne ściany między pomieszczeniami</b> okna pomiarów;", "wpisz <b>grubość zmierzoną w drzwiach</b>; jeśli w tej samej ścianie są już wspólne drzwi z grubością, pole może zostać puste;", "w obliczeniu oba boki pozostają równoległe w tej odległości, z odchyłką w tabeli."],
    "p_hatch2": "Pełna ściana ustala odległość i obrót między dwoma pomieszczeniami, ale nie przesunięcie wzdłuż ściany: potrzebne są jeszcze drzwi w tej samej ścianie, druga, nierównoległa ściana do tego samego pomieszczenia albo stały obrys.",

    "h5": "5. Drzwi, okna i ściany zewnętrzne",
    "h_dw": "Drzwi i okna",
    "p_dw": "Okna i drzwi zewnętrzne również dodaje się przez <b>Dodaj drzwi lub okno</b> i kliknięcie na ścianie: typ "
            "(<b>drzwi</b>, <b>okno</b>, <b>otwór</b>), odległość od narożnika do ościeża i szerokość w świetle. Drzwi "
            "są zawieszone od strony narożnika, od którego mierzysz. Wysokości (parapet, nadproże) zanotuj na szkicu: "
            "wtyczka pracuje w rzucie.",
    "h_ext": "Ściany zewnętrzne",
    "p_ext": "Wtyczka odtwarza pomieszczenia na wewnętrznym licu. Dla obrysu zewnętrznego zmierz <b>grubość ściany w "
             "oknach</b> (z ościeżem), co najmniej jedną na elewację; potem w CAD wyznacz lico zewnętrzne poleceniem "
             "OFFSET od boków pomieszczeń, w zmierzonej odległości, i połącz narożniki. Gdzie się da, sprawdź to "
             "pomiarem z zewnątrz (długością elewacji).",

    "h6": "6. Przy biurku: obliczenie i kontrola",
    "calc": ["Naciśnij <b>3. Pomieszczenia: numery, litery i pomiary</b>, zaznacz wszystkie pomieszczenia (w kolejności szkicu) i naciśnij Enter. Otwiera się <b>jedno okno</b> ze wszystkimi pomieszczeniami jedno pod drugim, od 1 do ostatniego, każde z tytułem i stanem: przewijaj je paskiem po prawej i wpisuj boki i przekątne. Pod pomieszczeniami są drzwi i okna, pełne ściany, inne pomiary między pomieszczeniami i opcje.", "Naciśnij <b>▶ Oblicz</b>: polilinie pomieszczeń zostają przeniesione do obliczonego kształtu, a <b>rysunek aktualizuje się sam</b> (Ctrl+Z cofa). Jeśli pomiarów jest za mało lub się nie zgadzają, okno otwiera się ponownie z powodem i z już wpisanymi pomiarami.", "Wymiary, przekątne i tabela trafiają na <b>RILIEVO_QUOTE</b>; drzwi i okna na <b>RILIEVO_APERTURE</b>. Okno <b>Wynik obliczenia</b> podsumowuje powierzchnie, pomiary nadliczbowe i σ₀.", "Przeczytaj tabelę odchyłek: czerwona odchyłka wskazuje pomiar do ponownego sprawdzenia. Przy jednym pomiarze nadliczbowym błąd rozkłada się na kilka pomiarów: wykonanie kolejnego pomaga go znaleźć.", "<b>Dziennik pomiarów</b> zapisuje wszystko w pliku CSV dla Excela lub Calc (pomiary, odchyłki, współrzędne, powierzchnie)."],
    "h_vals": "Wartości orientacyjne",
    "t6": [["Wielkość", "Typowa wartość", "Jeśli jest większa"],
           ["odchyłka boku lub przekątnej", "do 1 cm", "zmierz ponownie; sprawdź przestawione cyfry i właściwy narożnik"],
           ["σ₀ pomieszczenia", "0,3–0,8 cm", "pomiary na różnych wysokościach, nieostre narożniki"],
           ["kąt ustalony na 90°", "odchyłka do 0,2°", "kąt nie jest prosty: usuń warunek i zmierz przekątną"]],
    "h_list": "Lista kontrolna przed opuszczeniem budowy",
    "list": [
        "każde pomieszczenie ma wszystkie boki i co najmniej n − 3 przekątne (lub sprawdzone warunki) oraz pomiar kontrolny;",
        "każde pomieszczenie jest powiązane: wspólne drzwi z grubością albo 3 pomiary do już powiązanego pomieszczenia;",
        "grubości ścian zewnętrznych zmierzone w oknach, co najmniej jedna na elewację;",
        "drzwi i okna: odległość od narożnika, szerokość, zanotowane wysokości;",
        "jedno zdjęcie na pomieszczenie.",
    ],
    "ex_h": "7. Pełny przykład, krok po kroku",
    "ex_p": "Mieszkanie z siedmioma pomieszczeniami w budynku o pochylonej lewej elewacji i ze skrzydłem o ściętym narożniku. Obrys zewnętrzny zmierzono tachimetrem; ściany wewnętrzne są pełne, a ich grubość można zmierzyć tylko w drzwiach.",
    "ex_fig": "zmierzony obrys E1–E7, pomieszczenia 1–7 narysowane na oko, pełne ściany zakreskowane",
    "ex_cap": "Rysunek wyjściowy: stały obrys E1–E7, ponumerowane pomieszczenia z literami narożników, ściany wypełnione kreskowaniem, drzwi P1–P6 i okna (na niebiesko).",
    "ex_ties": [["Pom.", "Boki na obrysie", "Pełne ściany (kreskowanie)", "Drzwi"], ["1", "C–D, D–A", "A–B z 4; B–C z 2", "P5"], ["2", "C–D", "D–A z 1; A–B z 5; B–C z 3", "P2"], ["3", "B–C, C–D, D–E", "A–B z 6; E–A z 2", "P6"], ["4", "A–B, D–A", "C–D z 1; B–C z 5", "P1, P5"], ["5", "A–B", "D–A z 4; C–D z 2; B–C z 6 i 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A z 5; A–B z 7; C–D z 3", "P3, P6"], ["7", "A–B, B–C", "C–D z 6; D–A z 5", "P4"]],
    "ex_ties_note": "Sześć z siedmiu pomieszczeń styka się z obrysem dwoma nierównoległymi bokami: sam obrys ustawia je na miejscu. Pomieszczenie 2 ma tylko jeden bok na obrysie, ale pełna ściana do pomieszczenia 1 (nierównoległa) je blokuje. Pełne ściany i drzwi służą więc także jako kontrola.",
    "ex_steps": [["Obrys z tachimetru", "Zmierz tachimetrem zewnętrzne narożniki budynku E1…E7 i połącz je w zamkniętą polilinię w rzeczywistych współrzędnych. To jedyny dokładny element rysunku: wtyczka nigdy go nie przesunie."], ["Pomieszczenia na oko", "Wewnątrz obrysu narysuj każde pomieszczenie jako zamkniętą polilinię na wewnętrznym licu ścian. Liczą się tylko liczba i kolejność narożników: pomieszczenie 3 ma pięć narożników, bo ściana zewnętrzna jest ścięta, pomieszczenia 1 i 4 biegną wzdłuż pochylonej elewacji."], ["Kreskowanie w pełnych ścianach", "Poleceniem kreskowania (HATCH) wypełnij przestrzeń między obrysem a pomieszczeniami, czyli wszystkie ściany. Kreskowanie mówi wtyczce, które pomieszczenia są oddzielone pełną ścianą: nie musi być dokładne, wystarczy, że leży między dwoma bokami."], ["Litery", "Naciśnij <b>3. Pomieszczenia: numery, litery i pomiary</b>, klikaj pomieszczenia w kolejności 1…7 z rysunku i naciśnij Enter. Numery i litery pojawiają się od razu i otwiera się okno pomiarów: zamknij je i wybierz zachowanie liter. Wydrukuj rysunek: to szkic do zabrania na budowę."], ["Na miejscu: pomieszczenia", "W każdym pomieszczeniu zmierz boki na tej samej wysokości i co najmniej jedną przekątną. Pomieszczeniom z dwoma bokami na obrysie przekątne nie są potrzebne do kształtu, ale dodatkowa przekątna to jedyny sposób, by zauważyć źle odczytaną cyfrę. Na przykład dla pomieszczenia 1:"], ["Na miejscu: grubości i drzwi", "Zmierz grubość ścian zewnętrznych w oknach (tu 0,40), a ścian wewnętrznych w drzwiach (tu 0,30). Dla każdych drzwi zapisz z obu stron odległość od narożnika do ościeża i szerokość:"], ["Przyciski 1 i 2: obrys i grubości", "W biurze naciśnij <b>1. Obrys zewnętrzny</b>, kliknij polilinię E1…E7 i naciśnij Enter. Potem naciśnij <b>2. Grubości ścian zewnętrznych</b>, kliknij pierwszy bok, wpisz 0,40 i zaznacz pole, by użyć jej dla wszystkich boków: wszystkie stają się zielone. Enter, aby zakończyć."], ["Przycisk 3: pomieszczenia i okno pomiarów", "Naciśnij <b>3. Pomieszczenia: numery, litery i pomiary</b>, klikaj pomieszczenia w tej samej kolejności 1…7 i naciśnij Enter. Otwiera się okno z pomieszczeniami od 1 do 7, jedno pod drugim; niżej sekcja <b>Pełne ściany między pomieszczeniami (z kreskowania)</b> z już zaproponowanymi wierszami: wpisz 0,30 tam, gdzie nie ma wspólnych drzwi, tam gdzie są, grubość jest brana z drzwi."], ["Boki, przekątne i drzwi", "Przewijaj okno i wpisuj boki i przekątne każdego pomieszczenia. Potem naciśnij <b>＋ Dodaj drzwi lub okno</b> i kliknij ścianę każdych drzwi P1…P6: wtyczka znajduje oba boki i pyta o odległości, szerokość i grubość, po czym wraca do okna. Okna dodaje się tak samo, na ścianie zewnętrznej."], ["Oblicz", "Gdy każde pomieszczenie pokazuje ✔ i pomieszczenia są powiązane, naciśnij <b>▶ Oblicz</b>: polilinie pomieszczeń przyjmują obliczony kształt, a rysunek aktualizuje się sam. Okno <b>Wynik obliczenia</b> podsumowuje pomiary nadliczbowe i σ₀; w tabeli czerwone odchyłki wskazują pomiary do sprawdzenia. <b>Dziennik pomiarów</b> zapisuje wszystko w pliku CSV."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Pomiar (pomieszczenie 1)", "Wartość", "Uwaga"], ["1A–1B", "4,10", "bok"], ["1B–1C", "8,30", "bok"], ["1C–1D", "4,58", "bok na obrysie"], ["1D–1A", "8,31", "bok na obrysie (pochylony)"], ["1A–1C", "9,26", "przekątna kontrolna"], ["1B–1D", "9,48", "druga przekątna (opcjonalna)"]],
    "ex_doors": [["Drzwi", "Pierwsze pom.: bok, odległość", "Drugie pom.: bok, odległość", "Szerokość", "Grubość"], ["P1", "4B–4C, od 4B 1,60", "5D–5A, od 5A 1,60", "0,90", "0,30"], ["P2", "5C–5D, od 5D 1,30", "2A–2B, od 2A 1,30", "0,90", "0,30"], ["P3", "5B–5C, od 5B 5,00", "6D–6A, od 6A 1,10", "0,90", "0,30"], ["P4", "5B–5C, od 5B 1,00", "7D–7A, od 7A 1,00", "0,90", "0,30"], ["P5", "4C–4D, od 4C 1,10", "1A–1B, od 1B 1,10", "0,90", "0,30"], ["P6", "6C–6D, od 6D 1,30", "3A–3B, od 3A 1,30", "0,80", "0,30"]],
    "ex_end": "Gdy obrys jest zmierzony raz, żaden pomiar nie przechodzi przez ściany: boki i przekątne mierzy się w pomieszczeniach, grubości w drzwiach i oknach.",
    "end": "Room Survey to projekt hobbystyczny, nieprzeznaczony do użytku zawodowego: autor nie ponosi "
           "odpowiedzialności za jego użycie. Kod i aktualizacje: github.com/Macchecaspita/opencad-rilievo-plugin",
}
