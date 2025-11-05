#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's unfinished ...

use chrono::{Datelike, Days, Local, NaiveDate};
use eframe::egui::{Align2, Visuals};
use eframe::egui::{Context, TextEdit};
use eframe::{Frame, egui};
use egui_extras::DatePickerButton;
use log::info;

use eframe::egui::FontFamily::Proportional;
use eframe::egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use ini::Ini;
use std::path::Path;
use week::WeekData;

mod week;

const EDIT_WIDTH: f32 = 200.0;
const TITLE: &str = "Menu → PDF";
const TIME_LONG: [&str; 2] = ["Mittag", "Abend"];
const TIME_SHORT: [&str; 2] = ["mi", "ab"];
// const DAY_LONG: [&str; 7] = [
const DAY_LONG: [&str; 2] = [
    "Montag",
    "Dienstag",
    // "Mittwoch",
    // "Donnerstag",
    // "Freitag",
    // "Samstag",
    // "Sonntag",
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

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let mut app = MenuPdfApp::new();

    // // Compute window size.
    // let ctx = egui::Context::default();
    //
    // let _ = ctx.run(egui::RawInput::default(), |_| {});
    // app.render(&ctx);
    // let size = ctx.used_size();

    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default()
    //         .with_resizable(false)
    //         .with_inner_size(size),
    //     ..Default::default()
    // };

    eframe::run_native(
        TITLE,
        // options,
        eframe::NativeOptions::default(),
        // Box::new(|cc| Ok(Box::new(MenuPdfApp::new(cc)))),
        Box::new(|_cc| Ok(Box::new(app))),
    )
}

#[derive(Default)]
struct MenuPdfApp {
    selected_monday: NaiveDate,
    week_data: WeekData,
    _first_render: bool,
}

impl MenuPdfApp {
    // pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    pub fn new() -> Self {
        let mut datum = Local::now().date_naive();
        datum = get_closest_last_monday(&mut datum);

        Self {
            selected_monday: datum,
            week_data: week::load_week(&datum),
            _first_render: true,
        }
    }
}

// TODO: Size window to fit content -> https://github.com/emilk/egui/discussions/2858
//       Origin of 2858   -> https://github.com/emilk/egui/discussions/4329
//       Alternative?     -> https://github.com/emilk/egui/discussions/465
//       Similar problem? -> https://github.com/emilk/egui/discussions/949

impl MenuPdfApp {
    fn pre_render(&mut self, ctx: &eframe::egui::Context) {
        // // From: https://github.com/emilk/egui/discussions/1627
        // ctx.set_visuals(Visuals::light());
        // ctx.set_pixels_per_point(2.0);

        egui::Window::new("pre_render")
            .title_bar(false)
            .fixed_pos((0.0, 0.0))
            .show(ctx, |ui| {
                self.render(ui);
                // ui.allocate_space(ui.available_size());
            });
    }
    fn render(&mut self, ui: &mut egui::Ui) {
    // fn render(&mut self, ctx: &egui::Ui) {
        // egui::CentralPanel::default().show(ctx, |ui| {
        // egui::Area::new(egui::Id::new("new-area"))
        //     .fixed_pos((0.0, 0.0))
        //     .show(ctx, |ui| {

                // let mut style = (*ctx.style()).clone();
            // style.text_styles = [
            //     (Heading, FontId::new(30.0, Proportional)),
            //     (Body, FontId::new(18.0, Proportional)),
            //     (Monospace, FontId::new(16.0, Proportional)),
            //     (Button, FontId::new(16.0, Proportional)),
            //     (Small, FontId::new(10.0, Proportional)),
            // ]
            // .into();
            // ctx.set_style(style);

        let mut datum = self.selected_monday;
        ui.horizontal(|ui| {
            ui.label("Datum: ");
            let left_button = ui.button("<");
            let datepicker_button =
                ui.add(DatePickerButton::new(&mut datum).format(UI_DATE_FORMAT));
            let right_button = ui.button(">");

            if left_button.clicked() {
                week::save_if_needed(&self.week_data, &datum);
                datum = datum
                    .checked_sub_days(DAYS_IN_WEEK)
                    .to_owned()
                    .expect("Subtracting 7 days failed.");
                self.selected_monday = datum;
                self.week_data = week::load_week(&datum);
            };
            if datepicker_button.changed() {
                datum = get_closest_last_monday(&mut datum).to_owned();
                if self.selected_monday == datum {
                    info!(
                        "Same date. selected_monday: {}, datum: {}",
                        self.selected_monday, datum
                    );
                } else {
                    info!(
                        "Different date. selected_monday: {}, datum: {}",
                        self.selected_monday, datum
                    );
                    week::save_if_needed(&self.week_data, &self.selected_monday);
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
                    self.week_data = week_string;
                }
                self.selected_monday = datum;
            }
            if right_button.clicked() {
                week::save_if_needed(&self.week_data, &datum);
                datum = datum
                    .checked_add_days(DAYS_IN_WEEK)
                    .to_owned()
                    .expect("Adding 7 days failed.");
                self.selected_monday = datum;
                self.week_data = week::load_week(&datum);
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
                    TextEdit::multiline(&mut self.week_data[i][0])
                        .min_size([EDIT_WIDTH, 1.0].into()),
                );
                ui.add(
                    TextEdit::multiline(&mut self.week_data[i][1])
                        .min_size([EDIT_WIDTH, 1.0].into()),
                );
                ui.end_row();
            }

            ui.label("");
            if ui.button("Load").clicked() {
                self.week_data = week::load_week(&datum);
            }

            if ui.button("Save").clicked() {
                week::save_if_needed(&self.week_data, &datum);
            }
        });

            // From: https://github.com/emilk/egui/blob/main/examples/confirm_exit/src/main.rs
            // if ctx.input(|i| i.viewport().close_requested()) {
            //     week::save_if_needed(&self.week_data, &datum);
            // }
        // });
    }
}

impl eframe::App for MenuPdfApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // From: https://github.com/emilk/egui/discussions/1627
        // ctx.set_visuals(Visuals::light());
        // ctx.set_pixels_per_point(2.0);
        // self.render(ctx)
        if self._first_render {
            self.pre_render(ctx);
            self._first_render = false;
            let window_size = ctx.used_size();
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(window_size));
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
            // egui::Window::new("pre_render")
            //     .title_bar(false)
            //     .fixed_pos((0.0, 0.0))
            //     .show(ctx, |ui| {
                self.render(ui);
            });
        }
    }
}
