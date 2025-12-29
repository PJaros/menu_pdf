# Readme

Typst is still pretty new to me. At first I just tried to recreate the giving layout of the old solution. But later I got curious if I can recreated more complicate designs.

## Raphael

"Alterswohnheim Raphael" used a Word file with tables to create the weekly menu printout. I received the original Docx file. Opening the file with Libreoffice Writer was useless as Libreoffice Writer showed the pages significantly different compared to Microsoft Word. I asked to get a printed version and used that to model the layout.

[<img src="raphael/spec/Menüplan_scan.png" alt="Scanned menu card">](raphael/spec/Menüplan_scan.pdf)

_The image here (and also the others) are linked to the PDF_

The yellow colors turned orange on my scanner. So the Docx file was used to find the correct color codes.

The egui application uses a INI file to store the data. Typst can read TOML data which is very similar. I am use various datasets to see how much data can fit on a single page.

[<img src="raphael/wochenmenu_155.png" alt="Typst generate Raphael menu card"/>](raphael/wochenmenu.pdf)

## Lines layout



## Two-column layout

I invested some time to see how other menu cards are designed. I found a [two-column menu card design on pinterest](https://ch.pinterest.com/pin/521854675593920037/). I applied the same data to show the overview of one week.

## Further Tools

The `watch.sh` script starts a `typst watch` with the correct resources and will automatically compile a new PDF file upon changing the TYP-file or the menu.ini. Changing the font requires a restart of the script.

This command will create a thumbnail-sized preview:
```bash
typst compile -f png --ppi=40 --font-path . two-column.typ two-column.png
```
 
To reorder pages in a PDF:
```bash
sudo pacman -S pdftk
pdftk '2025-09-22 Menüplan Alterswohnheim Raphael.pdf'  cat 1-3 5 4 output Menuüplan_scan.pdf
```

### Markdown Syntax
Linking resources for github markdown with a images to click on instead of text: https://stackoverflow.com/a/61072867/406423