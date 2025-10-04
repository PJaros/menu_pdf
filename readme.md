# Menu -> PDF

Generates menu pdf for a weekly overview what is going to be cooked for lunch and dinner.

## Current state

This is a work in progress. I shows a egui window with a calendar selector and several multiline editors.

Currently I'm working on dealing with figuring out how to store egui fields in a 2D array containing string data.
Later I will persist those values within a INI file.

There is a typst prototype ready to be included. It will use the data to generate the pdf. 