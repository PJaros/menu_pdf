#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chrono::{Datelike, Days, Local, NaiveDate};
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
const _DEMO_INI_FILE_PATH: &str = "demo_menu.ini";
const INI_FILE_PATH: &str = "menu.ini";
const UI_DATE_FORMAT: &str = "%e. %b %Y";
const INI_DATE_FORMAT: &str = "%Y-%m-%d";

type WeekData = [[String; 2]; 7];

fn get_closest_last_monday(datum: &mut NaiveDate) -> NaiveDate {
    let day: u64 = (datum.weekday().number_from_monday() - 1).into();
    datum
        .checked_sub_days(Days::new(day))
        .to_owned()
        .expect("Calculating closest past monday failed")
}

// # Rules for WeekDate persistence
//
// - Absent menu.ini is treated the same as a empty menu.ini.
// - Absent week-section is treated the same as a empty week-section.
// - A week-section with one entry with a single non-empty entry will be written with all entries.
// - Entries will be trimmed on load, save and before comparing.

fn create_empty_week() -> WeekData {
    // Initialize 2D array, from: https://users.rust-lang.org/t/how-to-init-2d-array-using-function/80737/2
    let week_string: WeekData = array::from_fn(|_y| array::from_fn(|_x| "".to_string()));
    week_string
}

fn load_week(datum: &NaiveDate) -> WeekData {
    let date_string = datum.format(INI_DATE_FORMAT).to_string();
    let ini_path = Path::new(INI_FILE_PATH);
    let conf: Ini = match ini_path.exists() {
        true => Ini::load_from_file(ini_path).expect("Error loading ini file"),
        false => Ini::new(),
    };

    let mut week_string = create_empty_week();
    for (y, day) in DAY_SHORT.iter().enumerate() {
        for (x, time) in TIME_SHORT.iter().enumerate() {
            let key = format!("{day}_{time}");
            week_string[y][x] = conf
                .get_from_or(Some(date_string.as_str()), key.as_str(), "")
                .to_owned();
        }
    }
    week_string
}

fn save_week(week_data: &WeekData, date_str: &str) {
    let ini_path = Path::new(INI_FILE_PATH);
    let mut conf: Ini = match ini_path.exists() {
        true => Ini::load_from_file(ini_path).expect("Error loading ini file"),
        false => Ini::new(),
    };

    let mut section = conf.with_section(Some(date_str));
    for (y, day) in DAY_SHORT.iter().enumerate() {
        for (x, time) in TIME_SHORT.iter().enumerate() {
            let key = format!("{day}_{time}");
            section.set(key, week_data[y][x].trim());
        }
    }
    conf.write_to_file(INI_FILE_PATH)
        .expect("Error writing ini file");
}

fn save_if_needed(week_data: &WeekData, datum: &NaiveDate) {
    let date_string = datum.format(INI_DATE_FORMAT).to_string();
    let date_str = date_string.as_str();
    let ini_path = Path::new(INI_FILE_PATH);
    let conf: Ini = match ini_path.exists() {
        true => Ini::load_from_file(ini_path).expect("Error loading ini file"),
        false => Ini::new(),
    };

    // Initialize empty 2D array, from: https://users.rust-lang.org/t/how-to-init-2d-array-using-function/80737/2
    let mut week_loaded: WeekData = create_empty_week();
    for (y, day) in DAY_SHORT.iter().enumerate() {
        for (x, time) in TIME_SHORT.iter().enumerate() {
            let key = format!("{day}_{time}");
            week_loaded[y][x] = conf
                .get_from_or(Some(date_str), key.as_str(), "")
                .to_owned();
        }
    }
    if !is_week_equal(&week_loaded, week_data) {
        info!("Saving, was not equal");
        save_week(week_data, date_string.as_str());
    }
}

fn is_week_equal(week_data1: &WeekData, week_data2: &WeekData) -> bool {
    for y in 0..DAY_SHORT.len() {
        for x in 0..TIME_SHORT.len() {
            if week_data1[y][x].trim() != week_data2[y][x].trim() {
                info!(
                    "week_data1[{}][{}] != week_data2[{}][{}], '{}' != '{}'",
                    x, y, x, y, week_data1[y][x], week_data2[y][x]
                );
                return false;
            }
        }
    }
    true
}

fn main() -> eframe::Result {
    const DAYS_IN_WEEK: Days = Days::new(7);
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([760.0, 800.0]),
        ..Default::default()
    };

    // let ini_path = Path::new(DEMO_INI_FILE_PATH);
    // let conf: Ini = match ini_path.exists() {
    //     true => Ini::load_from_file(ini_path).expect("Error loading ini file"),
    //     false => Ini::new(),
    // };
    //
    // let mut week_data: WeekData = create_empty_week();
    // for (y, day) in DAY_SHORT.iter().enumerate() {
    //     for (x, time) in TIME_SHORT.iter().enumerate() {
    //         let key = format!("{day}_{time}");
    //         week_data[y][x] = conf.get_from_or(Some("Week"), key.as_str(), "").to_owned();
    //     }
    // }

    // Calculate closest past (or today's) monday
    let mut datum = Local::now().date_naive();
    datum = get_closest_last_monday(&mut datum);
    let mut selected_monday = datum; // save selected monday
    let mut week_data = load_week(&datum);

    eframe::run_simple_native(TITLE, options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // From: https://github.com/emilk/egui/discussions/1627
            ctx.set_visuals(Visuals::light());
            ctx.set_pixels_per_point(1.5);

            ui.horizontal(|ui| {
                ui.label("Datum: ");
                let left_button = ui.button("<");
                let datepicker_button =
                    ui.add(DatePickerButton::new(&mut datum).format(UI_DATE_FORMAT));
                let right_button = ui.button(">");

                if left_button.clicked() {
                    save_if_needed(&week_data, &datum);
                    datum = datum
                        .checked_sub_days(DAYS_IN_WEEK)
                        .to_owned()
                        .expect("Subtracting 7 days failed.");
                    selected_monday = datum;
                    week_data = load_week(&datum);
                };
                if datepicker_button.changed() {
                    datum = get_closest_last_monday(&mut datum).to_owned();
                    if selected_monday == datum {
                        info!(
                            "Same date. selected_monday: {}, datum: {}",
                            selected_monday, datum
                        );
                    } else {
                        info!(
                            "Different date. selected_monday: {}, datum: {}",
                            selected_monday, datum
                        );
                        save_if_needed(&week_data, &selected_monday);
                        let date_string = datum.format(INI_DATE_FORMAT).to_string();
                        let date_str = date_string.as_str();
                        let ini_path = Path::new(INI_FILE_PATH);
                        let conf: Ini = match ini_path.exists() {
                            true => Ini::load_from_file(ini_path).expect("Error loading ini file"),
                            false => Ini::new(),
                        };

                        let mut week_string: WeekData = create_empty_week();
                        for (y, day) in DAY_SHORT.iter().enumerate() {
                            for (x, time) in TIME_SHORT.iter().enumerate() {
                                let key = format!("{day}_{time}");
                                week_string[y][x] = conf
                                    .get_from_or(Some(date_str), key.as_str(), "")
                                    .to_owned();
                            }
                        }
                        week_data = week_string;
                    }
                    selected_monday = datum;
                }
                if right_button.clicked() {
                    save_if_needed(&week_data, &datum);
                    datum = datum
                        .checked_add_days(DAYS_IN_WEEK)
                        .to_owned()
                        .expect("Adding 7 days failed.");
                    selected_monday = datum;
                    week_data = load_week(&datum);
                }
            });

            egui::Grid::new("grid_id").show(ui, |ui| {
                ui.label("");
                for s in TIME_LONG.into_iter() {
                    ui.label(s);
                }
                ui.end_row();

                for (i, day) in DAY_LONG.iter().enumerate() {
                    ui.label(*day);
                    ui.add(
                        TextEdit::multiline(&mut week_data[i][0])
                            .min_size([EDIT_WIDTH, 1.0].into()),
                    );
                    ui.add(
                        TextEdit::multiline(&mut week_data[i][1])
                            .min_size([EDIT_WIDTH, 1.0].into()),
                    );
                    ui.end_row();
                }

                ui.label("");
                if ui.button("Load").clicked() {
                    week_data = load_week(&datum);
                }

                if ui.button("Save").clicked() {
                    save_if_needed(&week_data, &datum);
                }
            });
        });
    })
}
