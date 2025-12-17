#import sys: inputs
// #let content = inputs
#let content = toml("menu.ini")
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
    // top: 0cm,
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

  [#content.w.mo_mi], 
  [#upper[#content.w.mo_day]\
  #content.w.mo_date], 
  [#content.w.mo_ab],
  
  [#content.w.di_mi], 
  [#upper[#content.w.di_day]\
  #content.w.di_date], 
  [#content.w.di_ab],
  
  [#content.w.mi_mi], 
  [#upper[#content.w.mi_day]\
  #content.w.mi_date], 
  [#content.w.mi_ab],
  
  [#content.w.do_mi], 
  [#upper[#content.w.do_day]\
  #content.w.do_date], 
  [#content.w.do_ab],
  
  [#content.w.fr_mi],
  [#upper[#content.w.fr_day]\
  #content.w.fr_date],
  [#content.w.fr_ab],
  
  [#content.w.sa_mi],
  [#upper[#content.w.sa_day]\
  #content.w.sa_date],
  [#content.w.sa_ab],
  
  [#content.w.so_mi],
  [#upper[#content.w.so_day]\
  #content.w.so_date],
  [#content.w.so_ab],
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

  [#content.w.mo_mi], 
  [#upper[#content.w.mo_day]\
  #content.w.mo_date], 
  [#content.w.mo_ab],
  [],
  
  [#content.w.di_mi], 
  [#upper[#content.w.di_day]\
  #content.w.di_date], 
  [#content.w.di_ab],
  [],
  
  [#content.w.mi_mi], 
  [#upper[#content.w.mi_day]\
  #content.w.mi_date], 
  [#content.w.mi_ab],
  [],
  
  [#content.w.do_mi], 
  [#upper[#content.w.do_day]\
  #content.w.do_date], 
  [#content.w.do_ab],
  [],
  
  [#content.w.fr_mi],
  [#upper[#content.w.fr_day]\
  #content.w.fr_date],
  [#content.w.fr_ab],
  [],
  
  [#content.w.sa_mi],
  [#upper[#content.w.sa_day]\
  #content.w.sa_date],
  [#content.w.sa_ab],
  [],

  [#content.w.so_mi],
  [#upper[#content.w.so_day]\
  #content.w.so_date],
  [#content.w.so_ab],
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
  
  [#content.w.di_mi],
  [#upper[#content.w.di_day]\
  #content.w.di_date],
  [#content.w.di_ab],
  
  [#content.w.mi_mi],
  [#upper[#content.w.mi_day]\
  #content.w.mi_date],
  [#content.w.mi_ab],
  
  [#content.w.do_mi],
  [#upper[#content.w.do_day]\
  #content.w.do_date],
  [#content.w.do_ab],
)
#table_comment_big

#pagebreak()
// Friday - Monday of next week
#title
#table(
  inset: (y: 1.5em),
  columns: (1fr, auto, 1fr),
  h[MITTAGESSEN], h[], h[ABENDESSEN],
  
  [#content.w.fr_mi],
  [#upper[#content.w.fr_day]\
  #content.w.fr_date],
  [#content.w.fr_ab],
  
  [#content.w.sa_mi],
  [#upper[#content.w.sa_day]\
  #content.w.sa_date],
  [#content.w.sa_ab],
  
  [#content.w.so_mi],
  [#upper[#content.w.so_day]\
  #content.w.so_date],
  [#content.w.so_ab],
  
  [#content.w.mo_mi_2],
  [#upper[#content.w.mo_day_2]\
  #content.w.mo_date_2],
  [#content.w.mo_ab_2],
)
#table_comment_big

