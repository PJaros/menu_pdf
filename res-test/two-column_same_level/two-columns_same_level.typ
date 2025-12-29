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
    bottom: 5cm,
    left: 1.75cm,
    right: 1.75cm,
  ),
  footer: rect(stroke:none)[
    #set align(center)
    #set text(font: "Hanken Grotesk", size: 12pt)
    #footer1
    #footer2

    #box(image("ornament_2_l.svg"), height: 18pt)
    #box(image("ornament_2_r.svg"), height: 18pt)
  ],
)

#{
  set text(font: "Lavishly Yours", size: 60pt)
  set par(spacing: 0.8em)
  set align(center)
  box(image("ornament_1_l.svg"),height:1em)
  h(0.25em)
  title
  h(0.25em)
  box(image("ornament_1_r.svg"),height:1em)
}

#set text(font: "Hanken Grotesk", size: 16pt)
#set align(left)

#let cday(day) =  grid.cell(text([#underline(day)]))
#let rday(day) =  grid.cell(align:right, text([#underline(day)]))
#let r(t) =  grid.cell(align: right, [#t])


#grid(
  columns: (auto, 1fr, 1fr),
  gutter: 1em,

  [], underline[#content.w.mo_day, #content.w.mo_date],
  rday([#content.w.di_day, #content.w.di_date]),
  [Mittag:], [#content.w.mo_mi], r(content.w.di_mi), 
  [Abend:],  [#content.w.mo_ab], r(content.w.di_ab),
  [], [], [],  
  [], underline[#content.w.mi_day, #content.w.mi_date],
  rday([#content.w.do_day, #content.w.do_date]),
  [Mittag:], [#content.w.mi_mi], r(content.w.do_mi), 
  [Abend:],  [#content.w.mi_ab], r(content.w.do_ab),
  [], [], [],  
  [], underline[#content.w.fr_day, #content.w.fr_date],
  rday([#content.w.sa_day, #content.w.sa_date]),
  [Mittag:], [#content.w.fr_mi], r(content.w.sa_mi), 
  [Abend:],  [#content.w.fr_ab], r(content.w.sa_ab),
)

#pad(
  top: 1em,
  x: 20%,
  grid(
    columns: 2,
    gutter: 1em,

    [], underline[#content.w.so_day, #content.w.so_date],
    [Mittag:], [#content.w.so_mi], 
    [Abend:],  [#content.w.so_ab],
  )
)

