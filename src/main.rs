#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{pos2, Color32, Frame, Stroke};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "A green window with a pink diagonal line",
        options,
        Box::new(|_a| Box::new(App {})),
    );
}

struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::from_rgb(0, 255, 0)))
            .show(ctx, |ui| {
                let inner_window_size = ui.min_size();
                let width = inner_window_size.x;
                let height = inner_window_size.y;

                let line_start = pos2(0.0, 0.0);
                let line_end = pos2(width, height);
                let line_stroke = Stroke::new(2.0, Color32::from_rgb(255, 0, 255));
                ui.painter()
                    .line_segment([line_start, line_end], line_stroke)
            });
    }
}
