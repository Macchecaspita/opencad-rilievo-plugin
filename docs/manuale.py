"""Genera docs/Manuale_rilievo.pdf — come rilevare un appartamento con
Rilievo Vani. Esegui:  python docs/manuale.py   (serve reportlab).

Il PDF viene anche incorporato nel plugin (comando RV_MANUALE)."""

import math
import os

from reportlab.graphics.shapes import Circle, Drawing, Line, PolyLine, Polygon, String, Wedge
from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib.units import mm
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (KeepTogether, PageBreak, Paragraph, SimpleDocTemplate, Spacer, Table,
                                TableStyle)

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "Manuale_rilievo.pdf")

FONT_DIR = "/usr/share/fonts/truetype/dejavu"
for name, file in [("DV", "DejaVuSans.ttf"), ("DV-B", "DejaVuSans-Bold.ttf"), ("DV-I", "DejaVuSans-Oblique.ttf")]:
    pdfmetrics.registerFont(TTFont(name, os.path.join(FONT_DIR, file)))
pdfmetrics.registerFontFamily("DV", normal="DV", bold="DV-B", italic="DV-I", boldItalic="DV-B")

INK = colors.HexColor("#222222")
GREY = colors.HexColor("#8a8a8a")
LIGHT = colors.HexColor("#f2f2f2")
ACCENT = colors.HexColor("#d4471f")      # arancio tendente al rosso
SIDE = colors.HexColor("#1f6fb4")        # lati
DIAG = colors.HexColor("#e08a00")        # diagonali
CHECK = colors.HexColor("#2e9a3e")       # misura di controllo
WALL = colors.HexColor("#555555")

base = ParagraphStyle("base", fontName="DV", fontSize=9.6, leading=13.6, textColor=INK, spaceAfter=5)
small = ParagraphStyle("small", parent=base, fontSize=8.2, leading=11, textColor=GREY)
h1 = ParagraphStyle("h1", parent=base, fontName="DV-B", fontSize=15, leading=19, spaceBefore=6, spaceAfter=8, textColor=ACCENT)
h2 = ParagraphStyle("h2", parent=base, fontName="DV-B", fontSize=11.5, leading=15, spaceBefore=8, spaceAfter=4)
cap = ParagraphStyle("cap", parent=small, alignment=TA_CENTER, spaceBefore=2, spaceAfter=10)
bullet = ParagraphStyle("bullet", parent=base, leftIndent=12, bulletIndent=2, spaceAfter=3)
title = ParagraphStyle("title", parent=base, fontName="DV-B", fontSize=24, leading=30, textColor=INK, spaceAfter=6)
subtitle = ParagraphStyle("subtitle", parent=base, fontSize=13, leading=18, textColor=ACCENT, spaceAfter=18)
note = ParagraphStyle("note", parent=base, backColor=LIGHT, borderPadding=(6, 8, 6, 8), spaceBefore=6, spaceAfter=10)


def P(text, style=base):
    return Paragraph(text, style)


def bullets(items):
    return [Paragraph(t, bullet, bulletText="•") for t in items]


# ── Disegni ────────────────────────────────────────────────────────────────

class Plan:
    """Un disegno in coordinate metriche, scalato in una Drawing."""

    def __init__(self, width_mm, bbox, pad=6):
        (x0, y0, x1, y1) = bbox
        self.s = (width_mm * mm - 2 * pad) / (x1 - x0)
        self.h = (y1 - y0) * self.s + 2 * pad
        self.x0, self.y0, self.pad = x0, y0, pad
        self.d = Drawing(width_mm * mm, self.h)

    def p(self, x, y):
        return (self.pad + (x - self.x0) * self.s, self.pad + (y - self.y0) * self.s)

    def poly(self, pts, color=INK, width=1.0, fill=None, dash=None, closed=True):
        flat = [c for q in pts for c in self.p(*q)]
        if closed:
            shape = Polygon(flat, strokeColor=color, strokeWidth=width, fillColor=fill)
        else:
            shape = PolyLine(flat, strokeColor=color, strokeWidth=width)
        if dash:
            shape.strokeDashArray = dash
        self.d.add(shape)

    def line(self, a, b, color=INK, width=1.0, dash=None):
        ln = Line(*self.p(*a), *self.p(*b), strokeColor=color, strokeWidth=width)
        if dash:
            ln.strokeDashArray = dash
        self.d.add(ln)

    def text(self, xy, s, size=7, color=INK, anchor="middle", bold=False):
        x, y = self.p(*xy)
        self.d.add(String(x, y - size * 0.35, s, fontName="DV-B" if bold else "DV", fontSize=size,
                          fillColor=color, textAnchor=anchor))

    def dot(self, xy, r=1.6, color=INK):
        x, y = self.p(*xy)
        self.d.add(Circle(x, y, r, fillColor=color, strokeColor=color))

    def door(self, hinge, other, inward, color=INK):
        """Porta: anta da `hinge` verso l'interno, arco fino a `other`."""
        w = math.dist(hinge, other)
        tip = (hinge[0] + inward[0] * w, hinge[1] + inward[1] * w)
        self.line(hinge, tip, color, 0.8)
        a0 = math.degrees(math.atan2(inward[1], inward[0]))
        a1 = math.degrees(math.atan2(other[1] - hinge[1], other[0] - hinge[0]))
        if (a1 - a0) % 360 > 180:
            a0, a1 = a1, a0
        cx, cy = self.p(*hinge)
        wd = Wedge(cx, cy, w * self.s, a0, a1, strokeColor=color, strokeWidth=0.5, fillColor=None)
        wd.strokeDashArray = [1.5, 1.5]
        self.d.add(wd)


def inside_offset(pts, i, k):
    """Punto spostato verso l'interno dello spigolo i di k metri."""
    n = len(pts)
    a, b, c = pts[i - 1], pts[i], pts[(i + 1) % n]
    area = sum(pts[j][0] * pts[(j + 1) % n][1] - pts[(j + 1) % n][0] * pts[j][1] for j in range(n))
    ccw = area > 0

    def nrm(p, q):
        dx, dy = q[0] - p[0], q[1] - p[1]
        l = math.hypot(dx, dy)
        return (-dy / l, dx / l) if ccw else (dy / l, -dx / l)

    n1, n2 = nrm(a, b), nrm(b, c)
    d = (n1[0] + n2[0], n1[1] + n2[1])
    l = math.hypot(*d) or 1
    return (b[0] + d[0] / l * k, b[1] + d[1] / l * k)


# L'appartamento d'esempio, simile a quello della domanda: muro esterno
# fuori squadra a sinistra, un corridoio centrale, sei vani.
OUTER = [(1.0, 0.0), (13.2, 0.0), (13.0, 12.0), (-0.6, 11.9)]
ROOMS = {
    1: [(0.75, 6.5), (5.4, 6.5), (5.4, 11.45), (-0.05, 11.45)],          # soggiorno
    2: [(1.3, 2.8), (5.4, 2.8), (5.4, 6.1), (0.8, 6.1)],                 # camera
    3: [(1.55, 0.45), (4.9, 0.45), (4.9, 2.4), (3.1, 2.4), (3.1, 2.05), (1.75, 2.05)],  # bagno con risega
    4: [(5.8, 0.45), (7.0, 0.45), (7.0, 11.45), (5.8, 11.45)],          # corridoio
    5: [(7.4, 6.4), (12.6, 6.4), (12.55, 11.5), (7.4, 11.5)],           # cucina
    6: [(7.4, 0.45), (12.75, 0.45), (12.65, 6.0), (7.4, 6.0)],          # camera 2
}


def plan_overview():
    pl = Plan(160, (-1.2, -0.8, 13.8, 12.6))
    pl.poly(OUTER, WALL, 1.6, fill=colors.HexColor("#e6e6e6"))
    for pts in ROOMS.values():
        pl.poly(pts, INK, 0.9, fill=colors.white)
    for r, pts in ROOMS.items():
        cx = sum(p[0] for p in pts) / len(pts)
        cy = sum(p[1] for p in pts) / len(pts)
        pl.text((cx, cy), f"Vano {r}", 8.5, ACCENT, bold=True)
        for i in range(len(pts)):
            q = inside_offset(pts, i, 0.42)
            pl.text(q, f"{r}{chr(65 + i)}", 5.6, INK)
    # porte (schematiche)
    pl.door((5.4, 8.0), (5.4, 8.9), (-1, 0))
    pl.door((5.4, 3.6), (5.4, 4.5), (-1, 0))
    pl.door((4.9, 1.0), (4.9, 1.8), (-1, 0))
    pl.door((7.4, 8.2), (7.4, 9.1), (1, 0))
    pl.door((7.4, 2.4), (7.4, 3.3), (1, 0))
    # finestre (doppia linea nel muro esterno)
    for a, b in [((1.6, 11.45), (3.4, 11.45)), ((8.6, 11.5), (10.6, 11.5)), ((13.0, 2.0), (13.0, 4.0))]:
        pl.line(a, b, SIDE, 2.2)
    pl.text((6.4, -0.45), "eidotipo: vani numerati, spigoli con lettere (scritte da Lettere vertici)", 6.5, GREY)
    return pl.d


def plan_room(check=True):
    pts = [(0.0, 0.0), (5.0, 0.0), (5.15, 4.0), (-0.1, 4.05)]
    pl = Plan(78, (-1.0, -0.9, 6.2, 4.9))
    pl.poly(pts, INK, 1.2)
    names = "ABCD"
    for i, q in enumerate(pts):
        pl.dot(q)
        o = inside_offset(pts, i, -0.32)
        pl.text(o, names[i], 8, INK, bold=True)
    labels = ["5,00", "4,00", "5,25", "4,05"]
    for i in range(4):
        a, b = pts[i], pts[(i + 1) % 4]
        m = ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)
        o = inside_offset([a, m, b], 1, 0.25)
        pl.text(o, labels[i], 6.5, SIDE)
        pl.line(a, b, SIDE, 2.0)
    pl.line(pts[0], pts[2], DIAG, 1.0, [3, 2])
    pl.text((2.2, 2.25), "A–C", 6.5, DIAG)
    if check:
        pl.line(pts[1], pts[3], CHECK, 1.0, [1, 2])
        pl.text((3.0, 1.35), "B–D controllo", 6.5, CHECK)
    return pl.d


def plan_l():
    pts = [(0, 0), (6, 0), (6, 2.5), (3, 2.5), (3, 5), (0, 5)]
    pl = Plan(78, (-0.9, -0.9, 6.9, 5.8))
    pl.poly(pts, INK, 1.2)
    for i, q in enumerate(pts):
        pl.dot(q)
        pl.text(inside_offset(pts, i, -0.35), "ABCDEF"[i], 8, INK, bold=True)
    for a, b in [(0, 2), (0, 3), (3, 5)]:
        pl.line(pts[a], pts[b], DIAG, 1.0, [3, 2])
    pl.line(pts[2], pts[4], ACCENT, 0.9, [1, 1.5])
    pl.text((4.9, 3.9), "C–E: passa fuori,", 6, ACCENT)
    pl.text((4.9, 3.5), "non misurabile", 6, ACCENT)
    pl.text((1.6, 1.1), "A–C, A–D, D–F", 6.5, DIAG)
    return pl.d


def plan_door():
    # muro verticale tra vano 1 (sinistra) e vano 2 (destra)
    t = 0.30
    pl = Plan(160, (-2.2, -0.4, 4.6, 4.2))
    pl.poly([(0, 0), (t, 0), (t, 4.0), (0, 4.0)], WALL, 0.8, fill=colors.HexColor("#d9d9d9"))
    pl.poly([(0, 1.0), (t, 1.0), (t, 1.9), (0, 1.9)], colors.white, 0.1, fill=colors.white)
    pl.line((0, 0), (0, 1.0), INK, 1.4)
    pl.line((0, 1.9), (0, 4.0), INK, 1.4)
    pl.line((t, 0), (t, 1.0), INK, 1.4)
    pl.line((t, 1.9), (t, 4.0), INK, 1.4)
    pl.line((0, 1.0), (t, 1.0), INK, 0.8)
    pl.line((0, 1.9), (t, 1.9), INK, 0.8)
    pl.door((0, 1.0), (0, 1.9), (-1, 0))
    pl.dot((0, 0)); pl.text((-0.25, -0.2), "1B", 7, INK, bold=True)
    pl.dot((0, 4.0)); pl.text((-0.25, 4.1), "1C", 7, INK, bold=True)
    pl.dot((t, 4.0)); pl.text((t + 0.25, 4.1), "2D", 7, INK, bold=True)
    pl.dot((t, 0)); pl.text((t + 0.25, -0.2), "2A", 7, INK, bold=True)
    # quote
    pl.line((-0.6, 0), (-0.6, 1.0), SIDE, 1.0)
    pl.text((-0.75, 0.5), "1,00", 6.5, SIDE, anchor="end")
    pl.text((-0.75, 0.2), "distanza da 1B", 5.5, GREY, anchor="end")
    pl.line((-1.4, 1.0), (-1.4, 1.9), DIAG, 1.4)
    pl.text((-1.55, 1.45), "0,90 larghezza", 6.5, DIAG, anchor="end")
    pl.line((0, 1.45), (t, 1.45), CHECK, 1.2)
    pl.text((t + 0.15, 1.45), "spessore 0,30 (nello stipite)", 6.5, CHECK, anchor="start")
    pl.line((t + 0.6, 4.0), (t + 0.6, 1.9), SIDE, 1.0)
    pl.text((t + 0.75, 3.0), "1,10 distanza da 2D", 6.5, SIDE, anchor="start")
    pl.text((-1.0, 3.4), "Vano 1", 9, ACCENT, bold=True)
    pl.text((2.6, 3.4), "Vano 2", 9, ACCENT, bold=True)
    return pl.d


def plan_obstacle():
    pts = [(0, 0), (5, 0), (5, 4), (0, 4)]
    pl = Plan(78, (-0.9, -0.9, 6.0, 4.9))
    pl.poly(pts, INK, 1.2)
    for i, q in enumerate(pts):
        pl.dot(q)
        pl.text(inside_offset(pts, i, -0.32), "ABCD"[i], 8, INK, bold=True)
    pl.poly([(1.8, 1.4), (3.4, 1.4), (3.4, 2.6), (1.8, 2.6)], GREY, 0.6, fill=colors.HexColor("#e8e8e8"))
    pl.text((2.6, 2.0), "mobile", 6.5, GREY)
    pl.line(pts[0], pts[2], ACCENT, 0.8, [1, 1.5])
    x, y = pl.p(0, 0)
    pl.d.add(Line(x + 10, y, x + 10, y + 10, strokeColor=CHECK, strokeWidth=1))
    pl.d.add(Line(x, y + 10, x + 10, y + 10, strokeColor=CHECK, strokeWidth=1))
    pl.text((0.85, 0.75), "A: 90°", 7, CHECK, bold=True)
    pl.text((3.9, 3.1), "A–C bloccata", 6.5, ACCENT)
    return pl.d


# ── Testo ──────────────────────────────────────────────────────────────────

def table(rows, widths, head=True):
    t = Table(rows, colWidths=[w * mm for w in widths])
    st = [
        ("FONT", (0, 0), (-1, -1), "DV", 8.2),
        ("VALIGN", (0, 0), (-1, -1), "TOP"),
        ("GRID", (0, 0), (-1, -1), 0.3, colors.HexColor("#c8c8c8")),
        ("TOPPADDING", (0, 0), (-1, -1), 3),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 3),
    ]
    if head:
        st += [("FONT", (0, 0), (-1, 0), "DV-B", 8.2), ("BACKGROUND", (0, 0), (-1, 0), LIGHT)]
    t.setStyle(TableStyle(st))
    return t


def cell(s):
    return Paragraph(s, ParagraphStyle("c", parent=base, fontSize=8.2, leading=10.5, spaceAfter=0))


def footer(canvas, doc):
    canvas.saveState()
    canvas.setFont("DV", 7.5)
    canvas.setFillColor(GREY)
    canvas.drawString(18 * mm, 10 * mm, "Rilievo Vani — manuale di rilievo · progetto hobbistico, non per uso professionale")
    canvas.drawRightString(A4[0] - 18 * mm, 10 * mm, str(doc.page))
    canvas.restoreState()


def build():
    s = []
    s += [Spacer(1, 30 * mm), P("Rilievo Vani", title),
          P("Come rilevare un appartamento a misure dirette<br/>e ricostruirlo in Open CAD Studio", subtitle),
          plan_overview(),
          Spacer(1, 6 * mm),
          P("<b>Avvertenza.</b> Rilievo Vani è un progetto hobbistico, realizzato per divertimento e <b>non adatto "
            "a un uso professionale</b>. È fornito così com'è, senza alcuna garanzia; l'autore non si assume alcuna "
            "responsabilità per il suo uso né per i risultati. Controlla sempre i risultati con misure di verifica.", note),
          PageBreak()]

    s += [P("1. Il metodo in breve", h1),
          P("Si disegna la pianta <b>a occhio</b>, si misurano in cantiere i lati e alcune diagonali di ogni vano, "
            "più qualche misura che lega i vani tra loro, e il plugin ricostruisce la pianta <b>vera</b> compensando "
            "le misure ai minimi quadrati. Gli scarti dicono subito se una misura è sbagliata."),
          table([["", "Dove", "Cosa si fa"],
                 ["1", "in studio", "eidotipo: si disegna ogni vano come polilinea chiusa (a occhio) e si scrivono le lettere"],
                 ["2", "in cantiere", "si misurano lati, diagonali, spessori dei muri, porte e finestre, segnandoli sull'eidotipo"],
                 ["3", "in studio", "si inseriscono le misure, si calcola, si leggono gli scarti, si completa il disegno"]],
                [10, 26, 128]),
          Spacer(1, 4 * mm),
          P("Attrezzatura", h2)]
    s += bullets([
        "distanziometro laser (meglio con misura dal retro dello strumento) e una cordella metrica da 5 m per le "
        "misure corte (spessori, mazzette, riseghe);",
        "l'eidotipo stampato, matita e gomma; una tavoletta rigida;",
        "un metro rigido o una squadra per verificare gli angoli quando le diagonali sono impedite;",
        "macchina fotografica o telefono: una foto per vano, con le lettere degli spigoli scritte a matita sull'eidotipo.",
    ])

    s += [P("2. Prima del sopralluogo: l'eidotipo nel CAD", h1),
          P("Disegna <b>ogni vano</b> come <b>polilinea chiusa</b> (PLINEA … Chiudi), seguendo il <b>filo interno</b> "
            "dei muri. Le misure non contano: conta che ogni spigolo vero sia un vertice della polilinea, nell'ordine "
            "giusto. Riseghe, nicchie, pilastri sporgenti e smussi sono vertici anche loro (vano 3 dell'esempio). "
            "Il corridoio è un vano come gli altri."),
          P("Con <b>Lettere vertici</b> seleziona i vani — uno alla volta nell'ordine che vuoi dare ai numeri, "
            "oppure tutti insieme con una finestra (numerati dall'alto in basso, da sinistra a destra) — e premi Invio: compaiono 1A, 1B … 2A … Stampa la pianta: è il tuo eidotipo. "
            "Se due lettere sono troppo vicine il plugin le sposta e le collega allo spigolo con una linea."),
          table([["Consiglio", "Perché"],
                 [cell("Numera i vani in un ordine che segui anche in cantiere"), cell("Si scrive più in fretta e si sbaglia meno vano.")],
                 [cell("Polilinea sul filo interno"), cell("Le misure si prendono da dentro i vani; i muri esterni si ricavano dopo dagli spessori.")],
                 [cell("Nessun vertice inutile"), cell("Un vertice su un lato dritto (angolo di 180°) chiede misure inutili.")]],
                [62, 102]),
          PageBreak()]

    s += [P("3. In cantiere: misurare un vano", h1),
          P("Un vano di <b>n</b> spigoli ha 2n − 3 incognite: si chiude con <b>tutti i lati</b> più <b>n − 3 "
            "diagonali</b>. Un rettangolo vuole 1 diagonale, un vano a L 3, il bagno con risega dell'esempio 3."),
          P("Misura i lati <b>alla stessa altezza</b> (per esempio a 1 m da terra, sopra battiscopa e mobili bassi), "
            "da spigolo a spigolo. Poi le diagonali. Le diagonali proposte dal plugin passano tutte <b>dentro</b> il vano: "
            "in un vano a L non c'è mai una diagonale che attraversa il muro."),
          Table([[plan_room(), plan_l()]], colWidths=[82 * mm, 82 * mm]),
          P("Sinistra: lati (blu) e diagonale (arancio) di un vano fuori squadra; la seconda diagonale (verde) è la misura "
            "di controllo. Destra: vano a L, le tre diagonali utili e una da evitare.", cap),
          P("La misura di controllo", h2),
          P("Prendi <b>sempre una diagonale in più</b> del minimo. Senza misure in più la forma è esatta qualunque cosa "
            "tu scriva, e una cifra letta male (6,85 al posto di 6,58) passa inosservata. Con una misura in più il "
            "plugin calcola gli <b>scarti</b> e σ₀ e colora in rosso le misure oltre la tolleranza (di solito 1–2 cm)."),
          P("Quando un mobile impedisce una diagonale", h2),
          Table([[plan_obstacle(), [
              P("Prova prima una diagonale diversa (B–D al posto di A–C), o alzala sopra il mobile tenendo il "
                "distanziometro in bolla."),
              P("Se non si può, usa un <b>vincolo</b> (nella scheda del vano): spigolo e angolo, per esempio "
                "<b>A, 90</b>, dopo averlo verificato con la squadra; oppure un lato <b>orizzontale</b> o <b>verticale</b>. "
                "Ogni vincolo vale una misura e riceve anche lui uno scarto."),
              P("Se manca ancora qualcosa, l'opzione <b>completa la forma con gli angoli del disegno</b> (attiva di "
                "solito) prende dal disegno quello che manca, e la tabella lo segnala in rosso. In quel punto il "
                "risultato vale quanto il tuo disegno a occhio: annotalo."),
          ]]], colWidths=[82 * mm, 82 * mm], style=[("VALIGN", (0, 0), (-1, -1), "TOP")]),
          PageBreak()]

    s += [P("4. Collegare i vani tra loro", h1),
          P("Ogni vano, misurato da solo, ha la forma giusta ma non sa dove si trova rispetto agli altri. Per "
            "collegarlo servono <b>almeno 3 misure</b> verso il vano accanto, oppure una <b>porta comune</b>. "
            "Il modo più comodo è la porta."),
          plan_door(),
          P("Una porta tra il vano 1 e il vano 2: distanza dallo spigolo e larghezza misurate da ciascuna parte, "
            "spessore del muro misurato nello stipite.", cap),
          P("Dall'elenco della finestra scegli <b>Aggiungi porta o finestra</b> e <b>clicca sul disegno</b>, sul muro tra i "
            "due vani, vicino alla porta. Il plugin riconosce da solo i due lati (li evidenzia in verde) e chiede:"),
          table([["Casella", "Esempio"],
                 ["Distanza dallo spigolo 1B allo stipite (misurata nel vano 1)", "1,00"],
                 ["Larghezza", "0,90"],
                 ["Spessore del muro, misurato nella porta", "0,30"],
                 ["Distanza dallo spigolo 2D allo stipite (misurata nel vano 2)", "1,10"]],
                [110, 30]),
          Spacer(1, 3 * mm),
          P("La distanza si prende dallo spigolo fino allo stipite; il plugin propone lo spigolo più vicino al punto "
            "cliccato e puoi sceglierne l'altro. Basta questo: i due vani "
            "risultano collegati (stipiti affacciati, muro parallelo, spessore noto). Una misura in più attraverso la "
            "porta (per esempio da 1B a 2D) fa da controllo."),
          P("Senza porta (o per controllo) scegli dall'elenco <b>Altre misure tra vani</b>:"),
          ]
    s += bullets([
        "<b>spessore di un muro</b>: Da = un lato (1B-1C), A = lo spigolo dell'altro vano dietro quel muro (2A), "
        "Misurata = lo spessore preso in squadra nella porta;",
        "<b>misure attraverso la porta</b>: Da = 1B, A = 2D, tra spigoli dei due vani che si vedono dalla porta; "
        "prendine due verso spigoli diversi.",
    ])
    s += [P("Ordine consigliato per l'appartamento d'esempio", h2),
          P("Parti dal <b>corridoio</b> (vano 4), che tocca quasi tutti gli altri, e collega ogni vano al corridoio con "
            "la sua porta: soggiorno e camera a sinistra, cucina e camera 2 a destra; il bagno si collega alla camera. "
            "In questo modo ogni vano è legato da una catena corta e un errore resta circoscritto.")]

    s += [P("5. Porte, finestre e muri esterni", h1),
          P("Porte e finestre", h2),
          P("Anche finestre e porte verso l'esterno si aggiungono con <b>Aggiungi porta o finestra</b> e un clic sul muro: "
            "tipo (<b>porta</b>, <b>finestra</b>, <b>apertura</b>), distanza dallo spigolo fino alla spalla e larghezza "
            "della luce. La porta è incernierata dalla parte dello spigolo da cui misuri. Le altezze (davanzale, "
            "architrave) annotale sull'eidotipo: il plugin lavora in pianta."),
          P("Muri esterni", h2),
          P("Il plugin ricostruisce i vani sul filo interno. Per il perimetro esterno misura lo <b>spessore del muro "
            "nelle finestre</b> (mazzetta compresa) e almeno uno per facciata; poi nel CAD ricava il filo esterno "
            "con OFFSET dai lati dei vani, alla distanza misurata, e raccorda gli spigoli. Dove possibile verifica "
            "con una misura esterna (la lunghezza di una facciata) presa da fuori."),
          P("6. In studio: calcolo e controllo", h1)]
    s += bullets([
        "<b>Misure e diagonali</b>: clicca tutti i vani (stesso ordine dell'eidotipo) e premi Invio. Si apre l'elenco dei "
        "vani con il loro stato: apri un vano alla volta e scrivi lati e diagonali, aggiungi le porte con un clic sul "
        "disegno; quando tutto è completo l'elenco propone <b>Calcola</b>.",
        "Se le misure non bastano o non tornano, la finestra si riapre con il motivo e con le misure già scritte.",
        "Il risultato va sul layer <b>RILIEVO_CORRETTO</b>, con la superficie di ogni vano; quote, diagonali e tabella su "
        "<b>RILIEVO_QUOTE</b>; porte e finestre su <b>RILIEVO_APERTURE</b>. Il disegno a occhio resta per confronto.",
        "Leggi la tabella degli scarti: uno scarto rosso indica la misura da ricontrollare. Con una sola misura in più "
        "l'errore si spalma su più misure: prenderne un'altra aiuta a trovarlo.",
        "<b>Libretto misure</b> salva tutto in un CSV per Excel o Calc (misure, scarti, coordinate, superfici).",
    ])
    s += [P("Valori indicativi", h2),
          table([["Grandezza", "Valore tipico", "Se è più alto"],
                 [cell("scarto di un lato o una diagonale"), cell("entro 1 cm"), cell("rimisura; controlla cifre invertite e spigolo giusto")],
                 [cell("σ₀ di un vano"), cell("0,3–0,8 cm"), cell("misure prese ad altezze diverse, spigoli non netti")],
                 [cell("angolo vincolato a 90°"), cell("scarto entro 0,2°"), cell("l'angolo non è retto: togli il vincolo e misura una diagonale")]],
                [56, 36, 72]),
          Spacer(1, 4 * mm),
          P("Lista di controllo prima di lasciare il cantiere", h2)]
    s += bullets([
        "ogni vano ha tutti i lati e almeno n − 3 diagonali (o vincoli verificati), più una misura di controllo;",
        "ogni vano è collegato: porta comune con spessore, oppure 3 misure verso un vano già collegato;",
        "spessori dei muri esterni presi nelle finestre, almeno uno per facciata;",
        "porte e finestre: distanza dallo spigolo, larghezza, altezze annotate;",
        "una foto per vano.",
    ])
    s += [Spacer(1, 6 * mm),
          P("Rilievo Vani è un progetto hobbistico, non adatto a un uso professionale: l'autore non si assume "
            "responsabilità per il suo uso. Codice e aggiornamenti: github.com/Macchecaspita/opencad-rilievo-plugin", small)]

    doc = SimpleDocTemplate(OUT, pagesize=A4, leftMargin=18 * mm, rightMargin=18 * mm, topMargin=16 * mm,
                            bottomMargin=18 * mm, title="Rilievo Vani — manuale di rilievo", author="Macchecaspita")
    doc.build(s, onFirstPage=footer, onLaterPages=footer)
    print(OUT)


if __name__ == "__main__":
    build()
