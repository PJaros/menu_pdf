#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chrono::Local;
use eframe::egui;
use eframe::egui::TextEdit;
use eframe::egui::Visuals;
use egui_extras::DatePickerButton;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 480.0]),
        ..Default::default()
    };

    const EDIT_WIDTH: f32 = 200.0;
    const TITLE: &str = "Menu → PDF";

    let mut datum = Local::now().date_naive();
    let mut montag_mittag = "".to_owned();
    let mut montag_abend = "".to_owned();
    let mut dienstag_mittag = "".to_owned();
    let mut dienstag_abend = "".to_owned();

    eframe::run_simple_native(TITLE, options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // From: https://github.com/emilk/egui/discussions/1627
            ctx.set_visuals(Visuals::light());
            ctx.set_pixels_per_point(1.5);

            ui.horizontal(|ui| {
                let datum_label = ui.label("Datum: ");
                ui.add(DatePickerButton::new(&mut datum).format("%e. %b %Y"))
                    .labelled_by(datum_label.id);
            });
            // TODO: Add two buttons left and right with arrows to decrement and increment by a week
            // TODO: Search why DatePickerButton is darker than TextEdit::multiline
            // TODO: Add https://crates.io/crates/pretty_ini as a dependency and use it to save and restore the state
            // TODO: Replace variables with array for each day
            egui::Grid::new("grid_id").show(ui, |ui| {
                ui.label("");
                ui.label("Mittag");
                ui.label("Abend");
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
