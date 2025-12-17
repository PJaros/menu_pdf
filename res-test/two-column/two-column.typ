#import sys: inputs
#let content = inputs
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

//#figure(
//  image("Titel.png", width: 80%),
//)

#show table.cell.where(y: 0): set text(weight: "bold")
#show table.cell.where(y: 0): set align(center)

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

#show table.cell.where(y: 0): set text(weight: "bold")
#set table(
  columns: (auto, 1fr),
  // columns: 2,
  stroke: none,
  row-gutter: (0.3em, auto),
)
#let gdark = green.darken(50%)
#let gcell(name) = (table.cell(text(fill: gdark, name)))
#let ghline = table.hline(stroke: gdark)
#let gheader(text) = table.cell(colspan: 2, gcell(text))
#let rdark = red.darken(50%)
#let rcell(name) = (table.cell(text(fill: rdark, name)))
#let rhline = table.hline(stroke: rdark)
#let rheader(text) = table.cell(colspan: 2, rcell(text))

#columns(2, gutter: 1.5cm)[
#table(
  gheader[Montag, 15. September],
  ghline,
  gcell[Mittag:], gcell[Steinpilzrisotto mit Randengemüse],
  gcell[Abend:], gcell[Cafe Complet],
)
#v(1em)
#table(
  rheader[Mittwoch, 17. September],
  rhline,
  rcell[Mittag:], rcell[Gschwellti mit Käse und Zibu],
  rcell[Abend:], rcell[Cafe complet],
)
#v(1em)
#table(
  gheader[Freitg, 19. September],
  ghline,
  gcell[Mittag:], gcell[Käsfladen],
  gcell[Abend:], gcell[Wienerli oder Haloumi im Schlafrock],
)
#v(1em)
#table(
  rheader[Sonntag, 21. September],
  rhline,
  rcell[Mittag:], rcell[Kalbsgeschnetzeltes mit Rösti],
  rcell[Abend:], rcell[Fruchtfladen],
)

#colbreak()
#v(3em)
#table(
  rheader[Dienstag, 16. September],
  rhline,
  rcell[Mittag:], rcell[Trutenpicatta Selleriepicatta Tomatenspaghetti],
  rcell[Abend:], rcell[Eierbrötli],
)
#v(1em)
#table(
  gheader[Donnerstag, 18. September],
  ghline,
  gcell[Mittag:], gcell[Kartoffelstock mit Hackbraten oder Linsenhackbraten],
  gcell[Abend:], gcell[Pizza],
)
#v(1em)
#table(
  rheader[Samstag, 20. September],
  rhline,
  rcell[Mittag:], rcell[Braten mit Nudeln],
  rcell[Abend:], rcell[Cafe complet],
)

/*#table(
  table.cell(colspan: 2, [Donnerstag, 18. September]),
  table.hline(),
  [Mittag:], [Kartoffelstock mit Hackbraten oder Linsenhackbraten],
  [Abend:], [Pizza],
)*/
]

/*
#table(
columns: (33%, 33%, 33%),
  [MITTAGESSEN], [], [ABENDESSEN],

  [Steinpilzrisotto mit Randengemüse], 
  [MONTAG\ 
  15. September], 
  [Cafe Complet],
  
  [Trutenpicatta Selleriepicatta Tomatenspaghetti], 
  [DIENSTAG\ 
  16. September], 
  [Eierbrötli],
  
  [Gschwellti mit Käse und Zibu], 
  [MITTWOCH\ 
  17. September], 
  [Cafe complet],
  
  [Kartoffelstock mit
Hackbraten oder
Linsenhackbraten], 
  [DONNERSTAG\ 
  18. September], 
  [Pizza],
  
  [Käsfladen],
  [FREITAG\
  19. September],
  [Wienerli oder Haloumi im Schlafrock],
  
  [Braten mit Nudeln],
  [SAMSTAG\
  20. September],
  [Cafe complet],
  
  [Kalbsgeschnetzeltes mit Rösti],
  [SONNTAG\
  21. September],
  [Fruchtfladen],
)

#footer_text
*/
