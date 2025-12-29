#import sys: inputs
// #let content = inputs
#let content = toml("menu.ini")
#let title = "Menü"
#let footer1 = "Zur Vorspeise gehören täglich Suppe und Salat, beim Hauptgang Saisongemüse und ein Dessert dazu."
#let footer2 = "Fleisch aus Schweizer Tierhaltung. Fisch: Estland, Norwegen, Holland. Brot und Backwaren aus der Schweiz."
#set page(
  paper: "a4",
  margin: (
    top: 1.25cm,
    bottom: 3cm,
    left: 1.5cm,
    right: 1.5cm,
  ),
  footer: rect(stroke:none)[
    #set align(center)
    #set text(font: "Roboto", stretch: 50%, size: 11pt)
    // #text(size: 18pt, weight: "medium", footer1)

    #footer2
  ],
)

#set align(center)
#block(
  inset: (bottom: 10pt),
  text(font: "Yellowtail", size: 80pt, "Menü")
)
#block(
  inset: (bottom: 20pt),
  text(font: "Roboto", size: 18pt, stretch: 50%, [#content.w.mo_day, #content.w.mo_date])
)

// #set text(font: "Yellowtail", size: 60pt)
// #set align(center)
// Menü

// #set text(font: "Roboto", stretch: 50%, weight: "bold", size: 18pt)
// #set text(font: "Roboto", stretch: 50%, size: 18pt)
#set text(font: "Roboto", stretch: 50%, weight: "medium", size: 18pt)
// #set text(hyphenate: true, lang: "de")
// #set par(justify: true)
// #set align(left)
#set grid(
  // columns: (auto, 1fr),
  columns: 3,
  stroke: none,
  align: left,
  // row-gutter: (0.3em, auto),
  // gutter: 3pt,
  // inline: 0.3em
  // inset: (x, y) => if x == 0 and y > 1 (200pt) else (x: 50pt, y:50pt), 
  inset: (x:5pt, y:5pt),
)
// #show grid.cell.where(x: 0): set grid.cell(inset: 15pt)
#let erow() = grid.cell(colspan: 3, inset:(y:3pt), text(size: 0.1pt, []))
#let cday(day) =  grid.cell(rowspan: 2, inset:(x:12pt, y:5pt), text(font: "Yellowtail", size: 40pt, [#day.slice(0, 2)]))
#let mrow(mi, ab) = (
  grid.cell(inset: (top: 0.2em), box(image("sunny.svg"),height:0.8em)), [#mi],
  grid.cell(inset: (top: 0.2em), box(image("nightlight.svg"),height:0.8em)), [#ab])

#set align(center)
#let vy = 0
#grid(
  // grid.hline(), erow(),
  cday(content.w.mo_day),
  // grid.vline(start: vy, end: vy + 2),
  ..mrow(content.w.mo_mi, content.w.mo_ab),

  erow(), grid.hline(), erow(),
  cday(content.w.di_day),
  // grid.vline(start: vy + 4, end: vy + 6),
  ..mrow(content.w.di_mi, content.w.di_ab),

  erow(), grid.hline(), erow(),
  cday(content.w.mi_day),
  // grid.vline(start: vy + 8, end: vy + 10),
  ..mrow(content.w.mi_mi, content.w.mi_ab),

  erow(), grid.hline(), erow(),
  cday(content.w.do_day),
  // grid.vline(start: vy + 12, end: vy + 14),
  ..mrow(content.w.do_mi, content.w.do_ab),

  erow(), grid.hline(), erow(),
  cday(content.w.fr_day),
  // grid.vline(start: vy + 16, end: vy + 18),
  ..mrow(content.w.fr_mi, content.w.fr_ab),

  erow(), grid.hline(), erow(),
  cday(content.w.sa_day),
  // grid.vline(start: vy + 20, end: vy + 22),
  ..mrow(content.w.sa_mi, content.w.sa_ab),

  erow(), grid.hline(), erow(),
  cday(content.w.so_day),
  // grid.vline(start: vy + 24, end: vy + 26),
  ..mrow(content.w.so_mi, content.w.so_ab),
  // erow(), grid.hline()
)

#v(10pt)
#text(size: 18pt, weight: "regular", footer1)
