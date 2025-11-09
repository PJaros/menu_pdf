#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's unfinished ...

use chrono::{Datelike, Days, Local, NaiveDate};
use eframe::egui;
use eframe::egui::TextEdit;
use eframe::egui::{Vec2, Visuals};
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

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let app = MenuPdfApp::new();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(TITLE, options, Box::new(|_cc| Ok(Box::new(app))))
}

#[derive(Default)]
enum Stage {
    #[default]
    PreRender,
    FirstRender,
    FirstResize,
    Initialized,
}

#[derive(Default)]
struct MenuPdfApp {
    _render_stage: Stage,
    _pre_render_cycles: isize,
    _initial_size: Option<Vec2>,
    selected_monday: NaiveDate,
    week_data: WeekData,
}

impl MenuPdfApp {
    pub fn new() -> Self {
        let mut datum = Local::now().date_naive();
        datum = get_closest_last_monday(&mut datum);

        Self {
            _render_stage: Stage::PreRender,
            _pre_render_cycles: 2,
            _initial_size: None,
            selected_monday: datum,
            week_data: week::load_week(&datum),
        }
    }
}

impl MenuPdfApp {
    fn pre_render(&mut self, ctx: &eframe::egui::Context) {
        egui::Window::new("pre_render")
            .title_bar(false)
            .fixed_pos((0.0, 0.0))
            .show(ctx, |ui| {
                self.render(ui);
            });
    }
    fn render(&mut self, ui: &mut egui::Ui) {
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
    }
}

impl eframe::App for MenuPdfApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        match self._render_stage {
            Stage::PreRender => {
                self.pre_render(ctx);
                self._initial_size = Some(ctx.used_size());
                self._pre_render_cycles -= 1;
                if self._pre_render_cycles <= 0 {
                    self._render_stage = Stage::FirstRender;
                }
            }
            Stage::FirstRender => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.render(ui);
                });
                self._render_stage = Stage::FirstResize;
            }
            Stage::FirstResize => {
                if let Some(size) = self._initial_size {
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
                    self._render_stage = Stage::Initialized;
                }
            }
            Stage::Initialized => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.render(ui);
                });
            }
        }

        // From: https://github.com/emilk/egui/blob/main/examples/confirm_exit/src/main.rs
        if ctx.input(|i| i.viewport().close_requested()) {
            week::save_if_needed(&self.week_data, &self.selected_monday);
        }
    }
}
