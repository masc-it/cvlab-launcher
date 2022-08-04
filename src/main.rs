#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod config;

use config::{Config, load_config, save_config};
use eframe::egui;
use egui::{Color32, Ui};
use egui::{FontFamily, FontId, TextStyle};
use std::process::Command;

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(30.0, Proportional)),
        (TextStyle::Body, FontId::new(22.0, Proportional)),
        (TextStyle::Monospace, FontId::new(18.0, Proportional)),
        (TextStyle::Button, FontId::new(18.0, Proportional)),
        (TextStyle::Small, FontId::new(14.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

fn main() {
    let options = eframe::NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        decorated: false,
        // To have rounded corners we need transparency:
        transparent: true,
        min_window_size: Some(egui::vec2(320.0, 320.0)),
        resizable: true,
        ..Default::default()
    };

    eframe::run_native(
        "CVLAB Launcher",
        options,
        Box::new(|_cc| {

            _cc.egui_ctx.set_visuals(egui::style::Visuals::dark());
            //egui::style::Margin::same(12.0);
            configure_text_styles(&_cc.egui_ctx);
            /* let mut style: egui::Style = (*_cc.egui_ctx.style()).clone();
            style.spacing.window_margin = egui::style::Margin::same(24.0);
            _cc.egui_ctx.set_style(style); */

            Box::new(MyApp::default())
        }),
    );


}


struct MyApp {
    
    od_model: bool,
    new_collection: String,
    err_msg: String,
    config: Config
}

impl Default for MyApp {
    
    
    fn default() -> Self {
        Self {
            od_model: true,
            err_msg: "".to_string(),
            new_collection: "".to_string(),
            config: load_config().unwrap()
        }
    }
}

fn separator(ui: &mut Ui) {
    ui.add_space(6.0);
    ui.separator();
    ui.add_space(6.0);
}

impl eframe::App for MyApp {

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners,
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        custom_window_frame(ctx, frame, "CVLAB Launcher", |ui| {
            
            ui.add_space(6.0);
            
            ui.vertical_centered(|ui| {
                ui.heading("Collections");
            });
            
            
            ui.vertical_centered(|ui| {
                
                if !self.err_msg.eq("") {
                    ui.colored_label( Color32::from_rgb(255, 0, 0), &mut self.err_msg);
                    
                }
                
                
                ui.horizontal(|ui| {

                    ui.add_space(12.0);
                    
                    if ui.add_sized(ui.available_size() / egui::Vec2 { x: 1.5, y:1.0 },
                        egui::TextEdit::singleline(&mut self.new_collection)).clicked() {
                               
                            self.err_msg = "".to_string();
                            
                        }
                    
                    if ui.button("Add Collection").clicked(){
                        self.err_msg = String::from("");
                        if !std::path::Path::new(&self.new_collection).exists() {
                            self.err_msg = "Collection does not exist!".to_string();

                        } else {
                            self.config.collections.push(self.new_collection.to_owned());
                            self.new_collection = "".to_string();
                        }
                       
                    }
                });
                
                ui.add_space(6.0);
                for (i, collection) in self.config.collections.to_owned().iter().enumerate() {
                    
                        ui.horizontal(|ui| {
                            ui.add_space(12.0);
                            
                            ui.add_sized(ui.available_size() / egui::Vec2 { x: 1.5, y:1.0 },
                            egui::Label::new(collection));
                            if ui.button("x").clicked() {
                                self.config.collections.remove(i);
                            }
                        });
                    
                }
               
            });

            separator(ui);
            ui.vertical(|ui| {
                
                ui.vertical_centered(|ui| {
                    ui.heading("Object detection model");
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    ui.label("Path");
                    ui.add_sized(ui.available_size() / egui::Vec2 { x: 1.5, y:1.0 },
                    egui::TextEdit::singleline(&mut self.config.od_model));
                });
                
                
            });

            separator(ui);

            ui.horizontal_top(|ui| {

                ui.add_space(12.0);
                if ui.button("Save").clicked() {
               
                    save_config(&self.config).unwrap();
    
                }
                if ui.button("Run CVLAB").clicked() {
               
                    if cfg!(target_os = "windows") {
                        Command::new("cmd")
                                .args(["/C", "D:\\Projects\\python\\yolo-lab\\CVLAB.bat"])
                                .spawn().unwrap();
                    } else {
                        Command::new("sh")
                                .arg("-c")
                                .arg("echo hello")
                                .spawn().unwrap();
                    };
    
                }
            });
            
     
        });
        
    }
}

fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;
    let text_color = ctx.style().visuals.text_color();

    // Height of the title bar
    let height = 28.0;

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            // Paint the frame:
            painter.rect(
                rect.shrink(1.0),
                10.0,
                ctx.style().visuals.window_fill(),
                Stroke::new(1.0, text_color),
            );

            // Paint the title:
            painter.text(
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(height - 2.0),
                text_color,
            );

            // Paint the line under the title:
            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ],
                Stroke::new(1.0, text_color),
            );

            // Add the close button:
            let close_response = ui.put(
                Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
                Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
            );
            if close_response.clicked() {
                frame.quit();
            }

            // Interact with the title bar (drag to move window):
            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };
            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                frame.drag_window();
            }

            // Add the contents:
            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
}