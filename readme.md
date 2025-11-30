# Menu -> PDF

Generates menu pdf for a weekly overview what is going to be cooked for lunch and dinner.

## Current state

This project is almost feature complete.

Currently, it shows a egui window with a calendar selector and several multiline editors.
Data is loaded and saved from the menu.ini file.

It has been developed and tested under Linux. It has been (briefly) tested on Windows 11 and works. But it won't work on
a virtualized Windows 11 as egui doesn't support that.

# Dependencies
 - Uses https://github.com/typst/typst to design and create the PDF
 - Uses https://github.com/Relacibo/typst-as-lib to integrate Typst into this application
 - Uses https://github.com/Byron/open-rs to open the generated PDF
 - Uses https://github.com/clap-rs/clap to interpret command-line options

# Todo
 - Add 3rd typst template spanning
