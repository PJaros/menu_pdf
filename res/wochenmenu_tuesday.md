#import sys: inputs

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
  }
}

#set text(
  font: "Helvetica",
  size: 14pt,
)

#figure(
  image("Titel.png", width: 80%),
)

#let content = inputs

#table(
  columns: (33%, 33%, 33%),
  [MITTAGESSEN], [], [ABENDESSEN],

  [#content.di_mi], 
  [#upper[#content.di_day]\
  #content.di_date], 
  [#content.di_ab],
  
  [#content.mi_mi], 
  [#upper[#content.mi_day]\
  #content.mi_date], 
  [#content.mi_ab],
  
  [#content.do_mi], 
  [#upper[#content.do_day]\
  #content.do_date], 
  [#content.do_ab],
  
  [#content.fr_mi],
  [#upper[#content.fr_day]\
  #content.fr_date],
  [#content.fr_ab],
  
  [#content.sa_mi],
  [#upper[#content.sa_day]\
  #content.sa_date],
  [#content.sa_ab],
  
  [#content.so_mi],
  [#upper[#content.so_day]\
  #content.so_date],
  [#content.so_ab],

  [#content.mo_mi],
  [#upper[#content.mo_day]\
  #content.mo_date],
  [#content.mo_ab],
)

Zur Vorspeise gehören täglich Suppe und Salat, beim Hauptgang Saisongemüse und ein Dessert dazu

Fleisch aus Schweizer Tierhaltung/ Fisch: Estland, Norwegen, Holland. Brot und Backwaren aus der Schweiz.