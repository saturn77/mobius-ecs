use eframe::{egui, App, Frame};
use bevy_ecs::prelude::*;
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};
use mobius_ecs::*;
use eframe::egui::ViewportBuilder;

// Dummy ToolWindows definition for compilation; replace with your actual implementation if needed
#[derive(Default)]
pub struct ToolWindows;

impl ToolWindows {
    pub fn show(&mut self, _window_id: &str) {}
    pub fn show_all(&mut self) {}
    pub fn hide_all(&mut self) {}
    pub fn visible_count(&self) -> usize { 0 }
    pub fn ui<F: FnMut(&mut egui::Ui, &str)>(&mut self, _ctx: &egui::Context, _f: F) {}
}

#[derive(Clone, Debug)]
struct Tab {
    name: String,
    kind: TabKind,
}

#[derive(Clone, Debug)]
enum TabKind {
    MainWork,
    Settings,
    EventLogger,
    Controls,
    Inspector,
}

pub struct MobiusToolWindowsDemo {
    world: World,
    app_entity: Option<Entity>,
    current_template: String,
    dock_state: DockState<Tab>,
    tab_viewer: MobiusTabViewer,
    tool_windows: ToolWindows, // Add this line
}

#[derive(Default)]
struct MobiusTabViewer {
    world_ptr: Option<*mut World>,
}

impl MobiusTabViewer {
    fn with_world(&mut self, world: *mut World) -> &mut Self {
        self.world_ptr = Some(world);
        self
    }
}

impl TabViewer for MobiusTabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        match tab.kind {
            TabKind::MainWork => "📋 Main Work Area".into(),
            TabKind::Settings => "⚙️ Settings".into(),
            TabKind::EventLogger => "📜 Event Logger".into(),
            TabKind::Controls => "🎛️ Controls".into(),
            TabKind::Inspector => "🔍 Inspector".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, tab: &mut Self::Tab) {
        if let Some(world_ptr) = self.world_ptr {
            let world = unsafe { &mut *world_ptr };
            
            match tab.kind {
                TabKind::MainWork => {
                    show_main_work_area(ui, world);
                    
                    ui.add_space(10.0);
                    ui.collapsing("💡 Main Work Area", |ui| {
                        ui.label("This is your primary workspace:");
                        ui.indent("indent", |ui| {
                            ui.label("• Gerber viewer with PCB visualization");
                            ui.label("• Text editor with syntax highlighting");
                            ui.label("• 3D model viewer");
                            ui.label("• Canvas for drawing/CAD operations");
                        });
                    });
                }
                TabKind::Settings => {
                    show_settings_panel(ui, world);
                    
                    ui.add_space(10.0);
                    ui.collapsing("💡 Settings Help", |ui| {
                        ui.label("Settings are automatically saved in the ECS world.");
                        ui.label("Try changing the units or timezone above!");
                    });
                }
                TabKind::EventLogger => {
                    show_event_logger_panel(ui, world);
                    
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("➕ Add Info").clicked() {
                            add_test_log(world);
                        }
                        if ui.button("⚠️ Add Warning").clicked() {
                            add_warning_log(world);
                        }
                        if ui.button("❌ Add Error").clicked() {
                            add_error_log(world);
                        }
                    });
                    
                    ui.add_space(5.0);
                    ui.collapsing("💡 Logger Help", |ui| {
                        ui.label("Event logging with colored levels and timestamps.");
                        ui.label("Ready for egui_lens integration!");
                    });
                }
                TabKind::Controls => {
                    show_controls_panel(ui, world);
                    
                    ui.add_space(10.0);
                    ui.collapsing("💡 Controls Help", |ui| {
                        ui.label("Tool palettes and action buttons.");
                        ui.label("Configurable layouts and shortcuts.");
                    });
                }
                TabKind::Inspector => {
                    ui.heading("🔍 Inspector");
                    ui.separator();
                    
                    // Show current world state
                    ui.group(|ui| {
                        ui.label("ECS World Inspector");
                        ui.label(format!("Total entities: {}", world.entities().len()));
                        
                        ui.add_space(10.0);
                        ui.label("Active Components:");
                        ui.indent("components", |ui| {
                            // Query for different component types
                            let main_work_count = world.query::<&MainWorkArea>().iter(world).count();
                            let settings_count = world.query::<&SettingsPanel>().iter(world).count();
                            let logger_count = world.query::<&EventLoggerPanel>().iter(world).count();
                            let controls_count = world.query::<&ControlsPanel>().iter(world).count();
                            
                            ui.label(format!("📋 MainWorkArea: {}", main_work_count));
                            ui.label(format!("⚙️ SettingsPanel: {}", settings_count));
                            ui.label(format!("📜 EventLoggerPanel: {}", logger_count));
                            ui.label(format!("🎛️ ControlsPanel: {}", controls_count));
                        });
                    });
                    
                    ui.add_space(10.0);
                    if ui.button("🔄 Refresh Inspector").clicked() {
                        // Force refresh
                    }
                }
            }
        }
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        true
    }
}

impl MobiusToolWindowsDemo {
    pub fn new() -> Self {

        // Create the initial dock layout
        let mut dock_state = DockState::new(vec![Tab {
            name: "Main".to_string(),
            kind: TabKind::MainWork,
        }]);

        // Split the surface into multiple areas
        let surface = dock_state.main_surface_mut();
        
        // Create left panel
        let [left, _] = surface.split_left(NodeIndex::root(), 0.2, vec![Tab {
            name: "Controls".to_string(),
            kind: TabKind::Controls,
        }]);
        
        // Add inspector below controls
        let [_, below] = surface.split_below(left, 0.5, vec![Tab {
            name: "Inspector".to_string(),
            kind: TabKind::Inspector,
        }]);
        
        // Split main area to add settings and logger
        let [right_top, _] = surface.split_right(NodeIndex::root(), 0.25, vec![Tab {
            name: "Settings".to_string(),
            kind: TabKind::Settings,
        }]);
        
        // Add event logger at the bottom
        let [_, bottom] = surface.split_below(NodeIndex::root(), 0.7, vec![Tab {
            name: "Event Logger".to_string(),
            kind: TabKind::EventLogger,
        }]);
        Self {
            world: World::new(),
            app_entity: None,
            current_template: "gerber_viewer".to_string(),
            dock_state,
            tab_viewer: MobiusTabViewer::default(),
            tool_windows: ToolWindows::default(), // Add this line
        }
        }
            fn switch_template(&mut self, template_name: &str) {
        self.world.clear_all();
        let registry = MobiusTemplateRegistry::default();
        let mut commands = self.world.commands();
        self.app_entity = registry.spawn_from_template(&mut commands, template_name);
        self.current_template = template_name.to_string();
    }
    }




// Helper functions moved outside of struct to be accessible by TabViewer
fn add_test_log(world: &mut World) {
    let mut query = world.query::<&mut EventLoggerPanel>();
    if let Some(mut logger) = query.iter_mut(world).next() {
        logger.entries.push(LogEntry {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            level: LogLevel::Info,
            message: "Test log entry added by user".to_string(),
        });
    }
}

fn add_warning_log(world: &mut World) {
    let mut query = world.query::<&mut EventLoggerPanel>();
    if let Some(mut logger) = query.iter_mut(world).next() {
        logger.entries.push(LogEntry {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            level: LogLevel::Warn,
            message: "This is a warning message".to_string(),
        });
    }
}

fn add_error_log(world: &mut World) {
    let mut query = world.query::<&mut EventLoggerPanel>();
    if let Some(mut logger) = query.iter_mut(world).next() {
        logger.entries.push(LogEntry {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            level: LogLevel::Error,
            message: "This is an error message".to_string(),
        });
    }
}

impl App for MobiusToolWindowsDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Initialize if needed
        if self.app_entity.is_none() {
            let registry = MobiusTemplateRegistry::default();
            let mut commands = self.world.commands();
            self.app_entity = registry.spawn_from_template(&mut commands, "gerber_viewer");
        }
        
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("🆕 New Project").clicked() {
                        self.world.clear_all();
                        self.app_entity = None;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("📁 Open...").clicked() {
                        // Handle open
                    }
                    if ui.button("💾 Save").clicked() {
                        // Handle save
                    }
                    ui.separator();
                    if ui.button("❌ Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("Templates", |ui| {
                    if ui.button("🔧 Gerber Viewer").clicked() {
                        self.switch_template("gerber_viewer");
                        ui.close_menu();
                    }
                    if ui.button("📝 Text Editor").clicked() {
                        self.switch_template("text_editor");
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.label("💡 Switch templates to see different layouts");
                });
                
                ui.menu_button("Windows", |ui| {
                    ui.label("Docked Tabs:");
                    if ui.button("📋 Add Main Work Tab").clicked() {
                        self.dock_state.push_to_focused_leaf(Tab {
                            name: "Main Work".to_string(),
                            kind: TabKind::MainWork,
                        });
                    }
                    if ui.button("⚙️ Add Settings Tab").clicked() {
                        self.dock_state.push_to_focused_leaf(Tab {
                            name: "Settings".to_string(),
                            kind: TabKind::Settings,
                        });
                    }
                    if ui.button("📜 Add Event Logger Tab").clicked() {
                        self.dock_state.push_to_focused_leaf(Tab {
                            name: "Event Logger".to_string(),
                            kind: TabKind::EventLogger,
                        });
                    }
                    ui.separator();
                    ui.label("Floating Windows:");
                    if ui.button("📊 Show Properties").clicked() {
                        self.tool_windows.show("properties");
                    }
                    if ui.button("📑 Show Layers").clicked() {
                        self.tool_windows.show("layers");
                    }
                    ui.separator();
                    if ui.button("🔄 Reset Dock Layout").clicked() {
                        // Reset to initial layout
                        self.dock_state = DockState::new(vec![Tab {
                            name: "Main".to_string(),
                            kind: TabKind::MainWork,
                        }]);
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("🌟 Show All Floating Windows").clicked() {
                        self.tool_windows.show_all();
                    }
                    if ui.button("🙈 Hide All Floating Windows").clicked() {
                        self.tool_windows.hide_all();
                    }
                });
                
                ui.separator();
                ui.label(format!("Template: {}", self.current_template));
            });
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🌀 Mobius-ECS with Dockable Windows");
                ui.separator();
                ui.label(format!("Entities: {}", self.world.entities().len()));
                ui.separator();
                ui.label(format!("Floating Windows: {}", self.tool_windows.visible_count()));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.hyperlink_to("📖 GitHub", "https://github.com/yourusername/mobius-ecs");
                    ui.label("Made with ❤️ and Rust");
                });
            });
        });

        // Show floating tool windows
        self.tool_windows.ui(ctx, |ui, window_id| {
            match window_id {
                "properties" => {
                    ui.heading("📊 Properties Panel");
                    ui.separator();
                    
                    // Show properties for selected entity
                    if let Some(entity) = self.app_entity {
                        ui.label(format!("Selected Entity: {:?}", entity));
                        
                        // Query for components on this entity
                        if let Some(entity_ref) = self.world.get_entity(entity) {
                            ui.label("Components:");
                            ui.indent("components", |ui| {
                                // This is where you'd inspect specific components
                                ui.label("• Transform");
                                ui.label("• Mesh");
                                ui.label("• Material");
                            });
                        }
                    } else {
                        ui.label("No entity selected");
                    }
                }
                "layers" => {
                    ui.heading("📑 Layers Panel");
                    ui.separator();
                    
                    // Example layer management
                    ui.label("PCB Layers:");
                    ui.indent("layers", |ui| {
                        ui.checkbox(&mut true, "Top Copper");
                        ui.checkbox(&mut true, "Bottom Copper");
                        ui.checkbox(&mut false, "Inner Layer 1");
                        ui.checkbox(&mut false, "Inner Layer 2");
                        ui.checkbox(&mut true, "Silk Screen");
                        ui.checkbox(&mut true, "Solder Mask");
                    });
                }
                _ => {
                    ui.label(format!("Unknown window: {}", window_id));
                }
            }
        });

        // Central panel with docking area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Update the tab viewer with the world pointer
            self.tab_viewer.with_world(&mut self.world as *mut World);
            
            // Show the docking area
            DockArea::new(&mut self.dock_state)
                .style(Style::from_egui(ui.ctx().style().as_ref()))
                .show_inside(ui, &mut self.tab_viewer);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Enable logging
    
    eframe::run_native(
        "Mobius ECS Demo - Dockable Windows",
        eframe::NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size([1400.0, 900.0]),
            ..Default::default()
        },
        Box::new(|cc|{
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MobiusToolWindowsDemo::new()))
        }))
}