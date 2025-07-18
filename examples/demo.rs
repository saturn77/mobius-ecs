use eframe::{egui, App, Frame};
use bevy_ecs::prelude::*;
use mobius_ecs::*;
//use egui::Pos2;
use egui::ViewportBuilder;
//use egui_dock::{DockArea, DockState, NodeIndex, Style, SurfaceIndex};

/// Demo application showcasing Mobius-ECS framework
#[derive(Default)]
pub struct MobiusDemoApp {
    world: World,
    active_tab: TabType,
    app_entity: Option<Entity>,
    current_template: String,
}

impl MobiusDemoApp {
    pub fn new() -> Self {
        Self::default()
    }
}

impl App for MobiusDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Initialize if needed
        if self.app_entity.is_none() {
            let registry = MobiusTemplateRegistry::default();
            let mut commands = self.world.commands();
            self.app_entity = registry.spawn_from_template(&mut commands, "gerber_viewer");
            self.current_template = "gerber_viewer".to_string();
        }
        
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        // Reset and create new project
                        self.world.clear_all();
                        self.app_entity = None;
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("Templates", |ui| {
                    let _registry = MobiusTemplateRegistry::default();
                    
                    if ui.button("🔧 Gerber Viewer").clicked() {
                        self.switch_template("gerber_viewer");
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    if ui.button("📝 Text Editor").clicked() {
                        self.switch_template("text_editor");
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    
                    ui.separator();
                    ui.label("💡 Choose a template to see different layouts");
                });
                
                ui.menu_button("View", |ui| {
                    ui.selectable_value(&mut self.active_tab, TabType::MainWork, "📋 Main Work Area");
                    ui.selectable_value(&mut self.active_tab, TabType::Settings, "⚙️ Settings");
                    ui.selectable_value(&mut self.active_tab, TabType::EventLogger, "📜 Event Logger");
                    ui.selectable_value(&mut self.active_tab, TabType::Controls, "🎛️ Controls");
                });
                
                ui.separator();
                ui.label(format!("Current Template: {}", self.current_template));
            });
        });

        // Tab bar
        egui::TopBottomPanel::top("tab_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, TabType::MainWork, "📋 Main");
                ui.selectable_value(&mut self.active_tab, TabType::Settings, "⚙️ Settings");
                ui.selectable_value(&mut self.active_tab, TabType::EventLogger, "📜 Logs");
                ui.selectable_value(&mut self.active_tab, TabType::Controls, "🎛️ Controls");
                
                ui.separator();
                ui.label("👆 Click tabs to switch views");
            });
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🌀 Mobius-ECS Demo");
                ui.separator();
                ui.label(format!("Entities: {}", self.world.entities().len()));
                ui.separator();
                ui.label(format!("Active Tab: {:?}", self.active_tab));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.hyperlink_to("GitHub", "https://github.com/yourusername/mobius-ecs");
                    ui.label("Made with ❤️ and Rust");
                });
            });
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.active_tab {
                TabType::MainWork => {
                    show_main_work_area(ui, &mut self.world);
                    
                    // Add some demo-specific content
                    ui.separator();
                    ui.collapsing("💡 Demo Info", |ui| {
                        ui.label("This is the main work area where your application content goes.");
                        ui.label("In a real application, this might contain:");
                        ui.label("• Canvas for drawing/viewing");
                        ui.label("• Text editor");
                        ui.label("• 3D viewport");
                        ui.label("• Data tables");
                        ui.label("• etc.");
                    });
                }
                TabType::Settings => {
                    show_settings_panel(ui, &mut self.world);
                    
                    ui.separator();
                    ui.collapsing("💡 Settings Info", |ui| {
                        ui.label("Settings are automatically saved in the ECS world.");
                        ui.label("Changes here update the underlying components.");
                        ui.label("Try changing the units or clock format above!");
                    });
                }
                TabType::EventLogger => {
                    show_event_logger_panel(ui, &mut self.world);
                    
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("➕ Add Test Log").clicked() {
                            self.add_test_log();
                        }
                        if ui.button("⚠️ Add Warning").clicked() {
                            self.add_warning_log();
                        }
                        if ui.button("❌ Add Error").clicked() {
                            self.add_error_log();
                        }
                    });
                }
                TabType::Controls => {
                    show_controls_panel(ui, &mut self.world);
                    
                    ui.separator();
                    ui.collapsing("💡 Controls Info", |ui| {
                        ui.label("Controls panels contain action buttons and tools.");
                        ui.label("These are defined in the template and can be customized.");
                        ui.label("In a real app, these might trigger commands, open dialogs, etc.");
                    });
                }
                TabType::Generic(name) => {
                    show_generic_tab(ui, &self.world, name);
                }
            }
        });
    }
}

impl MobiusDemoApp {
    fn switch_template(&mut self, template_name: &str) {
        // Clear the world and respawn with new template
        self.world.clear_all();
        let registry = MobiusTemplateRegistry::default();
        let mut commands = self.world.commands();
        self.app_entity = registry.spawn_from_template(&mut commands, template_name);
        self.current_template = template_name.to_string();
        self.active_tab = TabType::MainWork;
    }
    
    fn add_test_log(&mut self) {
        let mut query = self.world.query::<&mut EventLoggerPanel>();
        if let Some(mut logger) = query.iter_mut(&mut self.world).next() {
            logger.entries.push(LogEntry {
                timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                level: LogLevel::Info,
                message: "Test log entry added by user".to_string(),
            });
        }
    }
    
    fn add_warning_log(&mut self) {
        let mut query = self.world.query::<&mut EventLoggerPanel>();
        if let Some(mut logger) = query.iter_mut(&mut self.world).next() {
            logger.entries.push(LogEntry {
                timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                level: LogLevel::Warn,
                message: "This is a warning message".to_string(),
            });
        }
    }
    
    fn add_error_log(&mut self) {
        let mut query = self.world.query::<&mut EventLoggerPanel>();
        if let Some(mut logger) = query.iter_mut(&mut self.world).next() {
            logger.entries.push(LogEntry {
                timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                level: LogLevel::Error,
                message: "This is an error message".to_string(),
            });
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Enable logging
    
    eframe::run_native(
        "Mobius ECS Demo",
        eframe::NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size([1280.0, 768.0]),
            ..Default::default()
        },
        Box::new(|cc|{
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MobiusDemoApp::new()))
        }))
}

