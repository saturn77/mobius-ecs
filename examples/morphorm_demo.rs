use eframe::egui;
use mobius_ecs::morphorm_bridge::MorphormLayoutBridge;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Morphorm Layout Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "morphorm_demo",
        options,
        Box::new(|_cc| Ok(Box::new(MorphormDemo::default()))),
    )
}

struct MorphormDemo {
    /// Header text
    header_text: String,
    /// Content text
    content_text: String,
    /// Show debug info
    show_debug: bool,
}

impl Default for MorphormDemo {
    fn default() -> Self {
        Self {
            header_text: "Header Panel - Fixed Height (60px)".to_string(),
            content_text: "Content Panel - Stretches to fill remaining space\n\nTry resizing the window to see responsive behavior!".to_string(),
            show_debug: true,
        }
    }
}

impl eframe::App for MorphormDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Get or create the morphorm layout bridge from egui memory
        let mut bridge = MorphormLayoutBridge::get_or_create(ctx);

        // Clear previous layout
        bridge.clear();

        // Get available size
        let available_rect = ctx.screen_rect();
        let available_size = available_rect.size();

        // Set up root container
        bridge.setup_root(available_size);

        // Add header with fixed height
        let header_height = 60.0;
        let header_id = bridge.add_header(header_height);

        // Add content that stretches
        let content_id = bridge.add_content();

        // Compute the layout
        if let Err(e) = bridge.compute_layout(available_size) {
            eprintln!("Layout computation error: {}", e);
        }

        // Render using the computed layout
        if let Some(header_bounds) = bridge.get_bounds(header_id) {
            // Draw header panel
            egui::Area::new(egui::Id::new("header_area"))
                .fixed_pos(header_bounds.min)
                .show(ctx, |ui| {
                    ui.set_max_size(header_bounds.size());
                    
                    // Header frame
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(50, 50, 50))
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            let inner_size = header_bounds.size() - egui::vec2(20.0, 20.0);
                            if inner_size.x > 0.0 && inner_size.y > 0.0 {
                                ui.set_min_size(inner_size);
                            }
                            
                            ui.horizontal_centered(|ui| {
                                ui.heading(&self.header_text);
                            });
                        });
                });
        }

        if let Some(content_bounds) = bridge.get_bounds(content_id) {
            // Draw content panel
            egui::Area::new(egui::Id::new("content_area"))
                .fixed_pos(content_bounds.min)
                .show(ctx, |ui| {
                    ui.set_max_size(content_bounds.size());
                    
                    // Content frame
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(30, 30, 30))
                        .inner_margin(20.0)
                        .show(ui, |ui| {
                            let inner_size = content_bounds.size() - egui::vec2(40.0, 40.0);
                            if inner_size.x > 0.0 && inner_size.y > 0.0 {
                                ui.set_min_size(inner_size);
                            }
                            
                            ui.vertical(|ui| {
                                ui.label(&self.content_text);
                                
                                ui.add_space(20.0);
                                
                                ui.horizontal(|ui| {
                                    ui.label("Header text:");
                                    ui.text_edit_singleline(&mut self.header_text);
                                });
                                
                                ui.horizontal(|ui| {
                                    ui.label("Content text:");
                                    ui.text_edit_multiline(&mut self.content_text);
                                });
                                
                                ui.checkbox(&mut self.show_debug, "Show debug info");
                                
                                if self.show_debug {
                                    ui.separator();
                                    ui.label(format!("Window size: {:?}", available_size));
                                    ui.label(format!("Header bounds: {:?}", bridge.get_bounds(header_id)));
                                    ui.label(format!("Content bounds: {:?}", bridge.get_bounds(content_id)));
                                }
                            });
                        });
                });
        }

        // Store the bridge back to memory
        bridge.store(ctx);

        // Request repaint for continuous updates
        ctx.request_repaint();
    }
}