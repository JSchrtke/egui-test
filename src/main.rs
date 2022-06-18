#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{
    plot::{Line, Plot, Values},
    pos2, Color32, Frame, Rect, Rounding,
};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "A green window with a pink diagonal line",
        options,
        Box::new(|_a| Box::new(App::new(vec![0.0, 1.0, 2.0]))),
    );
}

struct App {
    data: Vec<f32>,
    pink_box: bool,
}

impl App {
    fn new(data: Vec<f32>) -> Self {
        Self {
            data,
            pink_box: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::from_rgb(255, 255, 255)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("some text");
                });

                if ui.input().key_pressed(egui::Key::Space) {
                    self.pink_box = !self.pink_box;
                }

                let values = Values::from_ys_f32(&self.data[..]);
                let line = Line::new(values);
                let plot = Plot::new("test_plot");
                plot.show(ui, |plotter| {
                    plotter.line(line);
                });

                if self.pink_box {
                    let window_size = ui.min_size();
                    let window_width = window_size.x;
                    let window_height = window_size.y;
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            pos2(0.0, 0.0),
                            egui::Vec2::new(window_width, window_height),
                        ),
                        Rounding::none(),
                        Color32::from_rgb(255, 0, 255),
                    );
                }
            });
    }
}
