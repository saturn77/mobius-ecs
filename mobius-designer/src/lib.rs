pub mod components;
pub mod systems;
pub mod resources;
pub mod bundles;
pub mod integration;
pub mod utils;
pub mod codegen;
pub mod syntax_highlighting;
pub mod events;
pub mod codegen_thread;

// Re-export commonly used items
pub use components::*;
pub use systems::*;
pub use resources::*;
pub use bundles::*;
pub use integration::*;
// Don't re-export utils::* to avoid ambiguous glob reexport with grid module
pub use utils::{get_grid_status, clear_all_selections, reset_button_clicks, add_designer_log, snap_to_grid, clear_all_ui_elements};

// Re-export external dependencies for convenience
pub use bevy_ecs;
pub use egui;
pub use egui_dock;
pub use mobius_ecs;

use bevy_ecs::prelude::*;
use std::path::PathBuf;
use crate::systems::tabs::PreviewMode;

/// Create the default dock layout with the desired tab arrangement
fn create_default_dock_layout() -> egui_dock::DockState<Tab> {
    use egui_dock::DockState;
    
    // Create a simple dock state with all tabs - user can arrange them as desired
    let dock_state = DockState::new(vec![
        Tab { name: "Main Work".to_string(), kind: TabKind::MainWork, id: 0 },
        Tab { name: "Inspector".to_string(), kind: TabKind::Inspector, id: 1 },
        Tab { name: "Controls".to_string(), kind: TabKind::Controls, id: 2 },
        Tab { name: "Settings".to_string(), kind: TabKind::Settings, id: 3 },
        Tab { name: "Event Logger".to_string(), kind: TabKind::EventLogger, id: 4 },
        Tab { name: "Preview".to_string(), kind: TabKind::Preview, id: 5 },
    ]);
    
    dock_state
}

/// Get the path for storing dock layout
fn get_dock_layout_path() -> PathBuf {
    let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    path.push("mobius_designer_layout.json");
    path
}

/// Save dock layout to file
fn save_dock_layout(dock_state: &egui_dock::DockState<Tab>) {
    if let Ok(serialized) = serde_json::to_string_pretty(dock_state) {
        let path = get_dock_layout_path();
        if let Err(e) = std::fs::write(&path, serialized) {
            eprintln!("Failed to save dock layout: {}", e);
        }
    }
}

/// Load dock layout from file, or return default if file doesn't exist
fn load_dock_layout() -> egui_dock::DockState<Tab> {
    let path = get_dock_layout_path();
    
    if let Ok(contents) = std::fs::read_to_string(&path) {
        if let Ok(dock_state) = serde_json::from_str::<egui_dock::DockState<Tab>>(&contents) {
            return dock_state;
        }
    }
    
    // Return default layout if loading fails
    create_default_dock_layout()
}

/// Initialize a new ECS world with default designer components
pub fn create_designer_world() -> World {
    let mut world = World::new();
    
    // Add default grid settings
    world.spawn(GridSettings::default());
    
    // Add distribution settings as a resource
    world.insert_resource(DistributionSettings::new());
    
    // Add default panels from mobius_ecs
    world.spawn(mobius_ecs::MainWorkArea {
        content: "Welcome to Mobius Designer!".to_string(),
        background_color: [0.1, 0.1, 0.1, 1.0],
    });
    
    world.spawn(DesignerSettingsPanel {
        units: LengthUnit::Metric,
        timezone: "UTC".to_string(),
    });
    
    world.spawn(mobius_ecs::EventLoggerPanel {
        entries: vec![
            mobius_ecs::LogEntry {
                timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                level: mobius_ecs::LogLevel::Info,
                message: "Designer initialized".to_string(),
            }
        ],
        max_entries: 1000,
        auto_scroll: true,
    });
    
    world.spawn(DesignerControlsPanel {
        selected_tool: Tool::Select,
    });
    
    world
}

/// Add a UI button to the world
pub fn add_ui_button(
    world: &mut World,
    label: String,
    x: f32,
    y: f32,
    tab_kind: TabKind,
) -> Entity {
    world.spawn(UiButtonBundle {
        button: UiButton {
            label,
            clicked: false,
            enabled: true,
            click_time: None,
            font_size: 14.0,
        },
        position: UiElementPosition { x, y },
        size: UiElementSize { width: 100.0, height: 30.0 },
        tab: UiElementTab { tab_kind, position: 0 },
        selected: UiElementSelected::default(),
        container: UiElementContainer { parent_group: None },
    }).id()
}

/// Add a UI text input to the world
pub fn add_ui_text_input(
    world: &mut World,
    label: String,
    x: f32,
    y: f32,
    tab_kind: TabKind,
) -> Entity {
    world.spawn(UiTextInputBundle {
        text_input: UiTextInput {
            label,
            value: String::new(),
            enabled: true,
            font_size: 14.0,
        },
        position: UiElementPosition { x, y },
        size: UiElementSize { width: 200.0, height: 50.0 },
        tab: UiElementTab { tab_kind, position: 0 },
        selected: UiElementSelected::default(),
        container: UiElementContainer { parent_group: None },
    }).id()
}

/// Add a UI checkbox to the world
pub fn add_ui_checkbox(
    world: &mut World,
    label: String,
    x: f32,
    y: f32,
    tab_kind: TabKind,
) -> Entity {
    world.spawn(UiCheckboxBundle {
        checkbox: UiCheckbox {
            label,
            checked: false,
            enabled: true,
            font_size: 14.0,
        },
        position: UiElementPosition { x, y },
        size: UiElementSize { width: 150.0, height: 25.0 },
        tab: UiElementTab { tab_kind, position: 0 },
        selected: UiElementSelected::default(),
        container: UiElementContainer { parent_group: None },
    }).id()
}

/// Add a UI radio button to the world
pub fn add_ui_radio_button(
    world: &mut World,
    label: String,
    group_id: String,
    x: f32,
    y: f32,
    tab_kind: TabKind,
) -> Entity {
    world.spawn(UiRadioButtonBundle {
        radio_button: UiRadioButton {
            label,
            selected: false,
            enabled: true,
            font_size: 14.0,
            group_id,
        },
        position: UiElementPosition { x, y },
        size: UiElementSize { width: 150.0, height: 25.0 },
        tab: UiElementTab { tab_kind, position: 0 },
        selected: UiElementSelected::default(),
        container: UiElementContainer { parent_group: None },
    }).id()
}

/// Add a UI group box to the world
pub fn add_ui_group_box(
    world: &mut World,
    label: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    tab_kind: TabKind,
) -> Entity {
    world.spawn(UiGroupBoxBundle {
        group_box: UiGroupBox {
            label,
            enabled: true,
            font_size: 14.0,
            contained_widgets: Vec::new(),
        },
        position: UiElementPosition { x, y },
        size: UiElementSize { width, height },
        tab: UiElementTab { tab_kind, position: 0 },
        selected: UiElementSelected::default(),
        container: UiElementContainer { parent_group: None },
    }).id()
}

/// Designer application state
pub struct DesignerApp {
    pub world: World,
    pub selection_state: SelectionState,
    pub distribution_settings: DistributionSettings,
    pub dock_state: egui_dock::DockState<Tab>,
    pub tab_viewer: MobiusTabViewer,
    pub codegen_state: events::CodeGenState,
    pub world_snapshot: egui_mobius::types::Value<Option<crate::codegen_thread::WorldSnapshot>>,
}

impl DesignerApp {
    pub fn new() -> Self {
        use egui_mobius::factory;
        
        // Create signal/slot pairs for code generation
        let (signal_to_codegen, slot_from_ui) = factory::create_signal_slot::<events::CodeGenEvent>();
        let (signal_to_ui, slot_for_responses) = factory::create_signal_slot::<events::CodeGenResponse>();
        
        // Create shared world snapshot
        let world_snapshot = egui_mobius::types::Value::new(None);
        
        // Start the background code generation thread
        crate::codegen_thread::start_codegen_thread(
            slot_from_ui,
            signal_to_ui.clone(),
            world_snapshot.clone(),
        );
        
        let mut app = Self {
            world: create_designer_world(),
            selection_state: SelectionState::default(),
            distribution_settings: DistributionSettings::new(),
            dock_state: load_dock_layout(),
            tab_viewer: MobiusTabViewer::new(),
            codegen_state: events::CodeGenState::new(signal_to_codegen, slot_for_responses),
            world_snapshot: world_snapshot.clone(),
        };
        
        // Start background timer thread for code generation
        {
            let signal_to_codegen = app.codegen_state.signal_to_codegen.clone();
            
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    
                    // Trigger code generation for both modes every 3 seconds
                    let _ = signal_to_codegen.send(crate::events::CodeGenEvent::RegenerateCode {
                        tab_kind: crate::integration::TabKind::MainWork,
                        mode: crate::events::CodeGenMode::PanelFunction,
                    });
                    let _ = signal_to_codegen.send(crate::events::CodeGenEvent::RegenerateCode {
                        tab_kind: crate::integration::TabKind::MainWork,
                        mode: crate::events::CodeGenMode::FullApp,
                    });
                }
            });
        }
        
        // Set up the tab viewer with world pointer
        app.tab_viewer.set_world(&mut app.world as *mut World);
        
        app
    }
    
    pub fn add_sample_ui_elements(&mut self) {
        add_ui_button(&mut self.world, "Sample Button".to_string(), 50.0, 50.0, TabKind::MainWork);
        add_ui_text_input(&mut self.world, "Name".to_string(), 50.0, 100.0, TabKind::MainWork);
        add_ui_checkbox(&mut self.world, "Enable Feature".to_string(), 50.0, 170.0, TabKind::MainWork);
        add_ui_radio_button(&mut self.world, "Option A".to_string(), "group1".to_string(), 50.0, 220.0, TabKind::MainWork);
        add_ui_radio_button(&mut self.world, "Option B".to_string(), "group1".to_string(), 50.0, 250.0, TabKind::MainWork);
        add_ui_group_box(&mut self.world, "Settings".to_string(), 300.0, 50.0, 200.0, 150.0, TabKind::MainWork);
    }
    
    fn get_current_tab_kind(&mut self) -> Option<TabKind> {
        if let Some(tab) = self.dock_state.find_active_focused() {
            Some(tab.1.kind.clone())
        } else {
            Some(TabKind::MainWork) // Default fallback
        }
    }
    
    fn add_button_to_current_tab(&mut self) {
        if let Some(current_tab_kind) = self.get_current_tab_kind() {
            let button_count = self.world.query::<&UiButton>().iter(&self.world).count();
            
            // Generate a position with some offset to avoid overlap
            let mut x_offset = (button_count % 5) as f32 * 120.0 + 20.0;
            let mut y_offset = (button_count / 5) as f32 * 50.0 + 50.0;
            
            // Apply grid snapping to initial position
            {
                let mut grid_query = self.world.query::<&GridSettings>();
                if let Some(grid_settings) = grid_query.iter(&self.world).next() {
                    let snapped_pos = snap_to_grid(egui::Pos2::new(x_offset, y_offset), grid_settings.spacing_pixels);
                    x_offset = snapped_pos.x;
                    y_offset = snapped_pos.y;
                }
            }
    
            self.world.spawn(UiButtonBundle {
                button: UiButton {
                    label: format!("Button {}", button_count + 1),
                    clicked: false,
                    enabled: true,
                    click_time: None,
                    font_size: 14.0,
                },
                tab: UiElementTab {
                    tab_kind: current_tab_kind,
                    position: 0,
                },
                position: UiElementPosition {
                    x: x_offset,
                    y: y_offset,
                },
                size: UiElementSize::default(),
                selected: UiElementSelected::default(),
                container: UiElementContainer { parent_group: None },
            });
    
            add_designer_log(&mut self.world, &format!("Added Button {} to tab", button_count + 1));
        }
    }
    
    fn add_text_input_to_current_tab(&mut self) {
        if let Some(current_tab_kind) = self.get_current_tab_kind() {
            let input_count = self.world.query::<&UiTextInput>().iter(&self.world).count();
            
            let mut x_offset = (input_count % 3) as f32 * 200.0 + 20.0;
            let mut y_offset = (input_count / 3) as f32 * 50.0 + 150.0;
            
            {
                let mut grid_query = self.world.query::<&GridSettings>();
                if let Some(grid_settings) = grid_query.iter(&self.world).next() {
                    let snapped_pos = snap_to_grid(egui::Pos2::new(x_offset, y_offset), grid_settings.spacing_pixels);
                    x_offset = snapped_pos.x;
                    y_offset = snapped_pos.y;
                }
            }
    
            self.world.spawn(UiTextInputBundle {
                text_input: UiTextInput {
                    label: format!("Input {}", input_count + 1),
                    value: String::new(),
                    enabled: true,
                    font_size: 14.0,
                },
                tab: UiElementTab {
                    tab_kind: current_tab_kind,
                    position: 0,
                },
                position: UiElementPosition {
                    x: x_offset,
                    y: y_offset,
                },
                size: UiElementSize { width: 200.0, height: 30.0 },
                selected: UiElementSelected::default(),
                container: UiElementContainer { parent_group: None },
            });
    
            add_designer_log(&mut self.world, &format!("Added Text Input {} to tab", input_count + 1));
        }
    }
    
    fn add_checkbox_to_current_tab(&mut self) {
        if let Some(current_tab_kind) = self.get_current_tab_kind() {
            let checkbox_count = self.world.query::<&UiCheckbox>().iter(&self.world).count();
            
            let mut x_offset = (checkbox_count % 4) as f32 * 150.0 + 20.0;
            let mut y_offset = (checkbox_count / 4) as f32 * 50.0 + 250.0;
            
            {
                let mut grid_query = self.world.query::<&GridSettings>();
                if let Some(grid_settings) = grid_query.iter(&self.world).next() {
                    let snapped_pos = snap_to_grid(egui::Pos2::new(x_offset, y_offset), grid_settings.spacing_pixels);
                    x_offset = snapped_pos.x;
                    y_offset = snapped_pos.y;
                }
            }
    
            self.world.spawn(UiCheckboxBundle {
                checkbox: UiCheckbox {
                    label: format!("Checkbox {}", checkbox_count + 1),
                    checked: false,
                    enabled: true,
                    font_size: 14.0,
                },
                tab: UiElementTab {
                    tab_kind: current_tab_kind,
                    position: 0,
                },
                position: UiElementPosition {
                    x: x_offset,
                    y: y_offset,
                },
                size: UiElementSize::default(),
                selected: UiElementSelected::default(),
                container: UiElementContainer { parent_group: None },
            });
    
            add_designer_log(&mut self.world, &format!("Added Checkbox {} to tab", checkbox_count + 1));
        }
    }
    
    fn add_radio_button_to_current_tab(&mut self) {
        if let Some(current_tab_kind) = self.get_current_tab_kind() {
            let radio_count = self.world.query::<&UiRadioButton>().iter(&self.world).count();
            
            let mut x_offset = (radio_count % 4) as f32 * 150.0 + 20.0;
            let mut y_offset = (radio_count / 4) as f32 * 50.0 + 350.0;
            
            {
                let mut grid_query = self.world.query::<&GridSettings>();
                if let Some(grid_settings) = grid_query.iter(&self.world).next() {
                    let snapped_pos = snap_to_grid(egui::Pos2::new(x_offset, y_offset), grid_settings.spacing_pixels);
                    x_offset = snapped_pos.x;
                    y_offset = snapped_pos.y;
                }
            }
    
            self.world.spawn(UiRadioButtonBundle {
                radio_button: UiRadioButton {
                    label: format!("Radio {}", radio_count + 1),
                    selected: false,
                    enabled: true,
                    font_size: 14.0,
                    group_id: "default_group".to_string(),
                },
                tab: UiElementTab {
                    tab_kind: current_tab_kind,
                    position: 0,
                },
                position: UiElementPosition {
                    x: x_offset,
                    y: y_offset,
                },
                size: UiElementSize::default(),
                selected: UiElementSelected::default(),
                container: UiElementContainer { parent_group: None },
            });
    
            add_designer_log(&mut self.world, &format!("Added Radio Button {} to tab", radio_count + 1));
        }
    }
    
    fn add_group_box_to_current_tab(&mut self) {
        if let Some(current_tab_kind) = self.get_current_tab_kind() {
            let group_count = self.world.query::<&UiGroupBox>().iter(&self.world).count();
            
            let mut x_offset = (group_count % 3) as f32 * 250.0 + 50.0;
            let mut y_offset = (group_count / 3) as f32 * 200.0 + 450.0;
            
            {
                let mut grid_query = self.world.query::<&GridSettings>();
                if let Some(grid_settings) = grid_query.iter(&self.world).next() {
                    let snapped_pos = snap_to_grid(egui::Pos2::new(x_offset, y_offset), grid_settings.spacing_pixels);
                    x_offset = snapped_pos.x;
                    y_offset = snapped_pos.y;
                }
            }
    
            self.world.spawn(UiGroupBoxBundle {
                group_box: UiGroupBox {
                    label: format!("Group {}", group_count + 1),
                    enabled: true,
                    font_size: 14.0,
                    contained_widgets: Vec::new(),
                },
                tab: UiElementTab {
                    tab_kind: current_tab_kind,
                    position: 0,
                },
                position: UiElementPosition {
                    x: x_offset,
                    y: y_offset,
                },
                size: UiElementSize { width: 200.0, height: 150.0 },
                selected: UiElementSelected::default(),
                container: UiElementContainer { parent_group: None },
            });
    
            add_designer_log(&mut self.world, &format!("Added Group Box {} to tab", group_count + 1));
        }
    }
}

impl Drop for DesignerApp {
    fn drop(&mut self) {
        save_dock_layout(&self.dock_state);
    }
}

impl eframe::App for DesignerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update tab viewer world pointer
        self.tab_viewer.set_world(&mut self.world as *mut World);
        self.tab_viewer.set_codegen_state(&mut self.codegen_state as *mut events::CodeGenState);
        
        // Reset button clicks after a delay (visual feedback)
        reset_button_clicks(&mut self.world);
        
        // Update world snapshot for codegen thread (only when it needs it)
        {
            let new_snapshot = crate::codegen_thread::WorldSnapshot::from_world(&mut self.world);
            let mut snapshot_guard = self.world_snapshot.lock().unwrap();
            *snapshot_guard = Some(new_snapshot);
        }
        
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("🆕 New Project").clicked() {
                        self.world.clear_all();
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    ui.separator();
                    if ui.button("📁 Open...").clicked() {
                        // Handle open
                    }
                    if ui.button("💾 Save Project").clicked() {
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    ui.separator();
                    if ui.button("❌ Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("Edit", |ui| {
                    if ui.button("🔄 Clear Selections").clicked() {
                        clear_all_selections(&mut self.world);
                        add_designer_log(&mut self.world, "Cleared all selections");
                        ui.close_kind(egui::UiKind::Menu);
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("🖥️ Add Preview Tab").clicked() {
                        // Check if Preview tab already exists
                        let mut has_preview = false;
                        for (_surface_info, tab) in self.dock_state.iter_all_tabs() {
                            if matches!(tab.kind, TabKind::Preview) {
                                has_preview = true;
                                break;
                            }
                        }
                        
                        if !has_preview {
                            let new_tab = Tab { 
                                name: "Preview".to_string(), 
                                kind: TabKind::Preview, 
                                id: 5 
                            };
                            // Try to find the first available leaf to add the tab
                            if let Some((surface_index, node_index, _tab_index)) = self.dock_state.find_tab(&Tab { name: "Controls".to_string(), kind: TabKind::Controls, id: 2 }) {
                                self.dock_state.set_focused_node_and_surface((surface_index, node_index));
                                self.dock_state.push_to_focused_leaf(new_tab);
                            }
                            add_designer_log(&mut self.world, "Added Preview tab");
                        } else {
                            add_designer_log(&mut self.world, "Preview tab already exists");
                        }
                        ui.close_kind(egui::UiKind::Menu);
                    }
                });
                
                ui.menu_button("UI Elements", |ui| {
                    if ui.button("➕ Add Button").clicked() {
                        self.add_button_to_current_tab();
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    
                    if ui.button("📝 Add Text Input").clicked() {
                        self.add_text_input_to_current_tab();
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    
                    if ui.button("☑️ Add Checkbox").clicked() {
                        self.add_checkbox_to_current_tab();
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    
                    if ui.button("🔘 Add Radio Button").clicked() {
                        self.add_radio_button_to_current_tab();
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    
                    if ui.button("📦 Add Group Box").clicked() {
                        self.add_group_box_to_current_tab();
                        ui.close_kind(egui::UiKind::Menu);
                    }
                    
                    ui.separator();
                    if ui.button("🗑️ Clear All UI Elements").clicked() {
                        clear_all_ui_elements(&mut self.world);
                        add_designer_log(&mut self.world, "Cleared all UI elements");
                        ui.close_kind(egui::UiKind::Menu);
                    }
                });
            });
        });
        
        // Main UI
        egui::CentralPanel::default().show(ctx, |_ui| {
            // Render the docking area
            egui_dock::DockArea::new(&mut self.dock_state)
                .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
                .show(ctx, &mut self.tab_viewer);
        });
    }
}