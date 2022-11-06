// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::OsString;

use eframe::egui::{self, Key};
use exitcode::ExitCode;
use std::fmt::Write;

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Christian".to_owned(),
            age: 32,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_multiline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            if ui.input().modifiers.ctrl && ui.input().key_pressed(Key::Enter) {
                writeln!(self.name).unwrap();
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

pub fn run(_args: Vec<OsString>) -> ExitCode {
    let options = eframe::NativeOptions::default();
    eframe::run_native("rift", options, Box::new(|_cc| Box::new(MyApp::default())));
    exitcode::OK
}
