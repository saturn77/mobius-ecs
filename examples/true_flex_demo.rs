use bevy_ecs::prelude::*;
use eframe::egui;
use mobius_ecs::simple_flex_dock::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("TRUE Flexbox Behavior Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "true_flex_demo",
        options,
        Box::new(|_cc| Ok(Box::new(TrueFlexDemo::new()))),
    )
}

struct TrueFlexDemo {
    world: World,
    flex_ratios: [f32; 3],
    show_flex_values: bool,
}

impl TrueFlexDemo {
    fn new() -> Self {
        let mut world = World::new();
        setup_simple_flex_dock(&mut world);
        
        Self { 
            world,
            flex_ratios: [1.0, 2.0, 1.0], // Default flex ratios
            show_flex_values: true,
        }
    }
}

impl eframe::App for TrueFlexDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Create the flex demonstration UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎯 TRUE Flexbox Behavior Demonstration");
            ui.separator();
            
            // Controls for flex ratios
            ui.horizontal(|ui| {
                ui.label("Flex Ratios:");
                ui.add(egui::Slider::new(&mut self.flex_ratios[0], 0.0..=5.0).text("Item 1"));
                ui.add(egui::Slider::new(&mut self.flex_ratios[1], 0.0..=5.0).text("Item 2"));
                ui.add(egui::Slider::new(&mut self.flex_ratios[2], 0.0..=5.0).text("Item 3"));
                ui.checkbox(&mut self.show_flex_values, "Show values");
            });
            
            ui.separator();
            
            // Flex container demonstration
            ui.group(|ui| {
                ui.label("Horizontal Flex Container (items grow proportionally):");
                
                let available_width = ui.available_width() - 32.0; // Account for padding
                let total_flex = self.flex_ratios[0] + self.flex_ratios[1] + self.flex_ratios[2];
                
                if total_flex > 0.0 {
                    ui.horizontal(|ui| {
                        // Item 1
                        let width1 = (self.flex_ratios[0] / total_flex) * available_width;
                        let text1 = if self.show_flex_values {
                            format!("Flex: {:.1}\nWidth: {:.0}px", self.flex_ratios[0], width1)
                        } else {
                            "Item 1".to_string()
                        };
                        
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(width1, 80.0),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(255, 100, 100);
                                let response = ui.allocate_response(ui.available_size(), egui::Sense::hover());
                                ui.painter().rect_filled(response.rect, 4.0, egui::Color32::from_rgb(255, 100, 100));
                                ui.painter().text(
                                    response.rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    &text1,
                                    egui::FontId::proportional(12.0),
                                    egui::Color32::BLACK,
                                );
                            }
                        );
                        
                        // Item 2
                        let width2 = (self.flex_ratios[1] / total_flex) * available_width;
                        let text2 = if self.show_flex_values {
                            format!("Flex: {:.1}\nWidth: {:.0}px", self.flex_ratios[1], width2)
                        } else {
                            "Item 2".to_string()
                        };
                        
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(width2, 80.0),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                let response = ui.allocate_response(ui.available_size(), egui::Sense::hover());
                                ui.painter().rect_filled(response.rect, 4.0, egui::Color32::from_rgb(100, 255, 100));
                                ui.painter().text(
                                    response.rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    &text2,
                                    egui::FontId::proportional(12.0),
                                    egui::Color32::BLACK,
                                );
                            }
                        );
                        
                        // Item 3
                        let width3 = (self.flex_ratios[2] / total_flex) * available_width;
                        let text3 = if self.show_flex_values {
                            format!("Flex: {:.1}\nWidth: {:.0}px", self.flex_ratios[2], width3)
                        } else {
                            "Item 3".to_string()
                        };
                        
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(width3, 80.0),
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                let response = ui.allocate_response(ui.available_size(), egui::Sense::hover());
                                ui.painter().rect_filled(response.rect, 4.0, egui::Color32::from_rgb(100, 100, 255));
                                ui.painter().text(
                                    response.rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    &text3,
                                    egui::FontId::proportional(12.0),
                                    egui::Color32::WHITE,
                                );
                            }
                        );
                    });
                } else {
                    ui.label("Set flex values > 0 to see flex behavior");
                }
            });
            
            ui.add_space(20.0);
            
            // Flex examples
            ui.group(|ui| {
                ui.label("Common Flex Patterns:");
                
                if ui.button("Equal Distribution (1:1:1)").clicked() {
                    self.flex_ratios = [1.0, 1.0, 1.0];
                }
                if ui.button("Center Emphasis (1:3:1)").clicked() {
                    self.flex_ratios = [1.0, 3.0, 1.0];
                }
                if ui.button("Left Heavy (3:1:1)").clicked() {
                    self.flex_ratios = [3.0, 1.0, 1.0];
                }
                if ui.button("Golden Ratio (1:1.618:1)").clicked() {
                    self.flex_ratios = [1.0, 1.618, 1.0];
                }
            });
            
            ui.add_space(20.0);
            
            // Vertical flex demonstration
            ui.group(|ui| {
                ui.label("Vertical Flex Container:");
                
                let available_height = 200.0;
                let total_flex = self.flex_ratios[0] + self.flex_ratios[1] + self.flex_ratios[2];
                
                if total_flex > 0.0 {
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(ui.available_width(), available_height),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            // Vertical Item 1
                            let height1 = (self.flex_ratios[0] / total_flex) * available_height;
                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(ui.available_width(), height1),
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    let response = ui.allocate_response(ui.available_size(), egui::Sense::hover());
                                    ui.painter().rect_filled(response.rect, 4.0, egui::Color32::from_rgb(255, 150, 150));
                                    ui.painter().text(
                                        response.rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        &format!("Vertical Item 1 (flex: {:.1})", self.flex_ratios[0]),
                                        egui::FontId::proportional(12.0),
                                        egui::Color32::BLACK,
                                    );
                                }
                            );
                            
                            // Vertical Item 2
                            let height2 = (self.flex_ratios[1] / total_flex) * available_height;
                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(ui.available_width(), height2),
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    let response = ui.allocate_response(ui.available_size(), egui::Sense::hover());
                                    ui.painter().rect_filled(response.rect, 4.0, egui::Color32::from_rgb(150, 255, 150));
                                    ui.painter().text(
                                        response.rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        &format!("Vertical Item 2 (flex: {:.1})", self.flex_ratios[1]),
                                        egui::FontId::proportional(12.0),
                                        egui::Color32::BLACK,
                                    );
                                }
                            );
                            
                            // Vertical Item 3
                            let height3 = (self.flex_ratios[2] / total_flex) * available_height;
                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(ui.available_width(), height3),
                                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    let response = ui.allocate_response(ui.available_size(), egui::Sense::hover());
                                    ui.painter().rect_filled(response.rect, 4.0, egui::Color32::from_rgb(150, 150, 255));
                                    ui.painter().text(
                                        response.rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        &format!("Vertical Item 3 (flex: {:.1})", self.flex_ratios[2]),
                                        egui::FontId::proportional(12.0),
                                        egui::Color32::WHITE,
                                    );
                                }
                            );
                        }
                    );
                }
            });
            
            ui.add_space(20.0);
            
            // Explanation
            ui.group(|ui| {
                ui.label("📚 How Flexbox Works:");
                ui.label("• Flex items grow proportionally based on their flex-grow values");
                ui.label("• If item A has flex: 2 and item B has flex: 1, A will be twice as wide");
                ui.label("• Total available space is divided by the sum of all flex values");
                ui.label("• Each item gets: (its_flex_value / total_flex) × available_space");
                ui.label("• Resize the window to see how items adapt to available space!");
            });
        });
        
        ctx.request_repaint();
    }
}