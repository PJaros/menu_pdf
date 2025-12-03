# Menu -> PDF
Generates a menu PDF for a weekly menu with several layouts.

The data is autosaved to an INI-file.

# Features
The GUI shows a week worth of data with a separated textboxs for lunch and dinner for every day. On the top two arrows
allow to change the current week. A date selector allows to switch directly to any week.

This application ships as a single executable file with batteries included. The stored data is saved as plain text INI
file.

There is no load or save button. The data is loaded at start. The program checks every 5 seconds and saves new data if
needed. Additionally changing the week, printing a document and closing the application will also save the data.

This project is developed under Linux. It has been tested and works on Windows.

# CLI
The GUI can be zoomed with an option. Alternatively one can call the application to create a PDF with embedded demo data
instead without presenting the GUI.

```bash
Usage: menu_pdf [OPTIONS]

Options:
  -z, --zoom <ZOOM>  [default: 1]
  -d, --demo-pdf     
  -h, --help         Print help
  -V, --version      Print version
```

# Dependencies
 - Uses https://github.com/typst/typst to design and create the PDF
 - Uses https://github.com/Relacibo/typst-as-lib to integrate Typst into the application
 - Uses https://github.com/Byron/open-rs to open the generated PDF
 - Uses https://github.com/clap-rs/clap to interpret command-line options

# Motivation
The main motivation is to learn Rust. I work best when there is a real-world problem to solve. I like building GUIs and
typesetting with Typst. I decided to use Egui as it supports accessibility, runs on Linux, Windows. While untested it
will most probably also work on MacOS and, with some more work, also in a browser.

My wife writes a weekly menu at work. The same menu data is shown in different forms on several sheets. Copying and
formating this in a word processor is a repetitive tedious job. A program helps here.

# Limitations
- Sadly Egui is unable to run on a virtualized Windows 11 as Egui doesn't support the graphics driver for the 
  virtual hardware. I've tested this on QEMU/KVM. It's a known Egui limitation at the time of writing,
- Egui seems to be unable to detect light-mode (at least on my linux) and will always run the application on dark-mode.
  So I forced the application to light-mode.
- Egui needs a rather ugly hack to resize the window automatically to the minimal needed size of its widgets.
- The GUI changes the window size when the computer goes to sleep and wakes up.

# Further ideas
 - Show the version on the GUI.
 - Add Application Icons on the executable and in the taskbar

   I have collected some links how to change the executable icons and how to change the icon in the taskbar. I
   haven't spent much time on this, just saved the links for later.
   - Example on how to load images and display it in egui: https://github.com/emilk/egui/tree/main/examples/images
   - Macro on how to include external files into the exetuable: https://docs.rs/egui/latest/egui/macro.include_image.html
   - How do I change taskbar icon: https://github.com/emilk/egui/discussions/3971
