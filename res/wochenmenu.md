#import sys: inputs
#let content = inputs
#let footer_text = "Zur Vorspeise gehören täglich Suppe und Salat, beim Hauptgang Saisongemüse und ein Dessert dazu

Fleisch aus Schweizer Tierhaltung/ Fisch: Estland, Norwegen, Holland. Brot und Backwaren aus der Schweiz."

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

#table(
  columns: (33%, 33%, 33%),
  [MITTAGESSEN], [], [ABENDESSEN],

  [#content.mo_mi], 
  [#upper[#content.mo_day]\
  #content.mo_date], 
  [#content.mo_ab],
  
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
)

#footer_text

#pagebreak()

#set text(
  font: "Helvetica",
  size: 13pt,
)

#figure(
  image("Titel.png", width: 80%),
)

#table(
  fill: (x, y) =>
    if y == 0 { rgb("#7F7F7F") }
    else if x == 3 { rgb("#FFF7D8") }
    else { rgb("#FFC000") },

  columns: (25%, 25%, 25%, 25%),
  [MITTAGESSEN], [], [ABENDESSEN], [],

  [#content.mo_mi], 
  [#upper[#content.mo_day]\
  #content.mo_date], 
  [#content.mo_ab],
  [],
  
  [#content.di_mi], 
  [#upper[#content.di_day]\
  #content.di_date], 
  [#content.di_ab],
  [],
  
  [#content.mi_mi], 
  [#upper[#content.mi_day]\
  #content.mi_date], 
  [#content.mi_ab],
  [],
  
  [#content.do_mi], 
  [#upper[#content.do_day]\
  #content.do_date], 
  [#content.do_ab],
  [],
  
  [#content.fr_mi],
  [#upper[#content.fr_day]\
  #content.fr_date],
  [#content.fr_ab],
  [],
  
  [#content.sa_mi],
  [#upper[#content.sa_day]\
  #content.sa_date],
  [#content.sa_ab],
  [],

  [#content.so_mi],
  [#upper[#content.so_day]\
  #content.so_date],
  [#content.so_ab],
  [],
)

#footer_text

#pagebreak()
// Tuesday - Thursday
#set text(
  font: "Helvetica",
  size: 14pt,
)

#figure(
  image("Titel.png", width: 80%),
)

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
)

#footer_text

#pagebreak()
// Friday - Monday
#set text(
  font: "Helvetica",
  size: 14pt,
)

#figure(
  image("Titel.png", width: 80%),
)

#table(
  columns: (33%, 33%, 33%),
  [MITTAGESSEN], [], [ABENDESSEN],
  
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
  
  [#content.mo_mi_2],
  [#upper[#content.mo_day_2]\
  #content.mo_date_2],
  [#content.mo_ab_2],
)

#footer_text
