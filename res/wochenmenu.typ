#import sys: inputs
#let content = inputs
// #let content = toml("menu.ini")
// #let content = toml("menu_mid.ini")
// #let content = toml("menu_max.ini")
#let tc_text = "Zur Vorspeise gehören täglich Suppe und Salat, beim Hauptgang Saisongemüse und ein Dessert dazu."
#let table_comment = {
  v(2pt)
  set text(size:14pt)
  set align(center)
  strong(tc_text)
}
#let table_comment_big = {
  v(10pt)
  set text(size:16pt)
  set align(center)
  strong(tc_text)
}

#let title = {
  figure(image("Titel.png", width: 80%))
  v(15pt)
}

#let page_footer = "Fleisch aus Schweizer Tierhaltung. Fisch aus Estland, Norwegen und Holland. Brot und Backwaren aus der Schweiz."

#set page(
  paper: "a4",
  margin: (
    top: 1.25cm,
    bottom: 2.5cm,
    left: 1.27cm,
    right: 1.27cm,
  ),
  footer: align(center)[
    #set text(font: "Helvetica", size: 10pt)
    #page_footer
  ]
)

#set table(
  stroke: none,
  fill: (x, y) =>
    if y == 0 { rgb("#7F7F7F") }
    else if x == 3 { rgb("#FFF7D8") }
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

// Overview
#title
#set text(font: "Helvetica", size: 14pt)
// #set text(lang: "de", hyphenate: true)

#table(
  columns: (1fr, auto, 1fr),
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
#table_comment

#pagebreak()

// Overview with space for notes
#title
#set text(font: "Helvetica", size: 13pt)
#table(
  row-gutter: 1.15em,

  columns: (1fr, auto, 1fr, 1fr,),
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
#table_comment

#pagebreak()
// Tuesday - Thursday
#title
#set text(font: "Helvetica", size: 17pt)
#let h(text) = table.cell(inset: 0.75em, text)

#table(
  inset: (y: 2em),
  columns: (1fr, auto, 1fr),
  h[MITTAGESSEN], h[], h[ABENDESSEN],
  
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
#table_comment_big

#pagebreak()
// Friday - Monday of next week
#title
#table(
  inset: (y: 1.5em),
  columns: (1fr, auto, 1fr),
  h[MITTAGESSEN], h[], h[ABENDESSEN],
  
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
#table_comment_big

