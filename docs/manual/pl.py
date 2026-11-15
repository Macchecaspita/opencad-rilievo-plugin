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
    "d_sketch": "szkic: ponumerowane pomieszczenia, narożniki z literami (napisanymi przez Litery narożników)",
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
    "p2b": "Poleceniem <b>Litery narożników</b> zaznacz pomieszczenia — po jednym w kolejności, jaką chcesz nadać numerom, "
           "albo wszystkie naraz oknem (numerowane z góry na dół, od lewej do prawej) — i naciśnij Enter: w narożnikach "
           "pojawiają się litery A, B, C …, a w każdym pomieszczeniu jego numer i powierzchnia; narożnik B pomieszczenia 2 "
           "nazywa się 2B. Małe okno pyta o wysokość liter: zmniejsz ją tam, gdzie narożniki są blisko. Wydrukuj rzut: "
           "to Twój szkic.",
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
    "p4b": "Z listy w oknie wybierz <b>Dodaj drzwi lub okno</b> i <b>kliknij na rysunku</b>, na ścianie między dwoma "
           "pomieszczeniami, blisko drzwi. Wtyczka sama rozpoznaje oba boki (podświetla je na zielono) i pyta o:",
    "t4": [["Pole", "Przykład"],
           ["Odległość od narożnika 1B do ościeża (mierzona w pomieszczeniu 1)", "1,00"],
           ["Szerokość", "0,90"],
           ["Grubość ściany, mierzona w drzwiach", "0,30"],
           ["Odległość od narożnika 2D do ościeża (mierzona w pomieszczeniu 2)", "1,10"]],
    "p4c": "Odległość mierzy się od narożnika do ościeża; wtyczka proponuje narożnik najbliższy klikniętemu punktowi, a "
           "możesz wybrać drugi. To wystarczy: oba pomieszczenia są powiązane (ościeża naprzeciw siebie, ściana "
           "równoległa, znana grubość). Jeden dodatkowy pomiar przez drzwi (np. od 1B do 2D) służy jako kontrola.",
    "p4d": "Bez drzwi (lub dla kontroli) wybierz z listy <b>Inne pomiary między pomieszczeniami</b>:",
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
    "fix": ["narysuj obrys jako <b>zamkniętą polilinię</b> w rzeczywistych współrzędnych, a pomieszczenia na oko w jego wnętrzu;", "po uruchomieniu <b>Boki i przekątne</b> otwiera się okno z pytaniem o <b>stały obrys</b> (opcjonalnie), zanim wybierzesz pomieszczenia: wybierz kliknięcie go na rysunku albo kontynuuj bez niego. Można go też dodać później z listy w oknie pomiarów;", "na stronie <b>Ściany zewnętrzne</b> wpisz grubość ścian zewnętrznych (zmierzoną w oknach) i sprawdź boki pomieszczeń zaproponowane wzdłuż obrysu: każdy pozostaje równoległy do obrysu, w tej odległości;", "pomieszczenie narożne z dwoma bokami na obrysie nie potrzebuje przekątnych do ustalenia położenia; pomieszczenie z jednym bokiem na obrysie trzeba nadal powiązać z innymi (np. drzwiami)."],
    "h_hatch": "Pełne ściany między pomieszczeniami: kreskowanie",
    "p_hatch": "Przez pełną ścianę nie da się mierzyć, ale jej grubość mierzy się w drzwiach. Jeśli na rysunku wstawisz <b>kreskowanie</b> w ściany między pomieszczeniami, wtyczka sama je rozpozna:",
    "hatch": ["znajduje dwa zwrócone do siebie boki różnych pomieszczeń z kreskowaniem pomiędzy nimi i proponuje je na stronie <b>Pełne ściany między pomieszczeniami</b>;", "wpisz <b>grubość zmierzoną w drzwiach</b>; jeśli w tej samej ścianie są już wspólne drzwi z grubością, pole może zostać puste;", "w obliczeniu oba boki pozostają równoległe w tej odległości, z odchyłką w tabeli."],
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
    "calc": [
        "<b>Boki i przekątne</b>: kliknij wszystkie pomieszczenia (w tej samej kolejności co na szkicu) i naciśnij Enter. "
        "Otwiera się lista pomieszczeń z ich stanem: otwieraj po jednym pomieszczeniu i wpisuj boki i przekątne, dodawaj "
        "drzwi kliknięciem na rysunku; gdy wszystko jest kompletne, lista proponuje <b>Oblicz</b>.",
        "Jeśli pomiarów jest za mało lub się nie zgadzają, okno otwiera się ponownie z przyczyną i z już wpisanymi pomiarami.",
        "Wynik trafia na warstwę <b>RILIEVO_CORRETTO</b>, z powierzchnią każdego pomieszczenia; wymiary, przekątne i "
        "tabela na <b>RILIEVO_QUOTE</b>; drzwi i okna na <b>RILIEVO_APERTURE</b>. Rysunek na oko zostaje do porównania. "
        "<b>Zastosuj na rysunku</b> przenosi potem oryginalne polilinie do obliczonego kształtu.",
        "Przeczytaj tabelę odchyłek: czerwona odchyłka wskazuje pomiar do ponownego sprawdzenia. Przy jednym pomiarze "
        "nadliczbowym błąd rozkłada się na kilka pomiarów: kolejny pomiar pomaga go znaleźć.",
        "<b>Dziennik pomiarów</b> zapisuje wszystko w pliku CSV dla Excela lub Calc (pomiary, odchyłki, współrzędne, "
        "powierzchnie).",
    ],
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
    "end": "Room Survey to projekt hobbystyczny, nieprzeznaczony do użytku zawodowego: autor nie ponosi "
           "odpowiedzialności za jego użycie. Kod i aktualizacje: github.com/Macchecaspita/opencad-rilievo-plugin",
}
