"""Genera i manuali di rilievo di Room Survey, uno per lingua:
docs/Manuale_rilievo.pdf (italiano) e docs/manual_<lingua>.pdf (en, es, fr,
tr, pt, pl). I testi sono in docs/manual/<lingua>.py.
Esegui:  python docs/manuale.py [lingua …]   (serve reportlab).

I PDF vengono anche incorporati nel plugin (comando RV_MANUALE)."""

import importlib.util
import math
import os
import sys

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
LANGS = ["it", "en", "es", "fr", "tr", "pt", "pl"]

# Testi della lingua in corso (impostati da build()).
T = {}
DEC = ","


def num(s):
    """Numero d'esempio con il separatore decimale della lingua."""
    return s.replace(",", DEC)


def load(lang):
    spec = importlib.util.spec_from_file_location(f"manual_{lang}", os.path.join(HERE, "manual", f"{lang}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m

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
        pl.text((cx, cy), str(r), 14, ACCENT, bold=True)
        for i in range(len(pts)):
            q = inside_offset(pts, i, 0.42)
            pl.text(q, chr(65 + i), 5.6, INK)
    # porte (schematiche)
    pl.door((5.4, 8.0), (5.4, 8.9), (-1, 0))
    pl.door((5.4, 3.6), (5.4, 4.5), (-1, 0))
    pl.door((4.9, 1.0), (4.9, 1.8), (-1, 0))
    pl.door((7.4, 8.2), (7.4, 9.1), (1, 0))
    pl.door((7.4, 2.4), (7.4, 3.3), (1, 0))
    # finestre (doppia linea nel muro esterno)
    for a, b in [((1.6, 11.45), (3.4, 11.45)), ((8.6, 11.5), (10.6, 11.5)), ((13.0, 2.0), (13.0, 4.0))]:
        pl.line(a, b, SIDE, 2.2)
    pl.text((6.4, -0.45), T["d_sketch"], 6.5, GREY)
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
    labels = [num(x) for x in ["5,00", "4,00", "5,25", "4,05"]]
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
        pl.text((3.0, 1.35), T["d_check"], 6.5, CHECK)
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
    pl.text((4.9, 3.9), T["d_out1"], 6, ACCENT)
    pl.text((4.9, 3.5), T["d_out2"], 6, ACCENT)
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
    pl.text((-0.75, 0.5), num("1,00"), 6.5, SIDE, anchor="end")
    pl.text((-0.75, 0.2), T["d_from1b"], 5.5, GREY, anchor="end")
    pl.line((-1.4, 1.0), (-1.4, 1.9), DIAG, 1.4)
    pl.text((-1.55, 1.45), T["d_width"], 6.5, DIAG, anchor="end")
    pl.line((0, 1.45), (t, 1.45), CHECK, 1.2)
    pl.text((t + 0.15, 1.45), T["d_thick"], 6.5, CHECK, anchor="start")
    pl.line((t + 0.6, 4.0), (t + 0.6, 1.9), SIDE, 1.0)
    pl.text((t + 0.75, 3.0), T["d_from2d"], 6.5, SIDE, anchor="start")
    pl.text((-1.0, 3.4), T["d_room"].format(r=1), 9, ACCENT, bold=True)
    pl.text((2.6, 3.4), T["d_room"].format(r=2), 9, ACCENT, bold=True)
    return pl.d


def plan_obstacle():
    pts = [(0, 0), (5, 0), (5, 4), (0, 4)]
    pl = Plan(78, (-0.9, -0.9, 6.0, 4.9))
    pl.poly(pts, INK, 1.2)
    for i, q in enumerate(pts):
        pl.dot(q)
        pl.text(inside_offset(pts, i, -0.32), "ABCD"[i], 8, INK, bold=True)
    pl.poly([(1.8, 1.4), (3.4, 1.4), (3.4, 2.6), (1.8, 2.6)], GREY, 0.6, fill=colors.HexColor("#e8e8e8"))
    pl.text((2.6, 2.0), T["d_furn"], 6.5, GREY)
    pl.line(pts[0], pts[2], ACCENT, 0.8, [1, 1.5])
    x, y = pl.p(0, 0)
    pl.d.add(Line(x + 10, y, x + 10, y + 10, strokeColor=CHECK, strokeWidth=1))
    pl.d.add(Line(x, y + 10, x + 10, y + 10, strokeColor=CHECK, strokeWidth=1))
    pl.text((0.85, 0.75), "A: 90°", 7, CHECK, bold=True)
    pl.text((3.9, 3.1), T["d_blocked"], 6.5, ACCENT)
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
    canvas.drawString(18 * mm, 10 * mm, T["foot"])
    canvas.drawRightString(A4[0] - 18 * mm, 10 * mm, str(doc.page))
    canvas.restoreState()


def rows(t):
    """Tabella: ogni cella come paragrafo, così il testo va a capo."""
    out = [t[0]]
    for r in t[1:]:
        out.append([cell(c) for c in r])
    return out


def build(lang):
    global T, DEC
    m = load(lang)
    T, DEC = m.S, m.DEC
    out = os.path.join(HERE, m.OUT)
    s = []
    s += [Spacer(1, 30 * mm), P("Room Survey", title),
          P(T["subtitle"], subtitle),
          plan_overview(),
          Spacer(1, 6 * mm),
          P(T["warn"], note),
          PageBreak()]

    s += [P(T["h1"], h1),
          P(T["p1"]),
          table(rows(T["t1"]), [10, 30, 124]),
          Spacer(1, 4 * mm),
          P(T["h_tools"], h2)]
    s += bullets(T["tools"])

    s += [P(T["h2"], h1),
          P(T["p2a"]),
          P(T["p2b"]),
          table(rows(T["t2"]), [62, 102]),
          PageBreak()]

    s += [P(T["h3"], h1),
          P(T["p3a"]),
          P(T["p3b"]),
          Table([[plan_room(), plan_l()]], colWidths=[82 * mm, 82 * mm]),
          P(T["cap3"], cap),
          P(T["h_check"], h2),
          P(T["p_check"]),
          P(T["h_obst"], h2),
          Table([[plan_obstacle(), [P(x) for x in T["obst"]]]], colWidths=[82 * mm, 82 * mm],
                style=[("VALIGN", (0, 0), (-1, -1), "TOP")]),
          PageBreak()]

    s += [P(T["h4"], h1),
          P(T["p4a"]),
          plan_door(),
          P(T["cap4"], cap),
          P(T["p4b"]),
          table(rows(T["t4"]), [110, 30]),
          Spacer(1, 3 * mm),
          P(T["p4c"]),
          P(T["p4d"])]
    s += bullets(T["links"])
    s += [P(T["h_order"], h2),
          P(T["p_order"])]

    s += [P(T["h5"], h1),
          P(T["h_dw"], h2),
          P(T["p_dw"]),
          P(T["h_ext"], h2),
          P(T["p_ext"]),
          P(T["h6"], h1)]
    s += bullets(T["calc"])
    s += [P(T["h_vals"], h2),
          table(rows(T["t6"]), [56, 36, 72]),
          Spacer(1, 4 * mm),
          P(T["h_list"], h2)]
    s += bullets(T["list"])
    s += [Spacer(1, 6 * mm), P(T["end"], small)]

    doc = SimpleDocTemplate(out, pagesize=A4, leftMargin=18 * mm, rightMargin=18 * mm, topMargin=16 * mm,
                            bottomMargin=18 * mm, title=T["doc_title"], author="Macchecaspita", lang=lang)
    doc.build(s, onFirstPage=footer, onLaterPages=footer)
    print(out)


if __name__ == "__main__":
    for lang in sys.argv[1:] or LANGS:
        build(lang)
