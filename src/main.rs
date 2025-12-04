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

use std::fs;
use std::time::{Duration, Instant};
use typst::foundations::{Dict, IntoValue};
use typst_as_lib::{TypstEngine, TypstTemplateMainFile};

use clap::{Parser, crate_version};

static TEMPLATE_MAIN_FILE: &str = include_str!("../res/wochenmenu.md");
static FONT_H: &[u8] = include_bytes!("../res/Helvetica.ttf");
static FONT_H_B: &[u8] = include_bytes!("../res/Helvetica-Bold.ttf");
static IMAGE: &[u8] = include_bytes!("../res/Titel.png");
static OUTPUT: &str = "./output.pdf";

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
const MONTH: [&str; 12] = [
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];
const DAY_SHORT: [&str; 7] = ["mo", "di", "mi", "do", "fr", "sa", "so"];
const INI_FILE_PATH: &str = "menu.ini";
const UI_DATE_FORMAT: &str = "%e. %b %Y";
const INI_DATE_FORMAT: &str = "%Y-%m-%d";
const DAYS_IN_WEEK: Days = Days::new(7);

enum Stage {
    PreRender(isize),
    FirstRender(Vec2),
    FirstResize(Vec2),
    Initialized(Vec2),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1.0)]
    zoom: f32,
    #[arg(short, long, default_value_t = false)]
    demo_pdf: bool,
}

struct MenuPdfApp {
    _render_stage: Stage,
    selected_monday: NaiveDate,
    zoom: Option<f32>,
    week_data: WeekData,
    last_save: Instant,
    engine: TypstEngine<TypstTemplateMainFile>,
}

fn get_closest_last_monday(datum: &mut NaiveDate) -> NaiveDate {
    let day: u64 = (datum.weekday().number_from_monday() - 1).into();
    datum
        .checked_sub_days(Days::new(day))
        .to_owned()
        .expect("Calculating closest past monday failed")
}

fn main() {
    let args = Args::parse();
    let engine: TypstEngine<TypstTemplateMainFile> = TypstEngine::builder()
        .with_static_file_resolver([("./Titel.png", IMAGE)])
        .main_file(TEMPLATE_MAIN_FILE)
        .fonts([FONT_H, FONT_H_B])
        .build();

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let app = MenuPdfApp::new(args.zoom, engine);
    match args.demo_pdf {
        true => {
            let demo_week_data = week::load_static_demo_week();
            write_pdf(demo_week_data, &app.selected_monday, &app.engine);
            open::that(OUTPUT).expect("Error opening PDF");
        }
        false => {
            let options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default().with_resizable(false),
                ..Default::default()
            };

            let version = crate_version!();
            let _ = eframe::run_native(
                format!("{TITLE} - v{version}").as_str(),
                options,
                Box::new(|_cc| Ok(Box::new(app))),
            );
        }
    }
}

impl MenuPdfApp {
    pub fn new(zoom: f32, engine: TypstEngine<TypstTemplateMainFile>) -> Self {
        let mut datum = Local::now().date_naive();
        datum = get_closest_last_monday(&mut datum);

        Self {
            _render_stage: Stage::PreRender(2_isize),
            selected_monday: datum,
            zoom: match zoom {
                1.0 => None,
                _ => Some(zoom),
            },
            week_data: week::load_week(&datum),
            last_save: Instant::now(),
            engine,
        }
    }
}

impl MenuPdfApp {
    fn pre_render(&mut self, ctx: &egui::Context) {
        egui::Window::new("pre_render")
            .title_bar(false)
            .fixed_pos((0.0, 0.0))
            .show(ctx, |ui| {
                self.render(ui);
            });
    }
    fn render(&mut self, ui: &mut egui::Ui) {
        let mut datum = self.selected_monday;
        egui::Grid::new("grid_id").show(ui, |ui| {
            ui.label("Datum: ");
            ui.horizontal(|ui| {
                let left_button = ui.button("<");
                let datepicker_button =
                    ui.add(DatePickerButton::new(&mut datum).format(UI_DATE_FORMAT));
                let right_button = ui.button(">");

                if left_button.clicked() {
                    self.save_if_needed(&datum);
                    self.last_save = Instant::now();
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
                        self.save_if_needed(&self.selected_monday.clone());
                        self.last_save = Instant::now();
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
                    self.save_if_needed(&datum);
                    datum = datum
                        .checked_add_days(DAYS_IN_WEEK)
                        .to_owned()
                        .expect("Adding 7 days failed.");
                    self.selected_monday = datum;
                    self.week_data = week::load_week(&datum);
                }
            });
            ui.end_row();

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
            ui.vertical(|ui| {
                ui.add_space(10.0);
                if ui.button("Drucken").clicked() {
                    self.save_if_needed(&datum);
                    let next_week = datum
                        .checked_add_days(DAYS_IN_WEEK)
                        .to_owned()
                        .expect("Adding 7 days failed.");
                    let next_week_data = week::load_week(&next_week);
                    write_pdf(
                        [self.week_data.clone(), next_week_data],
                        &datum,
                        &self.engine,
                    );
                    open::that(OUTPUT).expect("Error opening main PDF");
                }
            });
        });
    }

    fn check_if_time_passed(&mut self) {
        if Instant::now() - self.last_save > Duration::from_secs(5) {
            info!("Time has passed. Checking if save is needed");
            self.last_save = Instant::now();
            self.save_if_needed(&self.selected_monday.clone());
        }
    }

    fn save_if_needed(&mut self, datum: &NaiveDate) {
        let date_string = datum.format(INI_DATE_FORMAT).to_string();
        if !week::is_equal_to_saved(&self.week_data, datum) {
            self.last_save = Instant::now();
            week::save_week(&self.week_data, date_string.as_str());
        }
    }
}

fn write_pdf(
    week_data_arr: [WeekData; 2],
    datum: &NaiveDate,
    engine: &TypstEngine<TypstTemplateMainFile>,
) {
    let date = *datum;
    let mut dict = Dict::new();
    for count in 0..=7_usize {
        // count 0,1,2,3,4,5,6,7
        let cur_date = date + Days::new(count as u64);
        let day_of_week = count % 7; // convert to 0,1,2,3,4,5,6,0
        let day = DAY_SHORT[day_of_week];
        let add_on = if count >= 7 { "_2" } else { "" };
        dict.insert(
            format!("{day}_day{add_on}").into(),
            DAY_LONG[day_of_week].into_value(),
        );
        let day_of_month = cur_date.day();
        let month = date.month0();
        let month_str = MONTH[month as usize];
        let datum_str = format!("{day_of_month}. {month_str}");
        dict.insert(
            format!("{day}_date{add_on}").into(),
            datum_str.to_owned().into_value(),
        );
        for (x, time) in TIME_SHORT.iter().enumerate() {
            let key = format!("{day}_{time}{add_on}");
            let arr_count = count / 7;
            dict.insert(
                key.into(),
                week_data_arr[arr_count][day_of_week][x].trim().into_value(),
            );
        }
    }

    // Create document
    let doc = engine
        .compile_with_input(dict)
        .output
        .expect("typst::compile() returned an error!");

    // Render pdf
    let pdf = typst_pdf::pdf(&doc, &Default::default()).expect("Could not render pdf.");
    fs::write(OUTPUT, pdf).expect("Could not write pdf.");
}

impl eframe::App for MenuPdfApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light());
        if let Some(x) = self.zoom {
            ctx.set_zoom_factor(x)
        }
        match self._render_stage {
            Stage::PreRender(mut pre_render_cycle) => {
                self.pre_render(ctx);
                pre_render_cycle -= 1;
                if pre_render_cycle > 0 {
                    self._render_stage = Stage::PreRender(pre_render_cycle)
                } else {
                    self._render_stage = Stage::FirstRender(ctx.used_size());
                }
            }
            Stage::FirstRender(size) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.render(ui);
                });
                self._render_stage = Stage::FirstResize(size)
            }
            Stage::FirstResize(size) => {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
                self._render_stage = Stage::Initialized(size)
            }
            Stage::Initialized(_size) => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.render(ui);
                    self.check_if_time_passed()
                });
            }
        };

        // From: https://github.com/emilk/egui/blob/main/examples/confirm_exit/src/main.rs
        if ctx.input(|i| i.viewport().close_requested()) {
            self.save_if_needed(&self.selected_monday.clone());
        }
    }
}
