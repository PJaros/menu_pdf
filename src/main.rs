#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chrono::{Datelike, Days, Local};
use eframe::egui;
use eframe::egui::TextEdit;
use eframe::egui::Visuals;
use egui_extras::DatePickerButton;
use log::info;
use std::array;

use ini::Ini;
use std::path::Path;
use std::string::String;

const EDIT_WIDTH: f32 = 200.0;
const TITLE: &str = "Menu → PDF";
const TIME_LONG: [&str; 2] = ["Mittag", "Abend"];
const TIME_SHORT: [&str; 2] = ["mi", "ab"];
const DAY_LONG: [&str; 7] = [
    "Montag",
    "Dienstag",
    "Mittwoch",
    "Donnerstag",
    "Freitag",
    "Samstag",
    "Sonntag",
];
const DAY_SHORT: [&str; 7] = ["mo", "di", "mi", "do", "fr", "sa", "so"];
const INI_FILE_PATH: &str = "demo_menu.ini";

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([760.0, 800.0]),
        ..Default::default()
    };

    let ini_path = Path::new(INI_FILE_PATH);
    let conf: Ini = match ini_path.exists() {
        true => Ini::load_from_file(ini_path).expect("Error loading ini file"),
        false => Ini::new(),
    };

    // Initialize 2D array, from: https://users.rust-lang.org/t/how-to-init-2d-array-using-function/80737/2
    let mut week_string: [[String; 2]; 7] =
        array::from_fn(|_y| array::from_fn(|_x| "".to_string()));

    for (y, day) in DAY_SHORT.iter().enumerate() {
        for (x, time) in TIME_SHORT.iter().enumerate() {
            let key = format!("{day}_{time}");
            week_string[y][x] = conf.get_from_or(Some("Week"), key.as_str(), "").to_owned();
        }
    }

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

            // TODO: Use arrays to write the ini file
            // TODO: On date selection: Check if a monday was selected. If not, correct it to the next past monday
            egui::Grid::new("grid_id").show(ui, |ui| {
                ui.label("");
                for s in TIME_LONG.into_iter() {
                    ui.label(s);
                }
                ui.end_row();

                for (i, day) in DAY_LONG.iter().enumerate() {
                    ui.label(*day);
                    ui.add(
                        TextEdit::multiline(&mut week_string[i][0])
                            .min_size([EDIT_WIDTH, 1.0].into()),
                    );
                    ui.add(
                        TextEdit::multiline(&mut week_string[i][1])
                            .min_size([EDIT_WIDTH, 1.0].into()),
                    );
                    ui.end_row();
                }

                ui.label("");
                let save_button = ui.button("Print content to log");
                if save_button.clicked() {
                    // set RUST_LOG to show log, eg: RUST_LOG=info cargo run
                    for (y, day) in DAY_SHORT.iter().enumerate() {
                        for (x, time) in TIME_SHORT.iter().enumerate() {
                            info!("{}_{} = {}", day, time, week_string[y][x]);
                        }
                    }
                }
            });
        });
    })
}
