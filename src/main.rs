#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::ops::RangeInclusive;

use eframe::egui;
use egui::{
    plot::{Line, Plot, Values},
    Color32, Frame, RichText, Rounding, Stroke,
};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "A window with multipel panels",
        options,
        Box::new(|_a| {
            Box::new(App::new(
                vec!["01.01.22".into(), "02.02.22".into(), "03.03.22".into()],
                vec![10.0, 20.0, 30.0],
            ))
        }),
    );
}

struct App {
    dates: Vec<String>,
    values: Vec<f32>,
}

impl App {
    fn new(dates: Vec<String>, values: Vec<f32>) -> Self {
        Self { dates, values }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        show_left_panel(ctx, &self.dates, &self.values);
        show_center_panel(ctx, self.dates.clone(), &self.values);
    }
}

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);

fn show_left_panel(ctx: &egui::Context, dates: &[String], values: &[f32]) {
    egui::SidePanel::new(egui::panel::Side::Left, "left_panel")
        .frame(Frame::none().fill(WHITE))
        .show(ctx, |ui| {
            fill_bg(ui, WHITE);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("left panel").size(24.0).color(BLACK));
                ui.columns(2, |columns| {
                    dates.iter().enumerate().for_each(|(i, date)| {
                        columns[0].label(date);
                        columns[1].label(values[i].to_string());
                    })
                });
            });
        });
}

fn show_center_panel(ctx: &egui::Context, dates: Vec<String>, values: &[f32]) {
    egui::CentralPanel::default()
        .frame(Frame::none().fill(WHITE))
        .show(ctx, |ui| {
            let x_fmt = move |x_value, _range: &RangeInclusive<f64>| {
                if is_almost_int(x_value) && 0.0 <= x_value && x_value <= (dates.len() - 1) as f64 {
                    let i = x_value as usize;
                    dates[i].clone()
                } else {
                    String::new()
                }
            };

            let line = Line::new(Values::from_ys_f32(values));
            Plot::new("plot")
                .x_axis_formatter(x_fmt)
                .show(ui, |plot_ui| plot_ui.line(line));
        });
}

fn is_almost_int(value: f64) -> bool {
    value.fract() <= 10e-6
}

fn fill_bg(ui: &mut egui::Ui, fill_color: egui::Color32) {
    let ui_rect = ui.max_rect();
    ui.painter().rect(
        ui_rect,
        Rounding::none(),
        fill_color,
        Stroke::new(1.0, fill_color),
    );
}
