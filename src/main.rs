#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chrono::{Datelike, Days, Local};
use eframe::egui;
use eframe::egui::TextEdit;
use eframe::egui::Visuals;
// use eframe::egui::{TextEdit, Ui};
use egui_extras::DatePickerButton;

// use ini::Ini;
// use std::path::Path;

const EDIT_WIDTH: f32 = 200.0;
const TITLE: &str = "Menu → PDF";
const TIME_LONG: [&str; 2] = ["Mittag", "Abend"];
const _TIME_SHORT: [&str; 2] = ["mi", "ab"];
const _WEEK_LONG: [&str; 7] = [
    "Montag",
    "Dienstag",
    "Mittwoch",
    "Donnerstag",
    "Freitag",
    "Samstag",
    "Sonntag",
];
const _WEEK_SHORT: [&str; 7] = ["mo", "di", "mi", "do", "fr", "sa", "so"];

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 480.0]),
        ..Default::default()
    };

    /*
        const INI_FILE_PATH: &'static str = "menu.ini";
        let ini_path = Path::new(INI_FILE_PATH);
        println!("{}", ini_path.exists());

        let mut conf;
        if (ini_path.exists()) {
            conf = Ini::load_from_file(ini_path).expect("Error loading ini file");
        } else {
            conf = Ini::new();
        }
        conf.write_to_file(ini_path)
            .expect("Error writing ini file");
    */

    let mut _array_sting = ["1".to_string(), "2".to_string()];
    let mut _array_sting2 = [["1".to_string(), "2".to_string()], ["3".to_string(), "4".to_string()]];
    let mut _array_str: [&str; 2] = ["1", "2"];
    let mut _array2_str: [[&str; 2]; 2] = [["1", "2"], ["3", "4"]];
    let mut montag_mittag = "".to_owned();
    let mut montag_abend = "".to_owned();
    let mut dienstag_mittag = "".to_owned();
    let mut dienstag_abend = "".to_owned();

    // Calculate closest past (or today's) monday
    let mut datum = Local::now().date_naive();
    let day: u64 = (datum.weekday().number_from_monday() - 1).into();
    datum = datum.checked_sub_days(Days::new(day)).to_owned().unwrap();

    eframe::run_simple_native(TITLE, options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // From: https://github.com/emilk/egui/discussions/1627
            ctx.set_visuals(Visuals::light());
            ctx.set_pixels_per_point(1.5);

            ui.horizontal(|ui| {
                ui.label("Datum: ");
                let left_button = ui.button("<");
                if left_button.clicked() {
                    datum = datum.checked_sub_days(Days::new(7)).to_owned().unwrap();
                };
                ui.add(DatePickerButton::new(&mut datum).format("%e. %b %Y"));
                let right_button = ui.button(">");
                if right_button.clicked() {
                    datum = datum.checked_add_days(Days::new(7)).to_owned().unwrap();
                }
            });

            // TODO: Use arrays to retain and read/write them ini file
            // TODO: On date selection: Check if a monday was selected. If not, correct it to the next past monday
            egui::Grid::new("grid_id").show(ui, |ui| {
                ui.label("");
                for s in TIME_LONG.into_iter() {
                    ui.label(s);
                }
                ui.end_row();

                ui.label("Montag");
                ui.add(TextEdit::multiline(&mut montag_mittag).min_size([EDIT_WIDTH, 1.0].into()));
                ui.add(TextEdit::multiline(&mut montag_abend).min_size([EDIT_WIDTH, 1.0].into()));
                ui.end_row();

                ui.label("Dienstag");
                ui.add(TextEdit::multiline(&mut dienstag_mittag).min_size([EDIT_WIDTH, 1.0].into()));
                ui.add(TextEdit::multiline(&mut dienstag_abend).min_size([EDIT_WIDTH, 1.0].into()));
                ui.end_row();
            });
        });
    })
}
