# Menu -> PDF

Generates menu pdf for a weekly overview what is going to be cooked for lunch and dinner.

## Current state

This project is still work in progress. And important features are still missing.
Currently, it shows a egui window with a calendar selector and several multiline editors.
Data is loaded from the demo_menu-ini file.

# Dependencies
 - Uses https://github.com/typst/typst to design and create the PDF
 - Uses https://github.com/Relacibo/typst-as-lib to integrate Typst into this application
 - Uses https://github.com/Byron/open-rs to open the generated PDF

# Todo
 - Transfer data to typst document