#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chrono::{Datelike, Days, Local, NaiveDate};
use eframe::egui::{Align2, Visuals};
use eframe::egui::{Context, TextEdit};
use eframe::{Frame, egui};
use egui_extras::DatePickerButton;
use log::info;

use ini::Ini;
use std::path::Path;
use week::WeekData;

mod week;

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
const DAYS_IN_WEEK: Days = Days::new(7);

fn get_closest_last_monday(datum: &mut NaiveDate) -> NaiveDate {
    let day: u64 = (datum.weekday().number_from_monday() - 1).into();
    datum
        .checked_sub_days(Days::new(day))
        .to_owned()
        .expect("Calculating closest past monday failed")
}

// fn main() -> eframe::Result {
//     env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
//
//     let options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default().with_inner_size([760.0, 800.0]),
//         ..Default::default()
//     };
//
//     eframe::run_native(
//         TITLE,
//         options,
//         Box::new(|_cc| Ok(Box::<MenuPdfApp>::default())),
//     )
// }
//
// #[derive(Default)]
// struct MenuPdfApp {
//     show_confirmation_dialog: bool,
//     allowed_to_close: bool,
// }

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
    let mut week_data = week::load_week(&datum);

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
                    week::save_if_needed(&week_data, &datum);
                    datum = datum
                        .checked_sub_days(DAYS_IN_WEEK)
                        .to_owned()
                        .expect("Subtracting 7 days failed.");
                    selected_monday = datum;
                    week_data = week::load_week(&datum);
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
                        week::save_if_needed(&week_data, &selected_monday);
                        let date_string = datum.format(INI_DATE_FORMAT).to_string();
                        let date_str = date_string.as_str();
                        let ini_path = Path::new(INI_FILE_PATH);
                        let conf: Ini = match ini_path.exists() {
                            true => Ini::load_from_file(ini_path).expect("Error loading ini file"),
                            false => Ini::new(),
                        };

                        let mut week_string: WeekData = week::create_empty_week();
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
                    week::save_if_needed(&week_data, &datum);
                    datum = datum
                        .checked_add_days(DAYS_IN_WEEK)
                        .to_owned()
                        .expect("Adding 7 days failed.");
                    selected_monday = datum;
                    week_data = week::load_week(&datum);
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
                    week_data = week::load_week(&datum);
                }

                if ui.button("Save").clicked() {
                    week::save_if_needed(&week_data, &datum);
                }
            });
        });

        // if ctx.input(|i| i.viewport().close_requested()) {
        //     if self.allowed_to_close {
        //         // do nothing - we will close
        //     } else {
        //         ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        //         self.show_confirmation_dialog = true;
        //     }
        // }
        //
        // if self.show_confirmation_dialog {
        //     egui::Window::new("Do you want to quit?")
        //         .pivot(Align2::CENTER_CENTER) // .curent_pos(pos)
        //         .collapsible(false)
        //         .resizable(false)
        //         .show(ctx, |ui| {
        //             ui.horizontal(|ui| {
        //                 if ui.button("No").clicked() {
        //                     self.show_confirmation_dialog = false;
        //                     self.allowed_to_close = false;
        //                 }
        //
        //                 if ui.button("Yes").clicked() {
        //                     self.show_confirmation_dialog = false;
        //                     self.allowed_to_close = true;
        //                     ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //         });
        // }
    })
}
