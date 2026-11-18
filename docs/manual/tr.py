"""Kılavuz metinleri — Türkçe."""

OUT = "manual_tr.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — ölçüm kılavuzu",
    "foot": "Room Survey — ölçüm kılavuzu · hobi projesi, profesyonel kullanım için değildir",
    "subtitle": "Bir daire doğrudan ölçülerle nasıl rölöve edilir<br/>ve Open CAD Studio'da nasıl yeniden çizilir",
    "warn": "<b>Uyarı.</b> Room Survey bir hobi projesidir; eğlence için yapılmıştır ve <b>profesyonel kullanıma "
            "uygun değildir</b>. Olduğu gibi, hiçbir garanti olmadan sunulur; yazar, kullanımından veya sonuçlarından "
            "doğacak hiçbir sorumluluğu kabul etmez. Sonuçları her zaman kontrol ölçüleriyle doğrulayın.",

    "d_room": "Oda {r}",
    "d_sketch": "kroki: numaralı odalar, harfli köşeler (3. Odalar düğmesiyle yazılır)",
    "d_check": "B–D kontrol",
    "d_out1": "C–E: dışarıdan geçer,",
    "d_out2": "ölçülemez",
    "d_from1b": "1B'den uzaklık",
    "d_width": "0,90 genişlik",
    "d_thick": "kalınlık 0,30 (kasada)",
    "d_from2d": "1,10 2D'den uzaklık",
    "d_furn": "mobilya",
    "d_blocked": "A–C engelli",

    "h1": "1. Kısaca yöntem",
    "p1": "Plan <b>göz kararı</b> çizilir, sahada her odanın kenarları ve birkaç köşegeni ile odaları birbirine bağlayan "
          "birkaç ölçü alınır; eklenti ölçüleri en küçük kareler yöntemiyle dengeleyerek <b>gerçek</b> planı yeniden "
          "oluşturur. Artıklar, bir ölçünün yanlış olup olmadığını hemen gösterir.",
    "t1": [["", "Nerede", "Ne yapılır"],
           ["1", "büroda", "kroki: her oda (göz kararı) kapalı bir çoklu çizgi olarak çizilir ve harfler yazılır"],
           ["2", "sahada", "kenarlar, köşegenler, duvar kalınlıkları, kapılar ve pencereler ölçülür, krokiye not edilir"],
           ["3", "büroda", "ölçüler girilir, hesaplanır, artıklar okunur, çizim tamamlanır"]],
    "h_tools": "Ekipman",
    "tools": [
        "lazer metre (tercihen cihazın arkasından ölçen) ve kısa ölçüler (kalınlıklar, söveler, girintiler) için "
        "5 m'lik şerit metre;",
        "basılı kroki, kurşun kalem ve silgi; sert bir altlık;",
        "köşegenler engellendiğinde açıları kontrol etmek için katlanır metre veya gönye;",
        "fotoğraf makinesi veya telefon: her oda için bir fotoğraf, köşe harfleri krokiye kalemle yazılmış olarak.",
    ],

    "h2": "2. Ziyaretten önce: CAD'de kroki",
    "p2a": "<b>Her odayı</b>, duvarların <b>iç yüzünü</b> izleyerek <b>kapalı bir çoklu çizgi</b> (PLINE … Kapat) "
           "olarak çizin. Ölçüler önemli değildir: önemli olan, her gerçek köşenin doğru sırada çoklu çizginin bir "
           "köşesi olmasıdır. Girintiler, nişler, çıkıntılı kolonlar ve pahlar da köşedir (örnekteki 3. oda). "
           "Koridor da diğerleri gibi bir odadır.",
    "p2b": "<b>3. Odalar: numaralar, harfler ve ölçüler</b> düğmesine basın ve odaları seçin — numaralar için istediğiniz sırayla tek tek ya da bir pencereyle hepsini birden (yukarıdan aşağıya, soldan sağa numaralanır) — ve Enter'a basın: her odada numarası kalın olarak, köşelerde A, B, C … harfleri belirir; 2. odanın B köşesine 2B denir. Ölçü penceresi de açılır: şimdilik <b>Kapat</b> ile kapatın ve harfleri <b>tutmayı</b> seçin. Planı yazdırın: krokiniz budur.",
    "t2": [["Öneri", "Neden"],
           ["Odaları sahada da izleyeceğiniz bir sırayla numaralayın", "Daha hızlı yazılır ve oda daha az karıştırılır."],
           ["Çoklu çizgi iç yüzde", "Ölçüler odaların içinden alınır; dış duvarlar sonra kalınlıklardan çıkarılır."],
           ["Gereksiz köşe yok", "Düz bir kenar üzerindeki köşe (180° açı) gereksiz ölçü ister."]],

    "h3": "3. Sahada: bir odayı ölçmek",
    "p3a": "<b>n</b> köşeli bir odanın 2n − 3 bilinmeyeni vardır: <b>tüm kenarlar</b> ve <b>n − 3 köşegen</b> ile "
           "kapanır. Dikdörtgen 1 köşegen ister, L biçimli oda 3, örnekteki girintili banyo 3.",
    "p3b": "Kenarları köşeden köşeye, <b>aynı yükseklikte</b> ölçün (örneğin yerden 1 m, süpürgeliklerin ve alçak "
           "mobilyaların üstünde). Sonra köşegenleri. Eklentinin önerdiği köşegenlerin hepsi odanın <b>içinden</b> "
           "geçer: L biçimli bir odada duvarı kesen bir köşegen asla olmaz.",
    "cap3": "Solda: gönyesiz bir odanın kenarları (mavi) ve köşegeni (turuncu); ikinci köşegen (yeşil) kontrol "
            "ölçüsüdür. Sağda: L biçimli oda, üç yararlı köşegen ve kaçınılması gereken bir tane.",
    "h_check": "Kontrol ölçüsü",
    "p_check": "<b>Her zaman en azdan bir köşegen fazla</b> alın. Fazla ölçü olmadan ne yazarsanız yazın şekil kesindir "
               "ve yanlış okunmuş bir rakam (6,58 yerine 6,85) fark edilmez. Bir fazla ölçüyle eklenti <b>artıkları</b> "
               "ve σ₀'ı hesaplar ve toleransı (genellikle 1–2 cm) aşan ölçüleri kırmızıya boyar.",
    "h_obst": "Bir mobilya köşegeni engellediğinde",
    "obst": [
        "Önce başka bir köşegen deneyin (A–C yerine B–D) veya lazer metreyi terazide tutarak mobilyanın üstünden ölçün.",
        "Bu mümkün değilse bir <b>kısıt</b> kullanın (odanın sayfasında): köşe ve açı, örneğin gönyeyle kontrol "
        "ettikten sonra <b>A, 90</b>; ya da <b>yatay</b> veya <b>dikey</b> bir kenar. Her kısıt bir ölçü sayılır ve "
        "o da bir artık alır.",
        "Hâlâ bir şey eksikse, <b>şekli çizimin açılarıyla tamamla</b> seçeneği (genellikle açık) eksik olanı çizimden "
        "alır ve tablo bunu kırmızıyla gösterir. O noktada sonuç, göz kararı çiziminiz kadar güvenilirdir: not alın.",
    ],

    "h4": "4. Odaları birbirine bağlamak",
    "p4a": "Tek başına ölçülen her oda doğru şekle sahiptir ama diğerlerine göre nerede olduğunu bilmez. Bağlamak için "
           "komşu odaya doğru <b>en az 3 ölçü</b> ya da <b>ortak bir kapı</b> gerekir. En kolay yol kapıdır.",
    "cap4": "1. oda ile 2. oda arasındaki kapı: köşeden uzaklık ve genişlik her iki taraftan ölçülür, duvar kalınlığı "
            "kasada ölçülür.",
    "p4b": "Ölçü penceresinde, odaların altında <b>＋ Kapı veya pencere ekle</b> düğmesine basın ve <b>çizime</b>, iki oda arasındaki duvarın üzerine, kapının yakınına <b>tıklayın</b>. Eklenti iki kenarı kendisi bulur (yeşil gösterir) ve şunları sorar:",
    "t4": [["Kutu", "Örnek"],
           ["1B köşesinden kasaya uzaklık (1. odada ölçülen)", "1,00"],
           ["Genişlik", "0,90"],
           ["Duvar kalınlığı, kapıda ölçülen", "0,30"],
           ["2D köşesinden kasaya uzaklık (2. odada ölçülen)", "1,10"]],
    "p4c": "Uzaklık köşeden kasaya kadar alınır; eklenti tıklanan noktaya en yakın köşeyi önerir, diğerini de "
           "seçebilirsiniz. Bu kadarı yeter: iki oda bağlanmış olur (karşılıklı kasalar, paralel duvar, bilinen "
           "kalınlık). Kapıdan geçen bir ölçü daha (örneğin 1B'den 2D'ye) kontrol işi görür.",
    "p4d": "Kapı yoksa (veya kontrol için) pencerenin altındaki <b>Odalar arası diğer ölçüler</b> bölümünü kullanın:",
    "links": [
        "<b>duvar kalınlığı</b>: Başlangıç = bir kenar (1B-1C), Bitiş = o duvarın arkasındaki diğer odanın köşesi (2A), "
        "Ölçülen = kapıda gönyesinde alınan kalınlık;",
        "<b>kapıdan geçen ölçüler</b>: Başlangıç = 1B, Bitiş = 2D, kapıdan görünen iki odanın köşeleri arasında; "
        "farklı köşelere doğru iki tane alın.",
    ],
    "h_order": "Örnek daire için önerilen sıra",
    "p_order": "Neredeyse diğer tüm odalara değen <b>koridordan</b> (4. oda) başlayın ve her odayı kapısıyla koridora "
               "bağlayın: solda salon ve yatak odası, sağda mutfak ve 2. yatak odası; banyo yatak odasına bağlanır. "
               "Böylece her oda kısa bir zincirle bağlanır ve bir hata sınırlı kalır.",

    "h_fix": "Total station ile ölçülen sabit çevre",
    "p_fix": "Binanın dış çevresi total station ile ölçüldüyse (ya da zaten kesinse), eklenti onu sabit referans olarak kullanır: asla yerinden oynatılmaz ve odalar onun içine kendiliğinden yerleşir.",
    "fix": ["çevreyi gerçek koordinatlarda <b>kapalı çoklu çizgi</b> olarak, odaları da içinde göz kararı çizin;", "<b>1. Dış çevre</b> düğmesine basın, çoklu çizgiye tıklayın ve Enter'a basın (tıklamadan Enter = çevre yok);", "<b>2. Dış duvar kalınlıkları</b> düğmesine basın ve çevrenin kenarlarına tek tek tıklayın: her biri için duvar kalınlığını (pencerelerde ölçülen) girin ve yönü seçin; normal bir dış duvar için <i>merkeze doğru</i> (<i>dışa doğru</i> yalnızca çizgiyi çizer, hesaba girmez). Tamam'a basar basmaz o kenarın paralel çizgisi ve değeri çizimde, RILIEVO_PERIMETRO katmanında görünür: çizgisi olmayan kenar unutulmuş bir kenardır. Bitirmek için Enter: özet, kalınlığı olmayan kenarları listeler;", "hesapta çevre boyunca oda kenarları ona paralel ve bu uzaklıkta kalır: çevrede iki kenarı olan köşe odasının konumu için köşegen gerekmez; çevrede tek kenarı olan oda yine diğerlerine bağlanmalıdır (örneğin bir kapıyla)."],
    "h_hatch": "Odalar arası dolu duvarlar: tarama",
    "p_hatch": "Dolu bir duvarın içinden ölçüm yapılamaz, ama kalınlığı kapılarda ölçülür. Çizimde odalar arasındaki duvarlara <b>tarama</b> koyarsanız eklenti onları kendiliğinden tanır:",
    "hatch": ["aralarında tarama olan, karşılıklı duran farklı odaların iki kenarını bulur ve ölçü penceresinin <b>Odalar arası dolu duvarlar</b> bölümünde önerir;", "<b>bir kapıda ölçülen kalınlığı</b> girin; aynı duvarda kalınlığıyla birlikte ortak bir kapı zaten varsa kutu boş kalabilir;", "hesapta iki kenar bu uzaklıkta paralel kalır, artıkları tabloda görünür."],
    "p_hatch2": "Dolu bir duvar iki oda arasındaki uzaklığı ve dönüklüğü sabitler, ama duvar boyunca kaymayı sabitlemez: yine aynı duvarda bir kapı, aynı odaya doğru paralel olmayan ikinci bir duvar ya da sabit çevre gerekir.",

    "h5": "5. Kapılar, pencereler ve dış duvarlar",
    "h_dw": "Kapılar ve pencereler",
    "p_dw": "Pencereler ve dışarıya açılan kapılar da <b>Kapı veya pencere ekle</b> ve duvara bir tıklamayla eklenir: "
            "tür (<b>kapı</b>, <b>pencere</b>, <b>açıklık</b>), köşeden söveye uzaklık ve net genişlik. Kapı, ölçtüğünüz "
            "köşe tarafından menteşelidir. Yükseklikleri (denizlik, lento) krokiye not edin: eklenti planda çalışır.",
    "h_ext": "Dış duvarlar",
    "p_ext": "Eklenti odaları iç yüzde yeniden oluşturur. Dış çevre için <b>pencerelerde duvar kalınlığını</b> (söve "
             "dahil) ölçün, her cephede en az bir tane; sonra CAD'de odaların kenarlarından ölçülen uzaklıkta OFFSET "
             "ile dış yüzü çizin ve köşeleri birleştirin. Mümkünse dışarıdan alınan bir ölçüyle (bir cephenin "
             "uzunluğu) kontrol edin.",

    "h6": "6. Büroda: hesap ve kontrol",
    "calc": ["<b>3. Odalar: numaralar, harfler ve ölçüler</b> düğmesine basın, tüm odaları (krokideki sırayla) seçin ve Enter'a basın. 1. odadan sonuncusuna kadar tüm odaların alt alta, her birinin başlığı ve durumuyla yer aldığı <b>tek bir pencere</b> açılır: sağdaki çubukla kaydırın ve kenarları ve köşegenleri girin. Odaların altında kapılar ve pencereler, dolu duvarlar, odalar arası diğer ölçüler ve seçenekler bulunur.", "<b>▶ Hesapla</b> düğmesine basın: oda çoklu çizgileri hesaplanan şekle getirilir ve <b>çizim kendiliğinden güncellenir</b> (Ctrl+Z geri alır). Ölçüler yetmezse veya tutmazsa pencere, nedeni ve önceden yazılmış ölçülerle yeniden açılır.", "Ölçülendirmeler, köşegenler ve tablo <b>RILIEVO_QUOTE</b> üzerine; kapılar ve pencereler <b>RILIEVO_APERTURE</b> üzerine gider. <b>Hesap sonucu</b> penceresi alanları, fazla ölçüleri ve σ₀'ı özetler.", "Artıklar tablosunu okuyun: kırmızı bir artık yeniden kontrol edilecek ölçüyü gösterir. Tek bir fazla ölçüyle hata birkaç ölçüye dağılır: bir ölçü daha almak onu bulmaya yardım eder.", "<b>Ölçü defteri</b> her şeyi Excel veya Calc için bir CSV'ye kaydeder (ölçüler, artıklar, koordinatlar, alanlar)."],
    "h_vals": "Yaklaşık değerler",
    "t6": [["Büyüklük", "Tipik değer", "Daha yüksekse"],
           ["bir kenarın veya köşegenin artığı", "1 cm içinde", "yeniden ölçün; yer değiştirmiş rakamları ve doğru köşeyi kontrol edin"],
           ["bir odanın σ₀'ı", "0,3–0,8 cm", "farklı yüksekliklerde alınmış ölçüler, belirsiz köşeler"],
           ["90°'ye kısıtlanmış açı", "artık 0,2° içinde", "açı dik değil: kısıtı kaldırıp bir köşegen ölçün"]],
    "h_list": "Sahadan ayrılmadan önce kontrol listesi",
    "list": [
        "her odanın tüm kenarları ve en az n − 3 köşegeni (veya kontrol edilmiş kısıtları) ve bir kontrol ölçüsü var;",
        "her oda bağlı: kalınlığıyla ortak kapı ya da zaten bağlı bir odaya doğru 3 ölçü;",
        "dış duvar kalınlıkları pencerelerde alındı, her cephede en az bir tane;",
        "kapılar ve pencereler: köşeden uzaklık, genişlik, yükseklikler not edildi;",
        "her oda için bir fotoğraf.",
    ],
    "ex_h": "7. Adım adım eksiksiz bir örnek",
    "ex_p": "Sol cephesi eğik ve köşesi pahlı bir kanadı olan bir binada yedi odalı bir daire. Dış çevre total station ile ölçülmüştür; iç duvarlar doludur ve kalınlıkları yalnızca kapılarda ölçülebilir.",
    "ex_fig": "ölçülmüş çevre E1–E7, göz kararı çizilmiş odalar 1–7, taralı dolu duvarlar",
    "ex_cap": "Başlangıç çizimi: sabit çevre E1–E7, köşe harfleriyle numaralı odalar, taramayla doldurulmuş duvarlar, P1–P6 kapıları ve pencereler (mavi).",
    "ex_ties": [["Oda", "Çevre üzerindeki kenarlar", "Dolu duvarlar (tarama)", "Kapılar"], ["1", "C–D, D–A", "A–B 4 ile; B–C 2 ile", "P5"], ["2", "C–D", "D–A 1 ile; A–B 5 ile; B–C 3 ile", "P2"], ["3", "B–C, C–D, D–E", "A–B 6 ile; E–A 2 ile", "P6"], ["4", "A–B, D–A", "C–D 1 ile; B–C 5 ile", "P1, P5"], ["5", "A–B", "D–A 4 ile; C–D 2 ile; B–C 6 ve 7 ile", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A 5 ile; A–B 7 ile; C–D 3 ile", "P3, P6"], ["7", "A–B, B–C", "C–D 6 ile; D–A 5 ile", "P4"]],
    "ex_ties_note": "Yedi odadan altısı çevreye paralel olmayan iki kenarla değer: çevre tek başına onları yerine koyar. 2. odanın çevre üzerinde tek kenarı var, ama 1. odaya doğru olan (paralel olmayan) dolu duvar onu sabitler. Böylece dolu duvarlar ve kapılar kontrol işi de görür.",
    "ex_steps": [["Total station ile çevre", "Binanın E1…E7 dış köşelerini total station ile ölçün ve gerçek koordinatlarda kapalı bir çoklu çizgiyle birleştirin. Çizimin tek kesin öğesidir: eklenti onu asla oynatmaz."], ["Göz kararı odalar", "Çevrenin içinde her odayı duvarların iç yüzünde kapalı bir çoklu çizgi olarak çizin. Yalnızca köşelerin sayısı ve sırası önemlidir: dış duvar pahlı olduğu için 3. odanın beş köşesi var, 1. ve 4. odalar eğik cepheyi izler."], ["Dolu duvarlarda tarama", "Tarama komutuyla (HATCH) çevre ile odalar arasındaki boşluğu, yani tüm duvarları doldurun. Tarama eklentiye hangi odaların dolu bir duvarla ayrıldığını söyler: hassas olması gerekmez, iki kenar arasında olması yeterlidir."], ["Harfler", "<b>3. Odalar: numaralar, harfler ve ölçüler</b> düğmesine basın, odalara şekildeki 1…7 sırasıyla tıklayın ve Enter'a basın. Numaralar ve harfler hemen belirir ve ölçü penceresi açılır: kapatın ve harfleri tutmayı seçin. Çizimi yazdırın: sahaya götürülecek kroki budur."], ["Sahada: odalar", "Her odada kenarları aynı yükseklikte ve en az bir köşegeni ölçün. Çevre üzerinde iki kenarı olan odaların şekli için köşegen gerekmez, ama fazladan bir köşegen yanlış okunmuş bir rakamı fark etmenin tek yoludur. Örneğin 1. oda için:"], ["Sahada: kalınlıklar ve kapılar", "Dış duvarların kalınlığını pencerelerde (burada 0,40), iç duvarlarınkini kapılarda (burada 0,30) ölçün. Her kapı için iki taraftan köşeden kasaya uzaklığı ve genişliği not edin:"], ["1. ve 2. düğmeler: çevre ve kalınlıklar", "Büroda <b>1. Dış çevre</b> düğmesine basın, E1…E7 çoklu çizgisine tıklayın ve Enter'a basın. Sonra <b>2. Dış duvar kalınlıkları</b> düğmesine basın, ilk kenara tıklayın, 0,40 yazın, yönü <i>merkeze doğru</i> bırakın ve tüm kenarlarda kullanmak için kutuyu işaretleyin: Tamam'dan sonra tüm kenarların paralel çizgileri hemen görünür. Bitirmek için Enter."], ["3. düğme: odalar ve ölçü penceresi", "<b>3. Odalar: numaralar, harfler ve ölçüler</b> düğmesine basın, odalara aynı 1…7 sırasıyla tıklayın ve Enter'a basın. Pencere 1'den 7'ye odalarla, alt alta açılır; daha aşağıda satırları önceden önerilmiş <b>Odalar arası dolu duvarlar (taramadan)</b> bölümü: ortak kapı olmayan yerlere 0,30 yazın, olan yerlerde kapıdan alınır."], ["Kenarlar, köşegenler ve kapılar", "Pencereyi kaydırın ve her odanın kenarlarını ve köşegenlerini girin. Sonra <b>＋ Kapı veya pencere ekle</b> düğmesine basın ve P1…P6 kapılarının her birinin duvarına tıklayın: eklenti iki kenarı bulur, uzaklıkları, genişliği ve kalınlığı sorar, sonra pencereye döner. Pencereler de aynı şekilde, dış duvar üzerinde eklenir."], ["Hesapla", "Her oda ✔ gösterdiğinde ve odalar bağlı olduğunda <b>▶ Hesapla</b> düğmesine basın: oda çoklu çizgileri hesaplanan şekli alır ve çizim kendiliğinden güncellenir. <b>Hesap sonucu</b> penceresi fazla ölçüleri ve σ₀'ı özetler; tabloda kırmızı artıklar yeniden kontrol edilecek ölçüleri gösterir. <b>Ölçü defteri</b> her şeyi bir CSV'ye kaydeder."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Ölçü (1. oda)", "Değer", "Not"], ["1A–1B", "4,10", "kenar"], ["1B–1C", "8,30", "kenar"], ["1C–1D", "4,58", "çevre üzerindeki kenar"], ["1D–1A", "8,31", "çevre üzerindeki kenar (eğik)"], ["1A–1C", "9,26", "kontrol köşegeni"], ["1B–1D", "9,48", "ikinci köşegen (isteğe bağlı)"]],
    "ex_doors": [["Kapı", "Birinci oda: kenar, uzaklık", "İkinci oda: kenar, uzaklık", "Genişlik", "Kalınlık"], ["P1", "4B–4C, 4B'den 1,60", "5D–5A, 5A'dan 1,60", "0,90", "0,30"], ["P2", "5C–5D, 5D'den 1,30", "2A–2B, 2A'dan 1,30", "0,90", "0,30"], ["P3", "5B–5C, 5B'den 5,00", "6D–6A, 6A'dan 1,10", "0,90", "0,30"], ["P4", "5B–5C, 5B'den 1,00", "7D–7A, 7A'dan 1,00", "0,90", "0,30"], ["P5", "4C–4D, 4C'den 1,10", "1A–1B, 1B'den 1,10", "0,90", "0,30"], ["P6", "6C–6D, 6D'den 1,30", "3A–3B, 3A'dan 1,30", "0,80", "0,30"]],
    "ex_end": "Çevre bir kez ölçüldüğünde hiçbir ölçü duvarları kesmez: kenarlar ve köşegenler odaların içinde, kalınlıklar kapılarda ve pencerelerde alınır.",
    "end": "Room Survey bir hobi projesidir, profesyonel kullanıma uygun değildir: yazar kullanımından doğacak hiçbir "
           "sorumluluğu kabul etmez. Kod ve güncellemeler: github.com/Macchecaspita/opencad-rilievo-plugin",
}
