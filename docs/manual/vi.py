"""Manual texts — Tiếng Việt (dịch máy)."""

OUT = "manual_vi.pdf"
DEC = ","
S = {
    "doc_title": "Room Survey — hướng dẫn đo đạc",
    "foot": "Room Survey — hướng dẫn đo đạc · dự án sở thích, không dùng cho mục đích chuyên nghiệp",
    "subtitle": "Cách đo một căn hộ bằng phép đo trực tiếp<br/>và dựng lại trong Open CAD Studio",
    "warn": "<b>Miễn trừ trách nhiệm.</b> Room Survey là dự án sở thích, làm cho vui và <b>không phù hợp cho mục đích "
            "chuyên nghiệp</b>. Phần mềm được cung cấp nguyên trạng, không có bất kỳ bảo đảm nào; tác giả không chịu trách "
            "nhiệm về việc sử dụng hay kết quả. Luôn kiểm tra kết quả bằng các số đo bổ sung.",

    "d_room": "Phòng {r}",
    "d_sketch": "bản phác: phòng được đánh số, góc có chữ cái (do nút 3. Phòng ghi)",
    "d_check": "B–D kiểm tra",
    "d_out1": "C–E: đi ra ngoài,",
    "d_out2": "không đo được",
    "d_from1b": "khoảng cách từ 1B",
    "d_width": "0,90 rộng",
    "d_thick": "dày 0,30 (ở má cửa)",
    "d_from2d": "1,10 khoảng cách từ 2D",
    "d_furn": "đồ đạc",
    "d_blocked": "A–C bị chắn",

    "h1": "1. Tóm tắt phương pháp",
    "p1": "Bạn vẽ mặt bằng <b>ước lượng bằng mắt</b>, tại hiện trường đo các cạnh và vài đường chéo của mỗi phòng, cùng vài "
          "số đo nối các phòng với nhau, và plugin dựng lại mặt bằng <b>thực</b> bằng cách bình sai các số đo theo phương "
          "pháp bình phương nhỏ nhất. Số dư cho thấy ngay nếu một số đo bị sai.",
    "t1": [["", "Ở đâu", "Bạn làm gì"],
           ["1", "tại bàn", "bản phác: vẽ mỗi phòng thành đa tuyến kín (bằng mắt) và ghi các chữ cái"],
           ["2", "tại hiện trường", "đo cạnh, đường chéo, độ dày tường, cửa đi và cửa sổ, ghi lên bản phác"],
           ["3", "tại bàn", "nhập số đo, tính, đọc số dư, hoàn thiện bản vẽ"]],
    "h_tools": "Dụng cụ",
    "tools": [
        "máy đo khoảng cách laser (tốt hơn nếu đo từ mặt sau máy) và thước dây 5 m cho số đo ngắn (độ dày, má cửa, hốc);",
        "bản phác đã in, bút chì và tẩy; bảng kẹp giấy;",
        "thước gấp hoặc ê-ke để kiểm tra góc khi đường chéo bị chắn;",
        "máy ảnh hoặc điện thoại: mỗi phòng một ảnh, với chữ cái các góc ghi trên bản phác.",
    ],

    "h2": "2. Trước khi đến: bản phác trong CAD",
    "p2a": "Vẽ <b>mỗi phòng</b> thành <b>đa tuyến kín</b> (PLINE … Close) theo <b>mặt trong</b> của tường. Kích thước không "
           "quan trọng: quan trọng là mỗi góc thực là một đỉnh của đa tuyến, đúng thứ tự. Chỗ thụt, hốc tường, cột lồi và "
           "vát góc cũng là đỉnh (phòng 3 trong ví dụ). Hành lang là một phòng như các phòng khác.",
    "p2b": "Nhấn <b>3. Phòng: số, chữ và số đo</b> và chọn các phòng — từng phòng theo thứ tự bạn muốn cho số, hoặc tất cả cùng lúc bằng khung chọn (đánh số từ trên xuống, trái sang phải) — rồi nhấn Enter: mỗi phòng hiện số của nó in đậm và các góc hiện chữ A, B, C …; góc B của phòng 2 gọi là 2B. Cửa sổ số đo cũng mở: tạm thời đóng nó bằng <b>Đóng</b> và chọn <b>giữ</b> các chữ cái. In mặt bằng: đó là bản phác của bạn.",
    "t2": [["Mẹo", "Lý do"],
           ["Đánh số phòng theo thứ tự bạn cũng sẽ đi tại hiện trường", "Ghi nhanh hơn và ít nhầm phòng hơn."],
           ["Đa tuyến ở mặt trong", "Số đo lấy bên trong phòng; tường ngoài có sau từ độ dày."],
           ["Không có đỉnh thừa", "Một đỉnh trên cạnh thẳng (góc 180°) đòi số đo thừa."]],

    "h3": "3. Tại hiện trường: đo một phòng",
    "p3a": "Phòng có <b>n</b> góc có 2n − 3 ẩn số: nó được xác định bởi <b>tất cả các cạnh</b> cộng <b>n − 3 đường chéo</b>. "
           "Hình chữ nhật cần 1 đường chéo, phòng hình chữ L cần 3, phòng tắm có chỗ thụt trong ví dụ cần 3.",
    "p3b": "Đo các cạnh <b>ở cùng độ cao</b> (ví dụ 1 m trên sàn, cao hơn len chân tường và đồ đạc thấp), từ góc đến góc. "
           "Sau đó đo đường chéo. Các đường chéo plugin gợi ý đều nằm <b>bên trong</b> phòng: trong phòng hình chữ L "
           "không bao giờ có đường chéo cắt qua tường.",
    "cap3": "Trái: các cạnh (xanh dương) và đường chéo (cam) của một phòng không vuông; đường chéo thứ hai (xanh lá) là số "
            "đo kiểm tra. Phải: phòng hình chữ L, ba đường chéo hữu ích và một đường nên tránh.",
    "h_check": "Số đo kiểm tra",
    "p_check": "<b>Luôn đo thêm một đường chéo</b> so với mức tối thiểu. Không có số đo thừa thì hình dạng chính xác với bất cứ "
               "gì bạn nhập, và một con số đọc sai (6,85 thay vì 6,58) sẽ không bị phát hiện. Với một số đo thừa, plugin "
               "tính <b>số dư</b> và σ₀ và tô đỏ các số đo vượt dung sai (thường 1–2 cm).",
    "h_obst": "Khi đồ đạc chắn một đường chéo",
    "obst": [
        "Trước tiên thử đường chéo khác (B–D thay vì A–C), hoặc nâng lên trên đồ đạc, giữ máy đo nằm ngang.",
        "Nếu không được, dùng một <b>ràng buộc</b> (trong trang của phòng): góc và giá trị, ví dụ <b>A, 90</b>, sau khi "
        "kiểm tra bằng ê-ke; hoặc một cạnh <b>ngang</b> hay <b>dọc</b>. Mỗi ràng buộc tính là một số đo và cũng có số dư.",
        "Nếu vẫn còn thiếu, tùy chọn <b>hoàn thiện hình bằng các góc của bản vẽ</b> (thường bật) lấy phần thiếu từ bản vẽ, "
        "và bảng đánh dấu đỏ. Ở chỗ đó kết quả chỉ tốt bằng bản vẽ bằng mắt của bạn: hãy ghi chú lại.",
    ],

    "h4": "4. Nối các phòng",
    "p4a": "Mỗi phòng, đo riêng, có hình dạng đúng nhưng không biết mình ở đâu so với các phòng khác. Để nối, bạn cần <b>ít "
           "nhất 3 số đo</b> tới phòng kế bên, hoặc một <b>cửa chung</b>. Cửa là cách tiện nhất.",
    "cap4": "Cửa giữa phòng 1 và phòng 2: khoảng cách từ góc và chiều rộng đo ở mỗi bên, độ dày tường đo ở má cửa.",
    "p4b": "Trong cửa sổ số đo, bên dưới các phòng, nhấn <b>＋ Thêm cửa đi hoặc cửa sổ</b> và <b>nhấp vào bản vẽ</b>, trên tường giữa hai phòng, gần cửa. Plugin tự tìm cả hai cạnh (tô sáng màu xanh lá) và hỏi:",
    "t4": [["Ô", "Ví dụ"],
           ["Khoảng cách từ góc 1B đến má cửa (đo trong phòng 1)", "1,00"],
           ["Chiều rộng", "0,90"],
           ["Độ dày tường, đo ở cửa", "0,30"],
           ["Khoảng cách từ góc 2D đến má cửa (đo trong phòng 2)", "1,10"]],
    "p4c": "Khoảng cách tính từ góc đến má cửa; plugin gợi ý góc gần điểm bạn nhấp nhất và bạn có thể chọn góc kia. Thế là "
           "xong: hai phòng đã được nối (má cửa đối diện nhau, tường song song, độ dày đã biết). Một số đo thêm qua cửa "
           "(ví dụ từ 1B đến 2D) dùng để kiểm tra.",
    "p4d": "Không có cửa (hoặc để kiểm tra), dùng mục <b>Các số đo khác giữa các phòng</b> ở cuối cửa sổ:",
    "links": [
        "<b>độ dày tường</b>: Từ = một cạnh (1B-1C), Đến = góc của phòng kia sau bức tường đó (2A), Đo được = độ dày đo "
        "vuông góc ở cửa;",
        "<b>số đo qua cửa</b>: Từ = 1B, Đến = 2D, giữa các góc của hai phòng nhìn thấy qua cửa; lấy hai số đo tới các góc khác nhau.",
    ],
    "h_order": "Thứ tự gợi ý cho căn hộ ví dụ",
    "p_order": "Bắt đầu từ <b>hành lang</b> (phòng 4), nơi chạm gần như mọi phòng khác, và nối từng phòng với hành lang qua "
               "cửa của nó: phòng khách và phòng ngủ bên trái, bếp và phòng ngủ 2 bên phải; phòng tắm nối với phòng ngủ. "
               "Như vậy mỗi phòng treo trên một chuỗi ngắn và sai số chỉ ở cục bộ.",

    "h_fix": "Đường bao cố định đo bằng máy toàn đạc",
    "p_fix": "Nếu đường bao ngoài của tòa nhà đã được đo bằng máy toàn đạc (hoặc dù sao cũng chính xác), plugin dùng nó làm tham chiếu cố định: nó không bao giờ bị dịch chuyển và các phòng tự đặt vào bên trong.",
    "fix": ["vẽ đường bao thành <b>đa tuyến kín</b> theo tọa độ thực, và các phòng bằng mắt bên trong;", "nhấn <b>1. Đường bao ngoài</b>, nhấp vào đa tuyến và nhấn Enter (Enter không nhấp = không có đường bao);", "nhấn <b>2. Độ dày tường ngoài</b> và nhấp từng cạnh của đường bao: với mỗi cạnh nhập độ dày tường (đo ở cửa sổ) và chọn hướng, <i>về phía tâm</i> cho tường ngoài bình thường (<i>ra phía ngoài</i> chỉ vẽ đường, không đưa vào tính toán). Ngay khi bạn nhấn OK, đường song song của cạnh đó và giá trị hiện trên bản vẽ, ở lớp RILIEVO_PERIMETRO: cạnh không có đường là cạnh bị quên. Enter để kết thúc: phần tóm tắt liệt kê các cạnh chưa có độ dày;", "trong tính toán, các cạnh phòng dọc đường bao giữ song song với nó ở khoảng cách đó: phòng góc có hai cạnh trên đường bao không cần đường chéo để định vị; phòng chỉ có một cạnh trên đường bao vẫn phải nối với các phòng khác (ví dụ bằng cửa)."],
    "h_hatch": "Tường đặc giữa các phòng: mẫu tô",
    "p_hatch": "Không thể đo xuyên qua tường đặc, nhưng độ dày của nó đo được ở các cửa. Nếu trong bản vẽ bạn đặt <b>mẫu tô</b> vào các tường giữa các phòng, plugin tự tìm ra chúng:",
    "hatch": ["nó tìm hai cạnh đối diện của hai phòng khác nhau có mẫu tô ở giữa và đề xuất chúng trong mục <b>Tường đặc giữa các phòng</b> của cửa sổ số đo;", "nhập <b>độ dày đo ở một cửa</b>; nếu cùng bức tường đã có cửa chung với độ dày, ô có thể để trống;", "trong tính toán hai cạnh giữ song song ở khoảng cách đó, với số dư của chúng trong bảng."],
    "p_hatch2": "Tường đặc cố định khoảng cách và góc xoay giữa hai phòng, nhưng không cố định độ trượt dọc tường: bạn vẫn cần một cửa trong cùng bức tường, một bức tường thứ hai không song song tới cùng phòng, hoặc đường bao cố định.",

    "h5": "5. Cửa đi, cửa sổ và tường ngoài",
    "h_dw": "Cửa đi và cửa sổ",
    "p_dw": "Cửa sổ và cửa ra ngoài cũng được thêm bằng <b>Thêm cửa đi hoặc cửa sổ</b> và một cú nhấp vào tường: loại "
            "(<b>cửa đi</b>, <b>cửa sổ</b>, <b>lỗ mở</b>), khoảng cách từ góc đến má cửa và chiều rộng thông thủy. Cửa được "
            "gắn bản lề ở phía góc mà bạn đo từ đó. Ghi các độ cao (bậu cửa, lanh tô) trên bản phác: plugin làm việc trên mặt bằng.",
    "h_ext": "Tường ngoài",
    "p_ext": "Plugin dựng các phòng ở mặt trong. Với đường bao ngoài, đo <b>độ dày tường ở cửa sổ</b> (gồm cả má cửa), ít "
             "nhất một chỗ cho mỗi mặt tiền; sau đó trong CAD vẽ mặt ngoài bằng OFFSET từ các cạnh phòng, ở khoảng cách đã "
             "đo, và nối các góc. Khi có thể, kiểm tra bằng một số đo bên ngoài (chiều dài một mặt tiền) lấy từ ngoài trời.",

    "h6": "6. Tại bàn: tính toán và kiểm tra",
    "calc": ["Nhấn <b>3. Phòng: số, chữ và số đo</b>, chọn tất cả các phòng (cùng thứ tự như bản phác) và nhấn Enter. <b>Một cửa sổ duy nhất</b> mở ra với tất cả các phòng xếp chồng, từ phòng 1 đến phòng cuối, mỗi phòng có tiêu đề và trạng thái: cuộn bằng thanh bên phải và nhập cạnh và đường chéo. Dưới các phòng là cửa đi và cửa sổ, tường đặc, các số đo khác giữa các phòng và tùy chọn.", "Nhấn <b>▶ Tính</b>: đa tuyến của các phòng được đưa về hình đã tính và <b>bản vẽ tự cập nhật</b> (Ctrl+Z để hoàn tác). Nếu số đo không đủ hoặc không khớp, cửa sổ mở lại với lý do và các số đo đã nhập.", "Kích thước, đường chéo và bảng nằm ở <b>RILIEVO_QUOTE</b>; cửa đi và cửa sổ ở <b>RILIEVO_APERTURE</b>. Cửa sổ <b>Kết quả tính toán</b> tóm tắt diện tích, số đo thừa và σ₀.", "Đọc bảng số dư: số dư màu đỏ đánh dấu số đo cần kiểm tra lại. Chỉ với một số đo thừa, sai số lan ra nhiều số đo: đo thêm một số đo giúp tìm ra nó.", "<b>Sổ đo</b> lưu mọi thứ dạng CSV cho Excel hoặc Calc (số đo, số dư, tọa độ, diện tích)."],
    "h_vals": "Giá trị điển hình",
    "t6": [["Đại lượng", "Giá trị điển hình", "Nếu cao hơn"],
           ["số dư của một cạnh hoặc đường chéo", "trong 1 cm", "đo lại; kiểm tra chữ số bị đảo và đúng góc"],
           ["σ₀ của một phòng", "0,3–0,8 cm", "đo ở các độ cao khác nhau, góc không sắc"],
           ["góc ràng buộc 90°", "số dư trong 0,2°", "góc không vuông: bỏ ràng buộc và đo một đường chéo"]],
    "h_list": "Danh sách kiểm tra trước khi rời hiện trường",
    "list": [
        "mỗi phòng có đủ các cạnh và ít nhất n − 3 đường chéo (hoặc ràng buộc đã kiểm tra), cộng một số đo kiểm tra;",
        "mỗi phòng đều được nối: cửa chung có độ dày, hoặc 3 số đo tới một phòng đã được nối;",
        "độ dày tường ngoài đo ở cửa sổ, ít nhất một cho mỗi mặt tiền;",
        "cửa đi và cửa sổ: khoảng cách từ góc, chiều rộng, độ cao đã ghi;",
        "mỗi phòng một ảnh.",
    ],
    "ex_h": "7. Một ví dụ đầy đủ, từng bước",
    "ex_p": "Một căn hộ bảy phòng trong tòa nhà có mặt tiền trái xiên và một cánh có góc vát. Đường bao ngoài đã được đo bằng máy toàn đạc; tường bên trong là tường đặc và độ dày chỉ đo được ở các cửa.",
    "ex_fig": "đường bao E1–E7 đã đo, phòng 1–7 vẽ bằng mắt, tường đặc được tô mẫu",
    "ex_cap": "Bản vẽ ban đầu: đường bao cố định E1–E7, phòng được đánh số với chữ cái các góc, tường được tô mẫu, cửa đi P1–P6 và cửa sổ (màu xanh).",
    "ex_ties": [["Phòng", "Cạnh trên đường bao", "Tường đặc (mẫu tô)", "Cửa"], ["1", "C–D, D–A", "A–B với 4; B–C với 2", "P5"], ["2", "C–D", "D–A với 1; A–B với 5; B–C với 3", "P2"], ["3", "B–C, C–D, D–E", "A–B với 6; E–A với 2", "P6"], ["4", "A–B, D–A", "C–D với 1; B–C với 5", "P1, P5"], ["5", "A–B", "D–A với 4; C–D với 2; B–C với 6 và 7", "P1, P2, P3, P4"], ["6", "B–C, C–D", "D–A với 5; A–B với 7; C–D với 3", "P3, P6"], ["7", "A–B, B–C", "C–D với 6; D–A với 5", "P4"]],
    "ex_ties_note": "Sáu trong bảy phòng chạm đường bao bằng hai cạnh không song song: chỉ riêng đường bao đã đặt chúng vào chỗ. Phòng 2 chỉ có một cạnh trên đường bao, nhưng bức tường đặc về phía phòng 1 (không song song) khóa nó lại. Vì vậy tường đặc và cửa cũng đóng vai trò kiểm tra.",
    "ex_steps": [["Đường bao từ máy toàn đạc", "Đo các góc ngoài của tòa nhà E1…E7 bằng máy toàn đạc và nối chúng thành đa tuyến kín theo tọa độ thực. Đó là yếu tố chính xác duy nhất của bản vẽ: plugin sẽ không bao giờ dịch chuyển nó."], ["Các phòng vẽ bằng mắt", "Bên trong đường bao, vẽ mỗi phòng thành đa tuyến kín theo mặt trong của tường. Chỉ số lượng và thứ tự góc là quan trọng: phòng 3 có năm góc vì tường ngoài bị vát, phòng 1 và 4 theo mặt tiền xiên."], ["Mẫu tô trong tường đặc", "Dùng lệnh tô mẫu (HATCH) tô khoảng giữa đường bao và các phòng, tức là toàn bộ tường. Mẫu tô cho plugin biết phòng nào bị ngăn bởi tường đặc: không cần chính xác, chỉ cần nằm giữa hai cạnh."], ["Các chữ cái", "Nhấn <b>3. Phòng: số, chữ và số đo</b>, nhấp các phòng theo thứ tự 1…7 của hình và nhấn Enter. Số và chữ hiện ngay và cửa sổ số đo mở ra: đóng nó và giữ các chữ cái. In bản vẽ: đó là bản phác mang ra hiện trường."], ["Tại hiện trường: các phòng", "Trong mỗi phòng, đo các cạnh ở cùng độ cao và ít nhất một đường chéo. Phòng có hai cạnh trên đường bao không cần đường chéo cho hình dạng, nhưng một đường chéo thêm là cách duy nhất để phát hiện một con số đọc sai. Ví dụ với phòng 1:"], ["Tại hiện trường: độ dày và cửa", "Đo độ dày tường ngoài ở cửa sổ (ở đây 0,40) và tường trong ở cửa (ở đây 0,30). Với mỗi cửa, ghi ở cả hai bên khoảng cách từ góc đến má cửa và chiều rộng:"], ["Nút 1 và 2: đường bao và độ dày", "Tại bàn nhấn <b>1. Đường bao ngoài</b>, nhấp vào đa tuyến E1…E7 và nhấn Enter. Sau đó nhấn <b>2. Độ dày tường ngoài</b>, nhấp cạnh đầu tiên, nhập 0,40, để hướng <i>về phía tâm</i> và đánh dấu ô để áp dụng cho mọi cạnh: sau OK các đường song song của tất cả các cạnh hiện ra cùng lúc. Enter để kết thúc."], ["Nút 3: các phòng và cửa sổ số đo", "Nhấn <b>3. Phòng: số, chữ và số đo</b>, nhấp các phòng theo cùng thứ tự 1…7 và nhấn Enter. Cửa sổ mở với phòng 1 đến 7 xếp chồng; phía dưới là mục <b>Tường đặc giữa các phòng (theo mẫu tô)</b> với các dòng đã đề xuất: nhập 0,30 ở chỗ không có cửa chung; chỗ có cửa, độ dày lấy từ cửa."], ["Cạnh, đường chéo và cửa", "Cuộn cửa sổ và nhập cạnh và đường chéo của mỗi phòng. Sau đó nhấn <b>＋ Thêm cửa đi hoặc cửa sổ</b> và nhấp vào tường của mỗi cửa P1…P6: plugin tìm cả hai cạnh, hỏi khoảng cách, chiều rộng và độ dày, rồi quay lại cửa sổ. Cửa sổ được thêm theo cách tương tự, trên tường ngoài."], ["Tính", "Khi mỗi phòng hiện ✔ và các phòng đã được nối, nhấn <b>▶ Tính</b>: đa tuyến của các phòng nhận hình đã tính và bản vẽ tự cập nhật. Cửa sổ <b>Kết quả tính toán</b> tóm tắt số đo thừa và σ₀; trong bảng các số dư màu đỏ đánh dấu số đo cần kiểm tra lại. <b>Sổ đo</b> lưu mọi thứ dạng CSV."]],
    "ex_room_step": 5,
    "ex_door_step": 6,
    "ex_room1": [["Số đo (phòng 1)", "Giá trị", "Ghi chú"], ["1A–1B", "4,10", "cạnh"], ["1B–1C", "8,30", "cạnh"], ["1C–1D", "4,58", "cạnh trên đường bao"], ["1D–1A", "8,31", "cạnh trên đường bao (xiên)"], ["1A–1C", "9,26", "đường chéo kiểm tra"], ["1B–1D", "9,48", "đường chéo thứ hai (tùy chọn)"]],
    "ex_doors": [["Cửa", "Phòng thứ nhất: cạnh, khoảng cách", "Phòng thứ hai: cạnh, khoảng cách", "Rộng", "Dày"], ["P1", "4B–4C, từ 4B 1,60", "5D–5A, từ 5A 1,60", "0,90", "0,30"], ["P2", "5C–5D, từ 5D 1,30", "2A–2B, từ 2A 1,30", "0,90", "0,30"], ["P3", "5B–5C, từ 5B 5,00", "6D–6A, từ 6A 1,10", "0,90", "0,30"], ["P4", "5B–5C, từ 5B 1,00", "7D–7A, từ 7A 1,00", "0,90", "0,30"], ["P5", "4C–4D, từ 4C 1,10", "1A–1B, từ 1B 1,10", "0,90", "0,30"], ["P6", "6C–6D, từ 6D 1,30", "3A–3B, từ 3A 1,30", "0,80", "0,30"]],
    "ex_end": "Khi đường bao đã được đo một lần, không số đo nào phải xuyên qua tường: cạnh và đường chéo đo bên trong phòng, độ dày đo ở cửa đi và cửa sổ.",
    "end": "Room Survey là dự án sở thích, không phù hợp cho mục đích chuyên nghiệp: tác giả không chịu trách nhiệm về "
           "việc sử dụng. Mã nguồn và cập nhật: github.com/Macchecaspita/opencad-rilievo-plugin",
    "h_holes": 'Mở cửa đi và cửa sổ trên bản vẽ: nút «Cửa đi và cửa sổ»',
    "p_holes": 'Sau khi tính xong, nút <b>Cửa đi và cửa sổ</b> mở các lỗ trên tường: cắt các đường tường, vẽ má cửa, làm trống phần tô vật liệu và vẽ khung cửa cùng kích thước của nó.',
    "cap_holes": 'Một cửa sổ rộng 120 và cao 140 cm, cách góc 1A 1,50: khung ở mặt trong của tường; bên ngoài tòa nhà là dấu ngoặc xám và kích thước rộng trên cao, tính bằng xăng-ti-mét.',
    "holes": ['nhấn <b>Cửa đi và cửa sổ</b> và chọn <b>cửa đi</b> hoặc <b>cửa sổ</b>, với <b>chiều rộng</b> và <b>chiều cao</b> (các giá trị được đề xuất lại cho lỗ tiếp theo);', 'nhấp vào <b>góc</b> mà bạn đã đo từ đó: một góc của đường bao ngoài hoặc của một phòng;', 'nhấp vào <b>cạnh</b> có lỗ cửa, rồi nhập <b>khoảng cách</b> từ góc đến má cửa;', 'plugin cắt cạnh đã nhấp và đường ở phía bên kia tường (kể cả khi không song song), vẽ má cửa và làm trống phần tô trong lỗ;', 'trên lớp <b>RILIEVO_APERTURE</b> xuất hiện khung cửa và kích thước rộng/cao bằng xăng-ti-mét, bên ngoài đường bao của tòa nhà.'],
    "p_holes2": 'Cửa sổ có khung ở mặt trong của tường; cửa đi để lỗ trống. Lệnh chỉ làm việc trên bản vẽ: một phòng đã bị cắt không còn là đa tuyến kín và không tham gia tính toán nữa, vì vậy hãy mở lỗ <b>sau</b> khi tính. Ctrl+Z hoàn tác lỗ cuối cùng.',
}
