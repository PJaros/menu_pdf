#set page(
  paper: "a4",
  margin: (
    top: 1.25cm,
    bottom: 1.25cm,
    left: 1.27cm,
    right: 1.27cm,
  )
)

#set table(
  stroke: none,
  fill: (_, y) =>
    if y == 0 { rgb("#7F7F7F") }
    else { rgb("#FFC000") },
  // inset: (right: 1.5em),
  inset: 0.75em,
  align: center + horizon,
  row-gutter: 1em,
)

#show table.cell: it => {
  if it.y == 0 {
    set text(white)
    strong(it)
  } else {
    strong(it)
    // it
  }
}

#set text(
  // font: "Libertinus Serif",
  font: "Helvetica",
  size: 14pt,
)


#figure(
  image("Titel.png", width: 80%),
)


/*#let a = table.cell(
  fill: green.lighten(60%),
)[A]
#let b = table.cell(
  fill: aqua.lighten(60%),
)[B]*/

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

Zur Vorspeise gehören täglich Suppe und Salat, beim Hauptgang Saisongemüse und ein Dessert dazu

Fleisch aus Schweizer Tierhaltung/ Fisch: Estland, Norwegen, Holland. Brot und Backwaren aus der Schweiz.