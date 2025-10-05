# Menu -> PDF

Generates menu pdf for a weekly overview what is going to be cooked forlunch and dinner.

## Current state

This project is still work in progress. And important features are still missing.
Currently it shows a egui window with a calendar selector and several multiline editors.
Data is loaded from the demo_menu-ini file.

Todo:
 - Reading and writing to INI files is tested. But currently only a demo-file read is implemented.
 - There is a typst prototype ready to be included. Typst will need to added to this project with typst-as-lib or typst
   will need to ship with a compiled typst. 