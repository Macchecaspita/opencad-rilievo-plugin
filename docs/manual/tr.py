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
    "d_sketch": "kroki: numaralı odalar, harfli köşeler (Köşe harfleri ile yazılır)",
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
    "p2b": "<b>Köşe harfleri</b> ile odaları seçin — numaralara vermek istediğiniz sırayla tek tek ya da bir pencereyle "
           "hepsini birden (yukarıdan aşağıya, soldan sağa numaralanır) — ve Enter'a basın: köşelerde A, B, C … "
           "harfleri, her odada ise numarası ve alanı belirir; 2. odanın B köşesinin adı 2B'dir. Küçük bir pencere "
           "harf yüksekliğini sorar: köşelerin sık olduğu yerlerde küçültün. Planı yazdırın: bu sizin krokinizdir.",
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
    "p4b": "Penceredeki listeden <b>Kapı veya pencere ekle</b> seçin ve iki oda arasındaki duvarın üzerinde, kapının "
           "yakınında <b>çizime tıklayın</b>. Eklenti iki kenarı kendiliğinden bulur (yeşille vurgular) ve şunları sorar:",
    "t4": [["Kutu", "Örnek"],
           ["1B köşesinden kasaya uzaklık (1. odada ölçülen)", "1,00"],
           ["Genişlik", "0,90"],
           ["Duvar kalınlığı, kapıda ölçülen", "0,30"],
           ["2D köşesinden kasaya uzaklık (2. odada ölçülen)", "1,10"]],
    "p4c": "Uzaklık köşeden kasaya kadar alınır; eklenti tıklanan noktaya en yakın köşeyi önerir, diğerini de "
           "seçebilirsiniz. Bu kadarı yeter: iki oda bağlanmış olur (karşılıklı kasalar, paralel duvar, bilinen "
           "kalınlık). Kapıdan geçen bir ölçü daha (örneğin 1B'den 2D'ye) kontrol işi görür.",
    "p4d": "Kapı yoksa (veya kontrol için) listeden <b>Odalar arası diğer ölçüler</b> seçin:",
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
    "calc": [
        "<b>Kenarlar ve köşegenler</b>: tüm odalara tıklayın (krokiyle aynı sırada) ve Enter'a basın. Odaların "
        "durumlarıyla listesi açılır: her seferinde bir oda açıp kenarları ve köşegenleri yazın, kapıları çizime "
        "tıklayarak ekleyin; her şey tamamlandığında liste <b>Hesapla</b>'yı önerir.",
        "Ölçüler yetmez veya uyuşmazsa pencere, nedeni ve önceden yazılmış ölçülerle yeniden açılır.",
        "Sonuç her odanın alanıyla birlikte <b>RILIEVO_CORRETTO</b> katmanına gider; ölçülendirmeler, köşegenler ve "
        "tablo <b>RILIEVO_QUOTE</b> üzerinde; kapılar ve pencereler <b>RILIEVO_APERTURE</b> üzerinde. Göz kararı çizim "
        "karşılaştırma için kalır. <b>Çizime uygula</b> ardından özgün çoklu çizgileri hesaplanan şekle getirir.",
        "Artık tablosunu okuyun: kırmızı bir artık yeniden kontrol edilecek ölçüyü gösterir. Yalnızca bir fazla "
        "ölçüyle hata birkaç ölçüye dağılır: bir tane daha almak onu bulmaya yardım eder.",
        "<b>Ölçü defteri</b> her şeyi Excel veya Calc için bir CSV'ye kaydeder (ölçüler, artıklar, koordinatlar, alanlar).",
    ],
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
    "end": "Room Survey bir hobi projesidir, profesyonel kullanıma uygun değildir: yazar kullanımından doğacak hiçbir "
           "sorumluluğu kabul etmez. Kod ve güncellemeler: github.com/Macchecaspita/opencad-rilievo-plugin",
}
