// lib.rs - The main mobius-ecs library
use bevy_ecs::prelude::*;
use egui::Ui;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::Local;
use chrono_tz::Tz;

// =============================================================================
// Core Components and Systems (Simplified for MVP)
// =============================================================================

#[derive(Component, Clone, PartialEq, Debug)]
pub enum TabType {
    MainWork,
    Settings,
    EventLogger,
    Controls,
    Generic(String),
}

impl Default for TabType {
    fn default() -> Self {
        TabType::MainWork
    }
}

#[derive(Component)]
pub struct MobiusApp {
    pub app_name      : String,
    pub template_name : String,
    pub active_tab    : TabType,
}

#[derive(Component)]
pub struct MainWorkArea {
    pub content          : String,
    pub background_color : [f32; 4],
}

#[derive(Component)]
pub struct SettingsPanel {
    pub global_units_mils  : bool,
    pub user_timezone      : Option<String>,
    pub use_24_hour_clock  : bool,
    pub interface_language : String,
}

#[derive(Component)]
pub struct EventLoggerPanel {
    pub entries           : Vec<LogEntry>,
    pub max_entries       : usize,
    pub auto_scroll       : bool,
}

#[derive(Clone)]
pub struct LogEntry {
    pub timestamp  : String,
    pub level      : LogLevel,
    pub message    : String,
}

#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Debug, Info, Warn, Error,
}

#[derive(Component)]
pub struct ControlsPanel {
    pub buttons: Vec<ControlButton>,
}

#[derive(Clone)]
pub struct ControlButton {
    pub id: String,
    pub label: String,
    pub enabled: bool,
}

#[derive(Component)]
pub struct GenericTab {
    pub title: String,
    pub content: String,
}

// =============================================================================
// Template System (Simplified)
// =============================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct MobiusTemplate {
    pub name: String,
    pub description: String,
    pub main_area_type: MainAreaType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MainAreaType {
    GerberViewer,
    TextEditor,
    Generic,
}

impl MobiusTemplate {
    pub fn gerber_viewer() -> Self {
        Self {
            name: "gerber_viewer".to_string(),
            description: "PCB Gerber file viewer".to_string(),
            main_area_type: MainAreaType::GerberViewer,
        }
    }
    
    pub fn text_editor() -> Self {
        Self {
            name: "text_editor".to_string(),
            description: "Code/text editor".to_string(),
            main_area_type: MainAreaType::TextEditor,
        }
    }
}

// =============================================================================
// Registry and Spawning
// =============================================================================

#[derive(Resource)]
pub struct MobiusTemplateRegistry {
    pub templates: HashMap<String, MobiusTemplate>,
}

impl Default for MobiusTemplateRegistry {
    fn default() -> Self {
        let mut registry = Self {
            templates: HashMap::new(),
        };
        
        registry.templates.insert("gerber_viewer".to_string(), MobiusTemplate::gerber_viewer());
        registry.templates.insert("text_editor".to_string(), MobiusTemplate::text_editor());
        
        registry
    }
}

impl MobiusTemplateRegistry {
    pub fn get_templates(&self) -> &HashMap<String, MobiusTemplate> {
        &self.templates
    }
    
    pub fn spawn_from_template(&self, commands: &mut Commands, template_name: &str) -> Option<Entity> {
        let template = self.templates.get(template_name)?;
        
        // Create main app entity
        let app_entity = commands.spawn(MobiusApp {
            app_name: template.name.clone(),
            template_name: template_name.to_string(),
            active_tab: TabType::MainWork,
        }).id();
        
        // Create main work area
        let main_content = match template.main_area_type {
            MainAreaType::GerberViewer => "Gerber Viewer - Drop .gbr files here",
            MainAreaType::TextEditor => "Text Editor - Start typing...",
            MainAreaType::Generic => "Generic work area",
        };
        
        commands.spawn((
            MainWorkArea {
                content: main_content.to_string(),
                background_color: [0.1, 0.1, 0.1, 1.0],
            },
            TabType::MainWork,
        ));
        
        // Create settings panel
        commands.spawn((
            SettingsPanel {
                global_units_mils: false,
                user_timezone: None,
                use_24_hour_clock: true,
                interface_language: "English".to_string(),
            },
            TabType::Settings,
        ));
        
        // Create event logger
        commands.spawn((
            EventLoggerPanel {
                entries: vec![
                    LogEntry {
                        timestamp: "12:34:56".to_string(),
                        level: LogLevel::Info,
                        message: format!("Application '{}' started", template.name),
                    }
                ],
                max_entries: 1000,
                auto_scroll: true,
            },
            TabType::EventLogger,
        ));
        
        // Create controls panel
        commands.spawn((
            ControlsPanel {
                buttons: vec![
                    ControlButton {
                        id: "btn1".to_string(),
                        label: "Action 1".to_string(),
                        enabled: true,
                    },
                    ControlButton {
                        id: "btn2".to_string(),
                        label: "Action 2".to_string(),
                        enabled: true,
                    },
                ],
            },
            TabType::Controls,
        ));
        
        Some(app_entity)
    }
}

// =============================================================================
// UI Rendering Functions
// =============================================================================

pub fn show_main_work_area(ui: &mut Ui, world: &mut World) {
    let mut query = world.query::<&MainWorkArea>();
    
    if let Some(main_area) = query.iter(world).next() {
        ui.heading("Main Work Area");
        ui.separator();
        
        ui.group(|ui| {
            ui.label(&main_area.content);
            
            // Placeholder content area
            let available_size = ui.available_size();
            let (response, painter) = ui.allocate_painter(
                egui::Vec2::new(available_size.x, available_size.y.max(200.0)),
                egui::Sense::click_and_drag()
            );
            
            // Draw background
            painter.rect_filled(
                response.rect,
                4.0,
                egui::Color32::from_rgba_premultiplied(
                    (main_area.background_color[0] * 255.0) as u8,
                    (main_area.background_color[1] * 255.0) as u8,
                    (main_area.background_color[2] * 255.0) as u8,
                    (main_area.background_color[3] * 255.0) as u8,
                )
            );
            
            // Draw border
            painter.rect_stroke(
                response.rect, 4.0, (1.0, egui::Color32::GRAY), egui::StrokeKind::Outside
            );
            
            // Draw center text
            painter.text(
                response.rect.center(),
                egui::Align2::CENTER_CENTER,
                "Content Area\n(Implement your specific UI here)",
                egui::FontId::proportional(16.0),
                egui::Color32::WHITE,
            );
        });
    } else {
        ui.label("No main work area found");
    }
}

pub fn show_settings_panel(ui: &mut Ui, world: &mut World) {
    let mut query = world.query::<&mut SettingsPanel>();
    
    if let Some(mut settings) = query.iter_mut(world).next() {
        ui.heading("Application Settings");
        ui.separator();
        
        // Units Section
        ui.group(|ui| {
            ui.label("Display Units");
            ui.horizontal(|ui| {
                ui.label("Global Units:");
                let prev_units = settings.global_units_mils;
                ui.selectable_value(&mut settings.global_units_mils, false, "Millimeters (mm)");
                ui.selectable_value(&mut settings.global_units_mils, true, "Mils (1/1000 inch)");
                
                if prev_units != settings.global_units_mils {
                    let units_name = if settings.global_units_mils { "mils" } else { "mm" };
                    // TODO: Integrate with egui_lens logging
                    println!("Changed global units to {}", units_name);
                }
            });
            ui.label("Affects: Grid spacing, board dimensions, cursor position, zoom selection");
        });
        
        ui.add_space(20.0);
        
        // Timezone Section
        ui.group(|ui| {
            ui.label("Time & Localization");
            ui.horizontal(|ui| {
                ui.label("Timezone:");
                
                // Get current timezone name or use UTC as default
                let current_tz_name = settings.user_timezone.as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("UTC");
                
                egui::ComboBox::from_id_salt("timezone_selector")
                    .selected_text(current_tz_name)
                    .width(300.0)
                    .show_ui(ui, |ui| {
                        // Common timezones first
                        ui.label("Common Timezones:");
                        for tz_name in &[
                            "UTC",
                            "US/Eastern", 
                            "US/Central",
                            "US/Mountain", 
                            "US/Pacific",
                            "Europe/London",
                            "Europe/Paris",
                            "Europe/Berlin",
                            "Asia/Tokyo",
                            "Asia/Shanghai",
                            "Australia/Sydney",
                        ] {
                            if ui.selectable_value(&mut settings.user_timezone, Some(tz_name.to_string()), *tz_name).clicked() {
                                println!("Changed timezone to {}", tz_name);
                            }
                        }
                        
                        ui.separator();
                        ui.label("All Timezones:");
                        
                        // All timezones
                        for tz in chrono_tz::TZ_VARIANTS {
                            let tz_name = tz.name();
                            if ui.selectable_value(&mut settings.user_timezone, Some(tz_name.to_string()), tz_name).clicked() {
                                println!("Changed timezone to {}", tz_name);
                            }
                        }
                    });
            });
            
            ui.add_space(10.0);
            
            // Clock format selection
            ui.horizontal(|ui| {
                ui.label("Clock Format:");
                let prev_format = settings.use_24_hour_clock;
                ui.selectable_value(&mut settings.use_24_hour_clock, true, "24-hour (13:30:45)");
                ui.selectable_value(&mut settings.use_24_hour_clock, false, "12-hour (1:30:45 PM)");
                
                if prev_format != settings.use_24_hour_clock {
                    let format_name = if settings.use_24_hour_clock { "24-hour" } else { "12-hour" };
                    println!("Changed clock format to {}", format_name);
                }
            });
            
            // Show current time in selected timezone with chosen format (auto-updating)
            ui.group(|ui| {
                ui.label("🕐 Live Clock:");
                let time_format = if settings.use_24_hour_clock { "%Y-%m-%d %H:%M:%S %Z" } else { "%Y-%m-%d %I:%M:%S %p %Z" };
                
                if let Some(tz_name) = &settings.user_timezone {
                    if let Ok(tz) = tz_name.parse::<Tz>() {
                        let now = Local::now().with_timezone(&tz);
                        ui.colored_label(egui::Color32::from_rgb(100, 255, 100), format!("{}", now.format(time_format)));
                    }
                } else {
                    let now = Local::now();
                    ui.colored_label(egui::Color32::from_rgb(100, 255, 100), format!("{}", now.format(if settings.use_24_hour_clock { "%Y-%m-%d %H:%M:%S" } else { "%Y-%m-%d %I:%M:%S %p" })));
                }
            });
            
            // Request repaint every second to update the clock
            ui.ctx().request_repaint_after(std::time::Duration::from_secs(1));
        });
        
        ui.add_space(20.0);
        
        // Language Section
        ui.group(|ui| {
            ui.label("Language");
            ui.horizontal(|ui| {
                ui.label("Interface Language:");
                
                egui::ComboBox::from_id_salt("language_selector")
                    .selected_text(&settings.interface_language)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut settings.interface_language, "English".to_string(), "English");
                        ui.add_enabled(false, egui::Label::new("Français (coming soon)"));
                        ui.add_enabled(false, egui::Label::new("Deutsch (coming soon)"));
                        ui.add_enabled(false, egui::Label::new("中文 (coming soon)"));
                        ui.add_enabled(false, egui::Label::new("日本語 (coming soon)"));
                    });
            });
        });
    } else {
        ui.label("No settings panel found");
    }
}

pub fn show_event_logger_panel(ui: &mut Ui, world: &mut World) {
    let mut query = world.query::<&mut EventLoggerPanel>();
    
    if let Some(mut logger) = query.iter_mut(world).next() {
        ui.heading("Event Logger");
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Clear Logs").clicked() {
                logger.entries.clear();
            }
            ui.colored_label(egui::Color32::from_rgb(100, 200, 100), format!("📝 {} entries", logger.entries.len()));
            ui.separator();
            ui.colored_label(egui::Color32::from_rgb(150, 150, 150), "⬆️ Newest messages on top");
        });
        
        ui.separator();
        
        // Scroll area that shows newest messages at top
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for entry in &logger.entries {
                    ui.horizontal(|ui| {
                        let color = match entry.level {
                            LogLevel::Debug => egui::Color32::GRAY,
                            LogLevel::Info => egui::Color32::WHITE,
                            LogLevel::Warn => egui::Color32::YELLOW,
                            LogLevel::Error => egui::Color32::RED,
                        };
                        
                        ui.colored_label(color, &entry.timestamp);
                        ui.colored_label(color, format!("[{:?}]", entry.level));
                        ui.colored_label(color, &entry.message);
                    });
                }
            });
    } else {
        ui.label("No event logger found");
    }
}

pub fn show_controls_panel(ui: &mut Ui, world: &mut World) {
    let mut query = world.query::<&ControlsPanel>();
    
    if let Some(controls) = query.iter(world).next() {
        ui.heading("Controls");
        ui.separator();
        
        ui.group(|ui| {
            for button in &controls.buttons {
                ui.add_enabled(button.enabled, egui::Button::new(&button.label));
            }
        });
    } else {
        ui.label("No controls panel found");
    }
}

pub fn show_generic_tab(ui: &mut Ui, _world: &World, tab_name: &str) {
    ui.heading(format!("Generic Tab: {}", tab_name));
    ui.separator();
    ui.label("This is a placeholder for generic tab content");
}

// =============================================================================
// CLI Generation (Simplified)
// =============================================================================

pub fn generate_mobius_project(
    template_name: &str,
    project_name: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let registry = MobiusTemplateRegistry::default();
    
    if let Some(_template) = registry.templates.get(template_name) {
        // Create directory structure
        std::fs::create_dir_all(format!("{}/src", output_dir))?;
        
        // Generate Cargo.toml with current versions
        let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
eframe = "0.31"
egui = "0.31"
bevy_ecs = "0.14"
serde = {{ version = "1.0", features = ["derive"] }}
chrono = {{ version = "0.4", features = ["serde"] }}
chrono-tz = "0.8"
# egui_lens = {{ git = "https://github.com/yourusername/egui_lens" }}
# egui_mobius_reactive = "0.1"
"#, project_name);
        
        std::fs::write(format!("{}/Cargo.toml", output_dir), cargo_toml)?;
        
        // Generate main.rs
        let main_rs = format!(r#"use eframe::{{egui, App, Frame}};
use bevy_ecs::prelude::*;

mod mobius;
use mobius::*;

#[derive(Default)]
pub struct {}App {{
    world: World,
    active_tab: TabType,
    app_entity: Option<Entity>,
}}

impl App for {}App {{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {{
        // Initialize if needed
        if self.app_entity.is_none() {{
            let registry = MobiusTemplateRegistry::default();
            let mut commands = self.world.commands();
            self.app_entity = registry.spawn_from_template(&mut commands, "{}");
        }}
        
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {{
            egui::menu::bar(ui, |ui| {{
                ui.menu_button("File", |ui| {{
                    if ui.button("New").clicked() {{
                        // Handle new
                    }}
                }});
                ui.menu_button("View", |ui| {{
                    ui.selectable_value(&mut self.active_tab, TabType::MainWork, "Main Work Area");
                    ui.selectable_value(&mut self.active_tab, TabType::Settings, "Settings");
                    ui.selectable_value(&mut self.active_tab, TabType::EventLogger, "Event Logger");
                    ui.selectable_value(&mut self.active_tab, TabType::Controls, "Controls");
                }});
            }});
        }});

        // Tab bar
        egui::TopBottomPanel::top("tab_panel").show(ctx, |ui| {{
            ui.horizontal(|ui| {{
                ui.selectable_value(&mut self.active_tab, TabType::MainWork, "📋 Main");
                ui.selectable_value(&mut self.active_tab, TabType::Settings, "⚙️ Settings");
                ui.selectable_value(&mut self.active_tab, TabType::EventLogger, "📜 Logs");
                ui.selectable_value(&mut self.active_tab, TabType::Controls, "🎛️ Controls");
            }});
        }});

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {{
            match &self.active_tab {{
                TabType::MainWork => show_main_work_area(ui, &self.world),
                TabType::Settings => show_settings_panel(ui, &mut self.world),
                TabType::EventLogger => show_event_logger_panel(ui, &mut self.world),
                TabType::Controls => show_controls_panel(ui, &self.world),
                TabType::Generic(name) => show_generic_tab(ui, &self.world, name),
            }}
        }});
    }}
}}

fn main() -> Result<(), eframe::Error> {{
    let options = eframe::NativeOptions {{
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    }};
    eframe::run_native(
        "{}",
        options,
        Box::new(|_cc| Box::new({}App::default())),
    )
}}
"#, project_name, project_name, template_name, project_name, project_name);
        
        std::fs::write(format!("{}/src/main.rs", output_dir), main_rs)?;
        
        // Copy the mobius library code (this would be a dependency in real usage)
        std::fs::write(format!("{}/src/mobius.rs", output_dir), "// This would contain the mobius-ecs library code\n// In real usage, this would be a crate dependency")?;
        
        println!("✅ Generated Mobius-ECS project '{}' from template '{}'", project_name, template_name);
        println!("📁 Project created in: {}", output_dir);
        println!("🚀 Run with: cd {} && cargo run", output_dir);
        
        Ok(())
    } else {
        Err(format!("Template '{}' not found. Available templates: {:?}", 
                   template_name, 
                   registry.get_templates().keys().collect::<Vec<_>>()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_registry() {
        let registry = MobiusTemplateRegistry::default();
        assert!(registry.templates.contains_key("gerber_viewer"));
        assert!(registry.templates.contains_key("text_editor"));
    }
}