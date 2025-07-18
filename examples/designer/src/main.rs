use eframe::{egui, App, Frame};
use bevy_ecs::prelude::*;
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};
use mobius_ecs::*;
use eframe::egui::ViewportBuilder;
use std::collections::HashMap;

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
    id: usize, // Unique identifier for each tab instance
}

#[derive(Clone, Debug)]
enum TabKind {
    MainWork,
    Settings,
    EventLogger,
    Controls,
    Inspector,
}

// ECS Components for UI Elements
#[derive(Component, Clone)]
struct UiButton {
    label: String,
    clicked: bool,
    enabled: bool,
    click_time: Option<std::time::Instant>,
    font_size: f32,
}

#[derive(Component, Clone)]
struct UiTextInput {
    label: String,
    value: String,
    enabled: bool,
    font_size: f32,
}

#[derive(Component, Clone)]
struct UiCheckbox {
    label: String,
    checked: bool,
    enabled: bool,
    font_size: f32,
}

#[derive(Component, Clone)]
struct UiRadioButton {
    label: String,
    selected: bool,
    enabled: bool,
    font_size: f32,
    group_id: String, // Radio buttons with same group_id are mutually exclusive
}

#[derive(Component, Clone)]
struct UiGroupBox {
    label: String,
    enabled: bool,
    font_size: f32,
    contained_widgets: Vec<Entity>, // Widgets inside this group box
}

#[derive(Component, Clone)]
struct UiElementTab {
    tab_kind: TabKind,
    position: usize, // Order in the tab
}

#[derive(Component, Clone)]
struct UiElementPosition {
    x: f32,
    y: f32,
}

#[derive(Component, Clone)]
struct UiElementDragging {
    offset_x: f32,
    offset_y: f32,
}

#[derive(Component, Clone, Default)]
struct UiElementSize {
    width: f32,
    height: f32,
}

#[derive(Component, Clone)]
struct UiElementContainer {
    parent_group: Option<Entity>, // The group box that contains this element
}

// Grid system components
#[derive(Component, Clone)]
struct GridSettings {
    enabled: bool,
    spacing_pixels: f32,  // Grid spacing in pixels
    dot_size: f32,       // Size of grid dots
    snap_enabled: bool,  // Whether to snap elements to grid
    show_grid: bool,     // Whether to visually show the grid
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            spacing_pixels: 10.0,  // Default 10 pixel grid
            dot_size: 0.5,
            snap_enabled: false,
            show_grid: false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum GridStatus {
    TooFine,
    TooCoarse, 
    Visible(f32),
}

// Group selection and distribution components
#[derive(Component, Clone, Default)]
struct UiElementSelected {
    selected: bool,
}

#[derive(Default)]
struct SelectionState {
    selecting: bool,
    start_pos: egui::Pos2,
    current_pos: egui::Pos2,
    selected_entities: Vec<Entity>,
}

#[derive(Default)]
struct DistributionSettings {
    vertical_spacing: f32,
    horizontal_spacing: f32,
}

// Bundle for UI elements
#[derive(Bundle)]
struct UiButtonBundle {
    button: UiButton,
    tab: UiElementTab,
    position: UiElementPosition,
    size: UiElementSize,
    selected: UiElementSelected,
}

#[derive(Bundle)]
struct UiTextInputBundle {
    input: UiTextInput,
    tab: UiElementTab,
    position: UiElementPosition,
    size: UiElementSize,
    selected: UiElementSelected,
}

#[derive(Bundle)]
struct UiCheckboxBundle {
    checkbox: UiCheckbox,
    tab: UiElementTab,
    position: UiElementPosition,
    size: UiElementSize,
    selected: UiElementSelected,
}

#[derive(Bundle)]
struct UiRadioButtonBundle {
    radio: UiRadioButton,
    tab: UiElementTab,
    position: UiElementPosition,
    size: UiElementSize,
    selected: UiElementSelected,
}

#[derive(Bundle)]
struct UiGroupBoxBundle {
    group_box: UiGroupBox,
    tab: UiElementTab,
    position: UiElementPosition,
    size: UiElementSize,
    selected: UiElementSelected,
    container: UiElementContainer,
}

pub struct MobiusToolWindowsDemo {
    world: World,
    app_entity: Option<Entity>,
    dock_state: DockState<Tab>,
    tab_viewer: MobiusTabViewer,
    tool_windows: ToolWindows,
    edit_mode: bool,
    next_tab_id: usize,
    tab_counts: std::collections::HashMap<String, usize>, // Track count per tab type
    renaming_entity: Option<Entity>,
    rename_buffer: String,
    resizing_entity: Option<Entity>,
    selection_state: SelectionState,
    distribution_settings: DistributionSettings,
    last_click_time: Option<std::time::Instant>,
}

#[derive(Default)]
struct MobiusTabViewer {
    world_ptr: Option<*mut World>,
    edit_mode: bool,
    renaming_entity: Option<Entity>,
    rename_buffer: String,
    resizing_entity: Option<Entity>,
}

impl MobiusTabViewer {
    fn with_world(&mut self, world: *mut World) -> &mut Self {
        self.world_ptr = Some(world);
        self
    }
    
    fn set_edit_mode(&mut self, edit_mode: bool) {
        self.edit_mode = edit_mode;
    }
    
    fn set_rename_state(&mut self, entity: Option<Entity>, buffer: String) {
        self.renaming_entity = entity;
        self.rename_buffer = buffer;
    }
    
    fn set_resize_state(&mut self, entity: Option<Entity>) {
        self.resizing_entity = entity;
    }
}

impl TabViewer for MobiusTabViewer {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        // Use the stored name which includes the unique suffix
        match tab.kind {
            TabKind::MainWork => format!("📋 {}", tab.name).into(),
            TabKind::Settings => format!("⚙️ {}", tab.name).into(),
            TabKind::EventLogger => format!("📜 {}", tab.name).into(),
            TabKind::Controls => format!("🎛️ {}", tab.name).into(),
            TabKind::Inspector => format!("🔍 {}", tab.name).into(),
        }
    }

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, tab: &mut Self::Tab) {
        if let Some(world_ptr) = self.world_ptr {
            let world = unsafe { &mut *world_ptr };
            
            // Draw grid first (so it appears underneath everything)
            {
                let mut grid_query = world.query::<&GridSettings>();
                if let Some(grid_settings) = grid_query.iter(world).next() {
                    draw_grid(ui, grid_settings);
                }
            }
            
            // Then render any dynamic UI elements for this tab
            let (renaming_entity, rename_buffer, resizing_entity) = render_dynamic_ui_elements(
                ui, 
                world, 
                &tab.kind, 
                self.edit_mode,
                self.renaming_entity,
                self.rename_buffer.clone(),
                self.resizing_entity
            );
            self.renaming_entity = renaming_entity;
            self.rename_buffer = rename_buffer;
            self.resizing_entity = resizing_entity;
            
            ui.separator();
            
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
            name: "Main Work Area".to_string(),
            kind: TabKind::MainWork,
            id: 0,
        }]);

        // Split the surface into multiple areas
        let surface = dock_state.main_surface_mut();
        
        // Create left panel
        let [left, _] = surface.split_left(NodeIndex::root(), 0.2, vec![Tab {
            name: "Controls".to_string(),
            kind: TabKind::Controls,
            id: 1,
        }]);
        
        // Add inspector below controls
        let [_, _below] = surface.split_below(left, 0.5, vec![Tab {
            name: "Inspector".to_string(),
            kind: TabKind::Inspector,
            id: 2,
        }]);
        
        // Split main area to add settings and logger
        let [_right_top, _] = surface.split_right(NodeIndex::root(), 0.25, vec![Tab {
            name: "Settings".to_string(),
            kind: TabKind::Settings,
            id: 3,
        }]);
        
        // Add event logger at the bottom
        let [_, _bottom] = surface.split_below(NodeIndex::root(), 0.7, vec![Tab {
            name: "Event Logger".to_string(),
            kind: TabKind::EventLogger,
            id: 4,
        }]);
        let mut world = World::new();
        
        // Add dynamic UI elements to the world
        add_ui_elements_to_world(&mut world);
        
        let mut tab_counts = HashMap::new();
        tab_counts.insert("MainWork".to_string(), 1);
        tab_counts.insert("Controls".to_string(), 1);
        tab_counts.insert("Inspector".to_string(), 1);
        tab_counts.insert("Settings".to_string(), 1);
        tab_counts.insert("EventLogger".to_string(), 1);
        
        Self {
            world,
            app_entity: None,
            dock_state,
            tab_viewer: MobiusTabViewer::default(),
            tool_windows: ToolWindows::default(),
            edit_mode: false,
            next_tab_id: 5, // Start after the initial tabs
            tab_counts,
            renaming_entity: None,
            rename_buffer: String::new(),
            resizing_entity: None,
            selection_state: SelectionState::default(),
            distribution_settings: DistributionSettings {
                vertical_spacing: 10.0,
                horizontal_spacing: 10.0,
            },
            last_click_time: None,
        }
        }
    }




// Grid rendering function with performance optimizations
fn draw_grid(ui: &mut egui::Ui, grid_settings: &GridSettings) {
    if !grid_settings.enabled || !grid_settings.show_grid {
        return;
    }

    let viewport_rect = ui.available_rect_before_wrap();
    let spacing = grid_settings.spacing_pixels;
    
    // Performance optimization: skip if grid too fine (< 5 pixels)
    if spacing < 5.0 {
        return;
    }
    
    // Performance optimization: skip if grid too coarse (> 50% of viewport)
    if spacing > viewport_rect.width() * 0.5 || spacing > viewport_rect.height() * 0.5 {
        return;
    }
    
    // Calculate grid bounds
    let start_x = (viewport_rect.min.x / spacing).floor() * spacing;
    let start_y = (viewport_rect.min.y / spacing).floor() * spacing;
    let end_x = viewport_rect.max.x;
    let end_y = viewport_rect.max.y;
    
    // Limit maximum number of grid points for performance
    let max_points = 10000;
    let estimated_points = ((end_x - start_x) / spacing) * ((end_y - start_y) / spacing);
    if estimated_points > max_points as f32 {
        return;
    }
    
    // Get painter for drawing
    let painter = ui.painter();
    
    // Adaptive opacity based on grid density
    let opacity = if spacing < 15.0 { 60 } else { 120 };
    let grid_color = egui::Color32::from_rgba_premultiplied(128, 128, 128, opacity);
    
    // Draw grid dots
    let mut y = start_y;
    while y <= end_y {
        let mut x = start_x;
        while x <= end_x {
            let grid_point = egui::Pos2::new(x, y);
            if viewport_rect.contains(grid_point) {
                painter.circle_filled(
                    grid_point,
                    grid_settings.dot_size,
                    grid_color,
                );
            }
            x += spacing;
        }
        y += spacing;
    }
}

fn get_grid_status(grid_settings: &GridSettings, viewport_rect: egui::Rect) -> GridStatus {
    let spacing = grid_settings.spacing_pixels;
    
    if spacing < 5.0 {
        GridStatus::TooFine
    } else if spacing > viewport_rect.width() * 0.5 || spacing > viewport_rect.height() * 0.5 {
        GridStatus::TooCoarse
    } else {
        GridStatus::Visible(spacing)
    }
}

fn snap_to_grid(position: egui::Pos2, grid_settings: &GridSettings) -> egui::Pos2 {
    if !grid_settings.enabled || !grid_settings.snap_enabled {
        return position;
    }
    
    let spacing = grid_settings.spacing_pixels;
    egui::Pos2::new(
        (position.x / spacing).round() * spacing,
        (position.y / spacing).round() * spacing,
    )
}

// Helper functions moved outside of struct to be accessible by TabViewer
fn add_test_log(world: &mut World) {
    let mut query = world.query::<&mut EventLoggerPanel>();
    if let Some(mut logger) = query.iter_mut(world).next() {
        // Insert at front for newest-first display
        logger.entries.insert(0, LogEntry {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            level: LogLevel::Info,
            message: "Test log entry added by user".to_string(),
        });
        
        // Keep within max entries limit
        let max_entries = logger.max_entries;
        if logger.entries.len() > max_entries {
            logger.entries.truncate(max_entries);
        }
    }
}

fn add_warning_log(world: &mut World) {
    let mut query = world.query::<&mut EventLoggerPanel>();
    if let Some(mut logger) = query.iter_mut(world).next() {
        // Insert at front for newest-first display
        logger.entries.insert(0, LogEntry {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            level: LogLevel::Warn,
            message: "This is a warning message".to_string(),
        });
        
        // Keep within max entries limit
        let max_entries = logger.max_entries;
        if logger.entries.len() > max_entries {
            logger.entries.truncate(max_entries);
        }
    }
}

fn add_error_log(world: &mut World) {
    let mut query = world.query::<&mut EventLoggerPanel>();
    if let Some(mut logger) = query.iter_mut(world).next() {
        // Insert at front for newest-first display
        logger.entries.insert(0, LogEntry {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            level: LogLevel::Error,
            message: "This is an error message".to_string(),
        });
        
        // Keep within max entries limit
        let max_entries = logger.max_entries;
        if logger.entries.len() > max_entries {
            logger.entries.truncate(max_entries);
        }
    }
}

impl App for MobiusToolWindowsDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Initialize FIRST - BEFORE ANYTHING ELSE!
        if self.app_entity.is_none() {
            let registry = MobiusTemplateRegistry::default();
            let mut commands = self.world.commands();
            self.app_entity = registry.spawn_from_template(&mut commands, "gerber_viewer");
            
            // Force flush commands to ensure all components are created immediately
            self.world.flush();
            
            // Verify SettingsPanel exists, if not create it manually
            let settings_count = self.world.query::<&SettingsPanel>().iter(&self.world).count();
            if settings_count == 0 {
                self.world.spawn(SettingsPanel {
                    global_units_mils: false,
                    user_timezone: None,
                    use_24_hour_clock: true,
                    interface_language: "English".to_string(),
                });
            }
            
            // Force correct GridSettings values
            let mut grid_query = self.world.query::<&mut GridSettings>();
            if let Some(mut grid_settings) = grid_query.iter_mut(&mut self.world).next() {
                // Force our desired defaults
                grid_settings.spacing_pixels = 10.0;
                grid_settings.dot_size = 0.5;
            } else {
                // Create with correct values if doesn't exist
                self.world.spawn(GridSettings {
                    enabled: false,
                    spacing_pixels: 10.0,
                    dot_size: 0.5,
                    snap_enabled: false,
                    show_grid: false,
                });
            }
        }
        
        // FORCE correct grid values EVERY FRAME - brute force fix
        {
            let mut grid_query = self.world.query::<&mut GridSettings>();
            if let Some(mut grid_settings) = grid_query.iter_mut(&mut self.world).next() {
                // FORCE the correct values every single frame
                grid_settings.spacing_pixels = 10.0;
                grid_settings.dot_size = 0.5;
            }
        }
        
        // Handle hotkey "G" for grid toggle GLOBALLY
        if ctx.input(|i| i.key_pressed(egui::Key::G)) {
            let mut grid_query = self.world.query::<&mut GridSettings>();
            if let Some(mut grid_settings) = grid_query.iter_mut(&mut self.world).next() {
                // Cycle through: Off -> Grid On -> Grid Off -> repeat
                if !grid_settings.enabled || !grid_settings.show_grid {
                    // Turn everything on
                    grid_settings.enabled = true;
                    grid_settings.show_grid = true;
                    grid_settings.snap_enabled = true;
                    add_designer_log(&mut self.world, "Grid & snap enabled (hotkey G)");
                } else {
                    // Turn everything off
                    grid_settings.show_grid = false;
                    grid_settings.snap_enabled = false;
                    add_designer_log(&mut self.world, "Grid & snap disabled (hotkey G)");
                }
            }
        }
        
        // Reset button clicks after a delay (visual feedback)
        reset_button_clicks(&mut self.world);
        
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
                    if ui.button("💾 Save Project").clicked() {
                        save_project_dialog(&mut self.world);
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("❌ Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("Windows", |ui| {
                    ui.label("Docked Tabs:");
                    if ui.button("📋 Add Main Work Tab").clicked() {
                        let count = self.tab_counts.entry("MainWork".to_string()).or_insert(0);
                        *count += 1;
                        let name = if *count == 1 {
                            "Main Work Area".to_string()
                        } else {
                            format!("Main Work Area {}", count)
                        };
                        self.dock_state.push_to_focused_leaf(Tab {
                            name,
                            kind: TabKind::MainWork,
                            id: self.next_tab_id,
                        });
                        self.next_tab_id += 1;
                    }
                    if ui.button("⚙️ Add Settings Tab").clicked() {
                        let count = self.tab_counts.entry("Settings".to_string()).or_insert(0);
                        *count += 1;
                        let name = if *count == 1 {
                            "Settings".to_string()
                        } else {
                            format!("Settings {}", count)
                        };
                        self.dock_state.push_to_focused_leaf(Tab {
                            name,
                            kind: TabKind::Settings,
                            id: self.next_tab_id,
                        });
                        self.next_tab_id += 1;
                    }
                    if ui.button("📜 Add Event Logger Tab").clicked() {
                        let count = self.tab_counts.entry("EventLogger".to_string()).or_insert(0);
                        *count += 1;
                        let name = if *count == 1 {
                            "Event Logger".to_string()
                        } else {
                            format!("Event Logger {}", count)
                        };
                        self.dock_state.push_to_focused_leaf(Tab {
                            name,
                            kind: TabKind::EventLogger,
                            id: self.next_tab_id,
                        });
                        self.next_tab_id += 1;
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
                            name: "Main Work Area".to_string(),
                            kind: TabKind::MainWork,
                            id: self.next_tab_id,
                        }]);
                        self.next_tab_id += 1;
                    }
                });
                
                ui.menu_button("UI Designer", |ui| {
                    // Edit mode toggle
                    if ui.checkbox(&mut self.edit_mode, "🔧 Edit Mode (Drag to reposition)").clicked() {
                        add_designer_log(&mut self.world, &format!("Edit mode {}", if self.edit_mode { "enabled" } else { "disabled" }));
                    }
                    
                    ui.separator();
                    
                    // Grid settings
                    ui.collapsing("📐 Grid Settings", |ui| {
                        // First, get grid settings values
                        let (mut enabled, mut show_grid, mut snap_enabled, mut spacing_pixels, mut dot_size, grid_status) = {
                            let mut grid_query = self.world.query::<&GridSettings>();
                            if let Some(grid_settings) = grid_query.iter(&self.world).next() {
                                let viewport_rect = ui.available_rect_before_wrap();
                                let grid_status = get_grid_status(grid_settings, viewport_rect);
                                (grid_settings.enabled, grid_settings.show_grid, grid_settings.snap_enabled, 
                                 grid_settings.spacing_pixels, grid_settings.dot_size, grid_status)
                            } else {
                                return; // No grid settings found
                            }
                        };
                        
                        let mut log_messages = Vec::new();
                        
                        // Grid enable/disable
                        if ui.checkbox(&mut enabled, "🔲 Enable Grid System").clicked() {
                            log_messages.push(format!("Grid system {}", if enabled { "enabled" } else { "disabled" }));
                        }
                        
                        ui.add_enabled(enabled, egui::Checkbox::new(&mut show_grid, "👁 Show Grid"));
                        ui.add_enabled(enabled, egui::Checkbox::new(&mut snap_enabled, "🧲 Snap to Grid"));
                        
                        ui.add_space(5.0);
                        
                        // Grid spacing
                        ui.horizontal(|ui| {
                            ui.label("Grid Spacing:");
                            if ui.add_enabled(enabled, egui::Slider::new(&mut spacing_pixels, 5.0..=100.0).suffix(" px")).changed() {
                                log_messages.push(format!("Grid spacing set to {:.1} pixels", spacing_pixels));
                            }
                        });
                        
                        // Dot size
                        ui.horizontal(|ui| {
                            ui.label("Dot Size:");
                            ui.add_enabled(enabled, egui::Slider::new(&mut dot_size, 0.5..=5.0).suffix(" px"));
                        });
                        
                        ui.add_space(5.0);
                        
                        // Grid status feedback
                        match grid_status {
                            GridStatus::TooFine => {
                                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "⚠️ Grid too fine (< 5px)");
                            }
                            GridStatus::TooCoarse => {
                                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "⚠️ Grid too coarse");
                            }
                            GridStatus::Visible(spacing) => {
                                ui.colored_label(egui::Color32::from_rgb(0, 255, 0), format!("✅ Grid visible ({:.1}px)", spacing));
                            }
                        }
                        
                        // Update grid settings if they changed
                        {
                            let mut grid_query = self.world.query::<&mut GridSettings>();
                            if let Some(mut grid_settings) = grid_query.iter_mut(&mut self.world).next() {
                                grid_settings.enabled = enabled;
                                grid_settings.show_grid = show_grid;
                                grid_settings.snap_enabled = snap_enabled;
                                grid_settings.spacing_pixels = spacing_pixels;
                                grid_settings.dot_size = dot_size;
                            }
                        }
                        
                        // Add log messages after grid settings update
                        for message in log_messages {
                            add_designer_log(&mut self.world, &message);
                        }
                    });
                    
                    ui.separator();
                    
                    // Group Selection and Distribution Tools
                    ui.collapsing("🎯 Selection & Distribution", |ui| {
                        // Selection info
                        let selected_count = self.selection_state.selected_entities.len();
                        if selected_count > 0 {
                            ui.colored_label(egui::Color32::from_rgb(100, 200, 255), format!("📌 {} items selected", selected_count));
                            
                            if ui.button("🚫 Clear Selection").clicked() {
                                clear_all_selections(&mut self.world);
                                self.selection_state.selected_entities.clear();
                                add_designer_log(&mut self.world, "Cleared selection");
                            }
                            
                            ui.separator();
                            
                            // Distribution controls
                            ui.label("Distribute Selected Items:");
                            ui.horizontal(|ui| {
                                ui.label("Vertical spacing:");
                                ui.add(egui::Slider::new(&mut self.distribution_settings.vertical_spacing, 0.0..=100.0).suffix("px"));
                            });
                            ui.horizontal(|ui| {
                                if ui.button("📏 Distribute Vertically").clicked() {
                                    distribute_items_vertically(&mut self.world, &self.selection_state.selected_entities, self.distribution_settings.vertical_spacing);
                                    add_designer_log(&mut self.world, &format!("Distributed {} items vertically with {}px spacing", selected_count, self.distribution_settings.vertical_spacing));
                                }
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("Horizontal spacing:");
                                ui.add(egui::Slider::new(&mut self.distribution_settings.horizontal_spacing, 0.0..=100.0).suffix("px"));
                            });
                            ui.horizontal(|ui| {
                                if ui.button("📐 Distribute Horizontally").clicked() {
                                    distribute_items_horizontally(&mut self.world, &self.selection_state.selected_entities, self.distribution_settings.horizontal_spacing);
                                    add_designer_log(&mut self.world, &format!("Distributed {} items horizontally with {}px spacing", selected_count, self.distribution_settings.horizontal_spacing));
                                }
                            });
                        } else {
                            ui.colored_label(egui::Color32::from_rgb(150, 150, 150), "💡 Select multiple items by dragging");
                            ui.colored_label(egui::Color32::from_rgb(150, 150, 150), "   Double-click to toggle edit mode");
                            ui.colored_label(egui::Color32::from_rgb(150, 150, 150), "   Press G to toggle grid & snap");
                        }
                    });
                    
                    ui.separator();
                    ui.label("Add UI Elements to Current Tab:");
                    ui.separator();
                    
                    if ui.button("➕ Add Button").clicked() {
                        add_button_to_current_tab(&mut self.world, &mut self.dock_state);
                        ui.close_menu();
                    }
                    
                    if ui.button("📝 Add Text Input").clicked() {
                        add_text_input_to_current_tab(&mut self.world, &mut self.dock_state);
                        ui.close_menu();
                    }
                    
                    if ui.button("☑️ Add Checkbox").clicked() {
                        add_checkbox_to_current_tab(&mut self.world, &mut self.dock_state);
                        ui.close_menu();
                    }
                    
                    if ui.button("🔘 Add Radio Button").clicked() {
                        add_radio_button_to_current_tab(&mut self.world, &mut self.dock_state);
                        ui.close_menu();
                    }
                    
                    if ui.button("📦 Add Group Box").clicked() {
                        add_group_box_to_current_tab(&mut self.world, &mut self.dock_state);
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    if ui.add_enabled(self.edit_mode, egui::Button::new("🗑️ Clear All UI Elements")).clicked() {
                        clear_all_ui_elements(&mut self.world);
                        ui.close_menu();
                    }
                    if !self.edit_mode {
                        ui.label("⚠️ Enable Edit Mode to clear elements");
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
                    ui.hyperlink_to("📖 GitHub", "https://github.com/saturn77/mobius-ecs");
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
                        if let Some(_entity_ref) = self.world.get_entity(entity) {
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

        // Handle multi-selection with drag rectangle in edit mode GLOBALLY
        if self.edit_mode {
            // Create a full-screen overlay for selection
            egui::Area::new("selection_overlay".into())
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.allocate_response(ctx.screen_rect().size(), egui::Sense::click_and_drag());
                    self.handle_multi_selection(ui);
                });
        }
        
        // Central panel with docking area
        egui::CentralPanel::default().show(ctx, |ui| {
            
            // Handle double-click detection for edit mode toggle
            if ui.input(|i| i.pointer.any_click()) {
                let now = std::time::Instant::now();
                if let Some(last_click) = self.last_click_time {
                    if now.duration_since(last_click).as_millis() < 500 { // 500ms double-click window
                        self.edit_mode = !self.edit_mode;
                        add_designer_log(&mut self.world, &format!("Edit mode {} (double-click)", if self.edit_mode { "enabled" } else { "disabled" }));
                    }
                }
                self.last_click_time = Some(now);
            }
            
            // Update the tab viewer with the world pointer and edit mode
            self.tab_viewer.with_world(&mut self.world as *mut World);
            self.tab_viewer.set_edit_mode(self.edit_mode);
            self.tab_viewer.set_rename_state(self.renaming_entity, self.rename_buffer.clone());
            self.tab_viewer.set_resize_state(self.resizing_entity);
            
            // Show the docking area
            DockArea::new(&mut self.dock_state)
                .style(Style::from_egui(ui.ctx().style().as_ref()))
                .show_inside(ui, &mut self.tab_viewer);
                
            // Update state from tab viewer
            self.renaming_entity = self.tab_viewer.renaming_entity;
            self.rename_buffer = self.tab_viewer.rename_buffer.clone();
            self.resizing_entity = self.tab_viewer.resizing_entity;
        });
    }
}

impl MobiusToolWindowsDemo {
    fn handle_multi_selection(&mut self, ui: &mut egui::Ui) {
        let input = ui.input(|i| i.clone());
        let available_rect = ui.available_rect_before_wrap();
        
        // Handle mouse input for selection rectangle
        if input.pointer.primary_down() {
            if let Some(pos) = input.pointer.interact_pos() {
                if !self.selection_state.selecting {
                    // Start selection if clicking in empty area (not on a widget)
                    let clicking_on_widget = self.is_clicking_on_widget(pos);
                    if !clicking_on_widget {
                        self.selection_state.selecting = true;
                        self.selection_state.start_pos = pos;
                        self.selection_state.current_pos = pos;
                        
                        // Clear current selection if not holding Ctrl
                        if !input.modifiers.ctrl {
                            self.clear_all_selections();
                        }
                    }
                } else {
                    // Update selection rectangle
                    self.selection_state.current_pos = pos;
                }
            }
        } else if self.selection_state.selecting {
            // Finish selection
            self.finish_selection();
            self.selection_state.selecting = false;
        }
        
        // Draw selection rectangle
        if self.selection_state.selecting {
            let min_x = self.selection_state.start_pos.x.min(self.selection_state.current_pos.x);
            let max_x = self.selection_state.start_pos.x.max(self.selection_state.current_pos.x);
            let min_y = self.selection_state.start_pos.y.min(self.selection_state.current_pos.y);
            let max_y = self.selection_state.start_pos.y.max(self.selection_state.current_pos.y);
            
            let selection_rect = egui::Rect::from_min_max(
                egui::pos2(min_x, min_y),
                egui::pos2(max_x, max_y)
            );
            
            let painter = ui.painter();
            // Draw selection rectangle with more visible colors (like CopperForge style)
            painter.rect_filled(
                selection_rect,
                0.0,
                egui::Color32::from_rgba_premultiplied(0, 120, 255, 40)  // Blue semi-transparent fill
            );
            painter.rect_stroke(
                selection_rect,
                0.0,
                egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 120, 255)),  // Thicker blue border
                egui::StrokeKind::Outside
            );
        }
        
        // Handle Escape key to cancel selection
        if input.key_pressed(egui::Key::Escape) {
            self.selection_state.selecting = false;
            self.clear_all_selections();
        }
    }
    
    fn is_clicking_on_widget(&mut self, pos: egui::Pos2) -> bool {
        // Check if clicking on any UI element
        let mut query = self.world.query::<(&UiElementPosition, &UiElementSize)>();
        for (element_pos, element_size) in query.iter(&self.world) {
            let element_rect = egui::Rect::from_min_size(
                egui::pos2(element_pos.x, element_pos.y),
                egui::vec2(element_size.width.max(100.0), element_size.height.max(25.0))
            );
            
            if element_rect.contains(pos) {
                return true;
            }
        }
        false
    }
    
    fn clear_all_selections(&mut self) {
        // Clear selection from all UI elements
        let entities_to_update: Vec<Entity> = {
            let mut query = self.world.query::<(Entity, &mut UiElementSelected)>();
            query.iter_mut(&mut self.world)
                .filter(|(_, selected)| selected.selected)
                .map(|(entity, _)| entity)
                .collect()
        };
        
        for entity in entities_to_update {
            if let Some(mut selected) = self.world.get_mut::<UiElementSelected>(entity) {
                selected.selected = false;
            }
        }
        
        self.selection_state.selected_entities.clear();
    }
    
    fn finish_selection(&mut self) {
        let min_x = self.selection_state.start_pos.x.min(self.selection_state.current_pos.x);
        let max_x = self.selection_state.start_pos.x.max(self.selection_state.current_pos.x);
        let min_y = self.selection_state.start_pos.y.min(self.selection_state.current_pos.y);
        let max_y = self.selection_state.start_pos.y.max(self.selection_state.current_pos.y);
        
        let selection_rect = egui::Rect::from_min_max(
            egui::pos2(min_x, min_y),
            egui::pos2(max_x, max_y)
        );
        
        // Only proceed if the selection rectangle has meaningful size
        if selection_rect.width() > 5.0 && selection_rect.height() > 5.0 {
            // Find all UI elements that intersect with the selection rectangle
            let mut entities_to_select = Vec::new();
            
            // Check buttons
            {
                let mut query = self.world.query::<(Entity, &UiElementPosition, &UiElementSize)>();
                for (entity, pos, size) in query.iter(&self.world) {
                    let element_rect = egui::Rect::from_min_size(
                        egui::pos2(pos.x, pos.y),
                        egui::vec2(size.width.max(100.0), size.height.max(25.0))
                    );
                    
                    if selection_rect.intersects(element_rect) {
                        entities_to_select.push(entity);
                    }
                }
            }
            
            // Update selection state
            for entity in entities_to_select {
                if let Some(mut selected) = self.world.get_mut::<UiElementSelected>(entity) {
                    selected.selected = true;
                    if !self.selection_state.selected_entities.contains(&entity) {
                        self.selection_state.selected_entities.push(entity);
                    }
                }
            }
            
            if !self.selection_state.selected_entities.is_empty() {
                add_designer_log(&mut self.world, &format!("Selected {} elements", self.selection_state.selected_entities.len()));
            }
        }
    }
}

// System to render dynamic UI elements for a specific tab
fn render_dynamic_ui_elements(
    ui: &mut egui::Ui, 
    world: &mut World, 
    tab_kind: &TabKind, 
    edit_mode: bool,
    mut renaming_entity: Option<Entity>,
    mut rename_buffer: String,
    mut resizing_entity: Option<Entity>
) -> (Option<Entity>, String, Option<Entity>) {
    // Use Area for absolute positioning
    let available_rect = ui.available_rect_before_wrap();
    
    // Store updates to apply after iteration
    let mut position_updates: Vec<(Entity, egui::Pos2)> = Vec::new();
    let mut button_updates: Vec<(Entity, bool)> = Vec::new();
    let mut text_updates: Vec<(Entity, String)> = Vec::new();
    let mut checkbox_updates: Vec<(Entity, bool)> = Vec::new();
    let mut radio_updates: Vec<(Entity, bool)> = Vec::new();
    let mut size_updates: Vec<(Entity, egui::Vec2)> = Vec::new();
    let mut log_messages = Vec::new();
    
    // Show mode indicator and grid status
    ui.horizontal(|ui| {
        if edit_mode {
            ui.label("🔧 Edit Mode - Drag elements to reposition");
        } else {
            ui.label("👁 View Mode - Right-click elements for options");
        }
        
        // Show grid status
        let mut grid_query = world.query::<&GridSettings>();
        if let Some(grid_settings) = grid_query.iter(world).next() {
            if grid_settings.enabled {
                ui.separator();
                if grid_settings.show_grid {
                    ui.colored_label(egui::Color32::from_rgb(0, 255, 0), "📐 Grid ON");
                } else {
                    ui.colored_label(egui::Color32::from_rgb(128, 128, 128), "📐 Grid (Hidden)");
                }
                if grid_settings.snap_enabled {
                    ui.colored_label(egui::Color32::from_rgb(0, 255, 255), "🧲 Snap ON");
                }
                ui.colored_label(egui::Color32::from_rgb(150, 150, 150), "(Press G)");
            } else {
                ui.separator();
                ui.colored_label(egui::Color32::from_rgb(100, 100, 100), "📐 Grid OFF (Press G)");
            }
        }
    });
    
    // Query and render buttons
    {
        let mut query = world.query::<(Entity, &UiButton, &UiElementTab, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
        let buttons: Vec<_> = query.iter(world)
            .filter(|(_, _, tab, _, _, _)| std::mem::discriminant(&tab.tab_kind) == std::mem::discriminant(tab_kind))
            .map(|(e, b, t, p, s, sel)| (e, b.clone(), t.clone(), p.clone(), s.clone(), sel.clone()))
            .collect();
        
        for (entity, button, _, position, size, selected) in buttons {
            let id = ui.id().with(entity);
            
            egui::Area::new(id)
                .fixed_pos(egui::pos2(
                    available_rect.min.x + position.x,
                    available_rect.min.y + position.y
                ))
                .show(ui.ctx(), |ui| {
                    let frame = if edit_mode {
                        let (fill_color, stroke_color) = if selected.selected {
                            // Highlighted colors for selected elements
                            (egui::Color32::from_rgba_premultiplied(255, 255, 100, 40), 
                             egui::Color32::from_rgb(255, 200, 0))
                        } else {
                            // Normal edit mode colors
                            (egui::Color32::from_rgba_premultiplied(255, 255, 255, 20), 
                             egui::Color32::from_rgb(100, 100, 255))
                        };
                        
                        egui::Frame::none()
                            .fill(fill_color)
                            .stroke(egui::Stroke::new(if selected.selected { 2.0 } else { 1.0 }, stroke_color))
                            .rounding(4.0)
                            .inner_margin(4.0)
                    } else {
                        egui::Frame::none()
                    };
                    
                    frame.show(ui, |ui| {
                        // Check if we're renaming this button
                        if renaming_entity == Some(entity) {
                            ui.horizontal(|ui| {
                                let response = ui.text_edit_singleline(&mut rename_buffer);
                                if response.lost_focus() {
                                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                        // Apply the rename
                                        if let Some(mut button) = world.get_mut::<UiButton>(entity) {
                                            button.label = rename_buffer.clone();
                                            add_designer_log(world, &format!("Renamed button to '{}'", rename_buffer));
                                        }
                                    }
                                    renaming_entity = None;
                                    rename_buffer.clear();
                                }
                                response.request_focus();
                            });
                        } else {
                            let mut label = button.label.clone();
                            if button.clicked {
                                label = format!("✓ {}", label);
                            }
                            
                            let font_id = egui::FontId::proportional(button.font_size);
                            
                            // Use fixed size if we have one, otherwise let it auto-size
                            let button_response = if size.width > 0.0 && size.height > 0.0 {
                                // Use allocated size for manual sizing
                                let (response, painter) = ui.allocate_painter(
                                    egui::vec2(size.width, size.height), 
                                    egui::Sense::click()
                                );
                                
                                // Draw the button manually
                                let visuals = ui.style().interact(&response);
                                let rect = response.rect;
                                
                                // Draw button background
                                painter.rect(
                                    rect,
                                    visuals.rounding(),
                                    visuals.bg_fill,
                                    visuals.bg_stroke,
                                    egui::StrokeKind::Outside,
                                );
                                
                                // Draw button text
                                painter.text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    &label,
                                    font_id,
                                    visuals.text_color(),
                                );
                                
                                response
                            } else {
                                // Auto-size button
                                let response = ui.add_enabled(button.enabled, 
                                    egui::Button::new(egui::RichText::new(&label).font(font_id)));
                                
                                // Update size only if not manually sized
                                if response.rect.size() != egui::vec2(size.width, size.height) {
                                    size_updates.push((entity, response.rect.size()));
                                }
                                
                                response
                            };
                            
                            // Show resize handles in edit mode
                            if edit_mode && resizing_entity == Some(entity) {
                                draw_resize_handles(ui, button_response.rect, &mut size_updates, entity);
                            }
                            
                            // Right-click context menu
                            button_response.context_menu(|ui| {
                                if ui.button("✏️ Rename").clicked() {
                                    renaming_entity = Some(entity);
                                    rename_buffer = button.label.clone();
                                    ui.close_menu();
                                }
                                ui.separator();
                                
                                ui.menu_button("🔤 Font Size", |ui| {
                                    for size in [12.0, 14.0, 16.0, 18.0, 20.0, 24.0] {
                                        if ui.button(format!("{}px", size)).clicked() {
                                            if let Some(mut button) = world.get_mut::<UiButton>(entity) {
                                                button.font_size = size;
                                                let label = button.label.clone();
                                                drop(button);
                                                add_designer_log(world, &format!("Changed font size of '{}' to {}px", label, size));
                                            }
                                            ui.close_menu();
                                        }
                                    }
                                });
                                
                                ui.separator();
                                let resize_text = if resizing_entity == Some(entity) { "🔒 Stop Resize" } else { "📏 Resize" };
                                if ui.button(resize_text).clicked() {
                                    resizing_entity = if resizing_entity == Some(entity) { None } else { Some(entity) };
                                    ui.close_menu();
                                }
                                
                                ui.separator();
                                if ui.button("🗑️ Delete").clicked() {
                                    world.despawn(entity);
                                    add_designer_log(world, &format!("Deleted button '{}'", button.label));
                                    ui.close_menu();
                                }
                                ui.separator();
                                let enabled_text = if button.enabled { "❌ Disable" } else { "✅ Enable" };
                                if ui.button(enabled_text).clicked() {
                                    if let Some(mut button) = world.get_mut::<UiButton>(entity) {
                                        button.enabled = !button.enabled;
                                        let label = button.label.clone();
                                        let status = if button.enabled { "enabled" } else { "disabled" };
                                        drop(button); // Release the borrow
                                        add_designer_log(world, &format!("Button '{}' {}", label, status));
                                    }
                                    ui.close_menu();
                                }
                            });
                            
                            // Handle dragging in edit mode
                            if edit_mode {
                                let drag_response = button_response.interact(egui::Sense::drag());
                                if drag_response.dragged() {
                                    let mut new_pos = drag_response.interact_pointer_pos().unwrap() - drag_response.drag_delta();
                                    
                                    // Apply grid snapping if enabled
                                    {
                                        let mut grid_query = world.query::<&GridSettings>();
                                        if let Some(grid_settings) = grid_query.iter(world).next() {
                                            new_pos = snap_to_grid(new_pos, grid_settings);
                                        }
                                    }
                                    
                                    position_updates.push((entity, new_pos - available_rect.min.to_vec2()));
                                }
                                
                                // Visual feedback when hovering in edit mode
                                if drag_response.hovered() {
                                    ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
                                }
                            } else if button_response.clicked() {
                                button_updates.push((entity, true));
                            }
                        }
                    });
                });
        }
    }
    
    // Query and render text inputs
    {
        let mut query = world.query::<(Entity, &UiTextInput, &UiElementTab, &UiElementPosition, &UiElementSize)>();
        let inputs: Vec<_> = query.iter(world)
            .filter(|(_, _, tab, _, _)| std::mem::discriminant(&tab.tab_kind) == std::mem::discriminant(tab_kind))
            .map(|(e, i, t, p, s)| (e, i.clone(), t.clone(), p.clone(), s.clone()))
            .collect();
        
        for (entity, input, _, position, size) in inputs {
            let id = ui.id().with(entity);
            
            egui::Area::new(id)
                .fixed_pos(egui::pos2(
                    available_rect.min.x + position.x,
                    available_rect.min.y + position.y
                ))
                .show(ui.ctx(), |ui| {
                    let frame = if edit_mode {
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgba_premultiplied(255, 255, 255, 20))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 255)))
                            .rounding(4.0)
                            .inner_margin(4.0)
                    } else {
                        egui::Frame::none()
                    };
                    
                    frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Check if we're renaming this input's label
                            if renaming_entity == Some(entity) {
                                let response = ui.text_edit_singleline(&mut rename_buffer);
                                ui.label(":");
                                if response.lost_focus() {
                                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                        // Apply the rename
                                        if let Some(mut input) = world.get_mut::<UiTextInput>(entity) {
                                            input.label = format!("{}:", rename_buffer.trim_end_matches(':'));
                                            let label = input.label.clone();
                                            drop(input); // Release the borrow
                                            add_designer_log(world, &format!("Renamed input to '{}'", label));
                                        }
                                    }
                                    renaming_entity = None;
                                    rename_buffer.clear();
                                }
                                response.request_focus();
                            } else {
                                ui.label(&input.label);
                            }
                            
                            let mut value = input.value.clone();
                            let response = ui.add_enabled(input.enabled && !edit_mode, 
                                egui::TextEdit::singleline(&mut value).desired_width(100.0));
                            
                            if response.changed() && !edit_mode {
                                text_updates.push((entity, value));
                            }
                            
                            // Right-click context menu on the whole horizontal area
                            let full_response = ui.interact(ui.min_rect(), id.with("menu"), egui::Sense::click());
                            full_response.context_menu(|ui| {
                                if ui.button("✏️ Rename Label").clicked() {
                                    renaming_entity = Some(entity);
                                    rename_buffer = input.label.trim_end_matches(':').to_string();
                                    ui.close_menu();
                                }
                                ui.separator();
                                if ui.button("🗑️ Delete").clicked() {
                                    world.despawn(entity);
                                    add_designer_log(world, &format!("Deleted input '{}'", input.label));
                                    ui.close_menu();
                                }
                                ui.separator();
                                let enabled_text = if input.enabled { "❌ Disable" } else { "✅ Enable" };
                                if ui.button(enabled_text).clicked() {
                                    if let Some(mut input) = world.get_mut::<UiTextInput>(entity) {
                                        input.enabled = !input.enabled;
                                        let label = input.label.clone();
                                        let status = if input.enabled { "enabled" } else { "disabled" };
                                        drop(input); // Release the borrow
                                        add_designer_log(world, &format!("Input '{}' {}", label, status));
                                    }
                                    ui.close_menu();
                                }
                            });
                            
                            // Handle dragging in edit mode
                            if edit_mode {
                                let full_response = ui.interact(
                                    ui.min_rect(), 
                                    id.with("drag"), 
                                    egui::Sense::drag()
                                );
                                
                                if full_response.dragged() {
                                    let mut new_pos = full_response.interact_pointer_pos().unwrap() - full_response.drag_delta();
                                    
                                    // Apply grid snapping if enabled
                                    {
                                        let mut grid_query = world.query::<&GridSettings>();
                                        if let Some(grid_settings) = grid_query.iter(world).next() {
                                            new_pos = snap_to_grid(new_pos, grid_settings);
                                        }
                                    }
                                    
                                    position_updates.push((entity, new_pos - available_rect.min.to_vec2()));
                                }
                                
                                if full_response.hovered() {
                                    ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
                                }
                            }
                        });
                        
                        // Update size
                        let rect = ui.min_rect();
                        if rect.size() != egui::vec2(size.width, size.height) {
                            size_updates.push((entity, rect.size()));
                        }
                    });
                });
        }
    }
    
    // Query and render checkboxes
    {
        let mut query = world.query::<(Entity, &UiCheckbox, &UiElementTab, &UiElementPosition, &UiElementSize)>();
        let checkboxes: Vec<_> = query.iter(world)
            .filter(|(_, _, tab, _, _)| std::mem::discriminant(&tab.tab_kind) == std::mem::discriminant(tab_kind))
            .map(|(e, c, t, p, s)| (e, c.clone(), t.clone(), p.clone(), s.clone()))
            .collect();
        
        for (entity, checkbox, _, position, size) in checkboxes {
            let id = ui.id().with(entity);
            
            egui::Area::new(id)
                .fixed_pos(egui::pos2(
                    available_rect.min.x + position.x,
                    available_rect.min.y + position.y
                ))
                .show(ui.ctx(), |ui| {
                    let frame = if edit_mode {
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgba_premultiplied(255, 255, 255, 20))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 255)))
                            .corner_radius(4.0)
                            .inner_margin(4.0)
                    } else {
                        egui::Frame::none()
                    };
                    
                    frame.show(ui, |ui| {
                        let mut checked = checkbox.checked;
                        let font_id = egui::FontId::proportional(checkbox.font_size);
                        let response = ui.add_enabled(checkbox.enabled && !edit_mode, 
                            egui::Checkbox::new(&mut checked, egui::RichText::new(&checkbox.label).font(font_id)));
                        
                        if response.changed() && !edit_mode {
                            checkbox_updates.push((entity, checked));
                        }
                        
                        // Right-click context menu
                        response.context_menu(|ui| {
                            if ui.button("✏️ Rename").clicked() {
                                renaming_entity = Some(entity);
                                rename_buffer = checkbox.label.clone();
                                ui.close_menu();
                            }
                            ui.separator();
                            
                            ui.menu_button("🔤 Font Size", |ui| {
                                for size in [12.0, 14.0, 16.0, 18.0, 20.0, 24.0] {
                                    if ui.button(format!("{}px", size)).clicked() {
                                        if let Some(mut checkbox) = world.get_mut::<UiCheckbox>(entity) {
                                            checkbox.font_size = size;
                                            let label = checkbox.label.clone();
                                            drop(checkbox);
                                            add_designer_log(world, &format!("Changed font size of checkbox '{}' to {}px", label, size));
                                        }
                                        ui.close_menu();
                                    }
                                }
                            });
                            
                            ui.separator();
                            if ui.button("🗑️ Delete").clicked() {
                                world.despawn(entity);
                                add_designer_log(world, &format!("Deleted checkbox '{}'", checkbox.label));
                                ui.close_menu();
                            }
                            ui.separator();
                            let enabled_text = if checkbox.enabled { "❌ Disable" } else { "✅ Enable" };
                            if ui.button(enabled_text).clicked() {
                                if let Some(mut checkbox) = world.get_mut::<UiCheckbox>(entity) {
                                    checkbox.enabled = !checkbox.enabled;
                                    let label = checkbox.label.clone();
                                    let status = if checkbox.enabled { "enabled" } else { "disabled" };
                                    drop(checkbox);
                                    add_designer_log(world, &format!("Checkbox '{}' {}", label, status));
                                }
                                ui.close_menu();
                            }
                        });
                        
                        // Handle dragging in edit mode
                        if edit_mode {
                            let drag_response = ui.interact(ui.min_rect(), id.with("drag"), egui::Sense::drag());
                            if drag_response.dragged() {
                                let mut new_pos = drag_response.interact_pointer_pos().unwrap() - drag_response.drag_delta();
                                
                                // Apply grid snapping if enabled
                                {
                                    let mut grid_query = world.query::<&GridSettings>();
                                    if let Some(grid_settings) = grid_query.iter(world).next() {
                                        new_pos = snap_to_grid(new_pos, grid_settings);
                                    }
                                }
                                
                                position_updates.push((entity, new_pos - available_rect.min.to_vec2()));
                            }
                            if drag_response.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
                            }
                        }
                        
                        // Update size
                        let rect = ui.min_rect();
                        if rect.size() != egui::vec2(size.width, size.height) {
                            size_updates.push((entity, rect.size()));
                        }
                    });
                });
        }
    }
    
    // Query and render radio buttons
    {
        let mut query = world.query::<(Entity, &UiRadioButton, &UiElementTab, &UiElementPosition, &UiElementSize)>();
        let radios: Vec<_> = query.iter(world)
            .filter(|(_, _, tab, _, _)| std::mem::discriminant(&tab.tab_kind) == std::mem::discriminant(tab_kind))
            .map(|(e, r, t, p, s)| (e, r.clone(), t.clone(), p.clone(), s.clone()))
            .collect();
        
        for (entity, radio, _, position, size) in radios {
            let id = ui.id().with(entity);
            
            egui::Area::new(id)
                .fixed_pos(egui::pos2(
                    available_rect.min.x + position.x,
                    available_rect.min.y + position.y
                ))
                .show(ui.ctx(), |ui| {
                    let frame = if edit_mode {
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgba_premultiplied(255, 255, 255, 20))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 255)))
                            .corner_radius(4.0)
                            .inner_margin(4.0)
                    } else {
                        egui::Frame::none()
                    };
                    
                    frame.show(ui, |ui| {
                        let mut selected = radio.selected;
                        let font_id = egui::FontId::proportional(radio.font_size);
                        let response = ui.add_enabled(radio.enabled && !edit_mode, 
                            egui::RadioButton::new(selected, egui::RichText::new(&radio.label).font(font_id)));
                        
                        if response.clicked() && !edit_mode {
                            radio_updates.push((entity, true));
                            // Deselect other radio buttons in the same group
                            let group_id = radio.group_id.clone();
                            let mut other_radios = world.query::<(Entity, &mut UiRadioButton)>();
                            for (other_entity, mut other_radio) in other_radios.iter_mut(world) {
                                if other_entity != entity && other_radio.group_id == group_id {
                                    other_radio.selected = false;
                                }
                            }
                        }
                        
                        // Right-click context menu
                        response.context_menu(|ui| {
                            if ui.button("✏️ Rename").clicked() {
                                renaming_entity = Some(entity);
                                rename_buffer = radio.label.clone();
                                ui.close_menu();
                            }
                            ui.separator();
                            
                            ui.menu_button("🔤 Font Size", |ui| {
                                for size in [12.0, 14.0, 16.0, 18.0, 20.0, 24.0] {
                                    if ui.button(format!("{}px", size)).clicked() {
                                        if let Some(mut radio) = world.get_mut::<UiRadioButton>(entity) {
                                            radio.font_size = size;
                                            let label = radio.label.clone();
                                            drop(radio);
                                            add_designer_log(world, &format!("Changed font size of radio '{}' to {}px", label, size));
                                        }
                                        ui.close_menu();
                                    }
                                }
                            });
                            
                            ui.separator();
                            if ui.button("🗑️ Delete").clicked() {
                                world.despawn(entity);
                                add_designer_log(world, &format!("Deleted radio button '{}'", radio.label));
                                ui.close_menu();
                            }
                            ui.separator();
                            let enabled_text = if radio.enabled { "❌ Disable" } else { "✅ Enable" };
                            if ui.button(enabled_text).clicked() {
                                if let Some(mut radio) = world.get_mut::<UiRadioButton>(entity) {
                                    radio.enabled = !radio.enabled;
                                    let label = radio.label.clone();
                                    let status = if radio.enabled { "enabled" } else { "disabled" };
                                    drop(radio);
                                    add_designer_log(world, &format!("Radio button '{}' {}", label, status));
                                }
                                ui.close_menu();
                            }
                        });
                        
                        // Handle dragging in edit mode
                        if edit_mode {
                            let drag_response = ui.interact(ui.min_rect(), id.with("drag"), egui::Sense::drag());
                            if drag_response.dragged() {
                                let mut new_pos = drag_response.interact_pointer_pos().unwrap() - drag_response.drag_delta();
                                
                                // Apply grid snapping if enabled
                                {
                                    let mut grid_query = world.query::<&GridSettings>();
                                    if let Some(grid_settings) = grid_query.iter(world).next() {
                                        new_pos = snap_to_grid(new_pos, grid_settings);
                                    }
                                }
                                
                                position_updates.push((entity, new_pos - available_rect.min.to_vec2()));
                            }
                            if drag_response.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
                            }
                        }
                        
                        // Update size
                        let rect = ui.min_rect();
                        if rect.size() != egui::vec2(size.width, size.height) {
                            size_updates.push((entity, rect.size()));
                        }
                    });
                });
        }
    }
    
    // Query and render group boxes
    {
        let mut query = world.query::<(Entity, &UiGroupBox, &UiElementTab, &UiElementPosition, &UiElementSize)>();
        let group_boxes: Vec<_> = query.iter(world)
            .filter(|(_, _, tab, _, _)| std::mem::discriminant(&tab.tab_kind) == std::mem::discriminant(tab_kind))
            .map(|(e, g, t, p, s)| (e, g.clone(), t.clone(), p.clone(), s.clone()))
            .collect();
        
        for (entity, group_box, _, position, size) in group_boxes {
            let id = ui.id().with(entity);
            
            egui::Area::new(id)
                .fixed_pos(egui::pos2(
                    available_rect.min.x + position.x,
                    available_rect.min.y + position.y
                ))
                .show(ui.ctx(), |ui| {
                    let group_frame = if edit_mode {
                        egui::Frame::group(ui.style())
                            .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 255, 100)))
                            .inner_margin(6.0)
                    } else {
                        egui::Frame::group(ui.style())
                    };
                    
                    group_frame.show(ui, |ui| {
                        // Group box with fixed size
                        ui.set_min_size(egui::vec2(size.width.max(100.0), size.height.max(80.0)));
                        ui.set_max_size(egui::vec2(size.width.max(100.0), size.height.max(80.0)));
                        
                        ui.vertical(|ui| {
                            // Group box title - handle renaming
                            if renaming_entity == Some(entity) {
                                ui.horizontal(|ui| {
                                    let response = ui.text_edit_singleline(&mut rename_buffer);
                                    if response.lost_focus() {
                                        if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                            // Apply the rename
                                            if let Some(mut group_box_mut) = world.get_mut::<UiGroupBox>(entity) {
                                                group_box_mut.label = rename_buffer.clone();
                                                add_designer_log(world, &format!("Renamed group box to '{}'", rename_buffer));
                                            }
                                        }
                                        renaming_entity = None;
                                        rename_buffer.clear();
                                    }
                                    response.request_focus();
                                });
                            } else {
                                let font_id = egui::FontId::proportional(group_box.font_size);
                                ui.heading(egui::RichText::new(&group_box.label).font(font_id));
                            }
                            ui.separator();
                            
                            // Content area for contained widgets
                            ui.allocate_space(egui::vec2(
                                size.width.max(100.0) - 20.0, 
                                size.height.max(80.0) - 40.0
                            ));
                        });
                        
                        // Handle dragging in edit mode
                        if edit_mode {
                            let drag_response = ui.interact(ui.min_rect(), ui.id(), egui::Sense::drag());
                            if drag_response.dragged() {
                                let mut new_pos = drag_response.interact_pointer_pos().unwrap() - drag_response.drag_delta();
                                
                                // Apply grid snapping if enabled
                                {
                                    let mut grid_query = world.query::<&GridSettings>();
                                    if let Some(grid_settings) = grid_query.iter(world).next() {
                                        new_pos = snap_to_grid(new_pos, grid_settings);
                                    }
                                }
                                
                                position_updates.push((entity, new_pos - available_rect.min.to_vec2()));
                                
                                // TODO: Move contained widgets with the group box
                            }
                            if drag_response.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
                            }
                        }
                        
                        // Right-click context menu in edit mode
                        if edit_mode {
                            ui.interact(ui.min_rect(), ui.id(), egui::Sense::click()).context_menu(|ui| {
                                if ui.button("✏️ Rename Group Box").clicked() {
                                    renaming_entity = Some(entity);
                                    rename_buffer = group_box.label.clone();
                                    ui.close_menu();
                                }
                                ui.separator();
                                if ui.button("🗑️ Delete Group Box").clicked() {
                                    // TODO: Remove contained widgets or move them out
                                    world.despawn(entity);
                                    add_designer_log(world, &format!("Deleted group box '{}'", group_box.label));
                                    ui.close_menu();
                                }
                            });
                        }
                    });
                });
        }
    }
    
    // Apply position updates
    for (entity, new_pos) in position_updates {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity) {
            pos.x = new_pos.x;
            pos.y = new_pos.y;
        }
    }
    
    // Apply size updates
    for (entity, new_size) in size_updates {
        if let Some(mut size) = world.get_mut::<UiElementSize>(entity) {
            size.width = new_size.x;
            size.height = new_size.y;
        }
    }
    
    // Apply button updates
    for (entity, clicked) in button_updates {
        if let Some(mut button) = world.get_mut::<UiButton>(entity) {
            if clicked && !button.clicked {
                log_messages.push(format!("Button '{}' clicked", button.label));
            }
            button.clicked = clicked;
            if clicked {
                button.click_time = Some(std::time::Instant::now());
            }
        }
    }
    
    // Apply text updates
    for (entity, new_value) in text_updates {
        if let Some(mut input) = world.get_mut::<UiTextInput>(entity) {
            input.value = new_value;
        }
    }
    
    // Apply checkbox updates
    for (entity, checked) in checkbox_updates {
        if let Some(mut checkbox) = world.get_mut::<UiCheckbox>(entity) {
            checkbox.checked = checked;
        }
    }
    
    // Apply radio button updates
    for (entity, selected) in radio_updates {
        if let Some(mut radio) = world.get_mut::<UiRadioButton>(entity) {
            radio.selected = selected;
        }
    }
    
    // Add log messages
    for message in log_messages {
        add_button_click_log_message(world, &message);
    }
    
    (renaming_entity, rename_buffer, resizing_entity)
}

// Helper function to draw resize handles
fn draw_resize_handles(ui: &mut egui::Ui, rect: egui::Rect, size_updates: &mut Vec<(Entity, egui::Vec2)>, entity: Entity) {
    let handle_size = 10.0;
    let bottom_right = rect.right_bottom() - egui::vec2(handle_size / 2.0, handle_size / 2.0);
    let handle_rect = egui::Rect::from_center_size(bottom_right, egui::vec2(handle_size, handle_size));
    
    let handle_response = ui.interact(handle_rect, ui.id().with(entity).with("resize_handle"), egui::Sense::drag());
    
    // Draw the handle with better visibility
    ui.painter().rect_filled(handle_rect, 2.0, egui::Color32::from_rgb(0, 100, 255));
    ui.painter().rect_stroke(handle_rect, 2.0, egui::Stroke::new(1.0, egui::Color32::WHITE), egui::StrokeKind::Outside);
    
    if handle_response.dragged() {
        let new_size = rect.size() + handle_response.drag_delta();
        let min_size = egui::vec2(60.0, 25.0); // Minimum size for buttons
        let max_size = egui::vec2(300.0, 100.0); // Maximum size to prevent huge buttons
        let clamped_size = egui::vec2(
            new_size.x.clamp(min_size.x, max_size.x), 
            new_size.y.clamp(min_size.y, max_size.y)
        );
        size_updates.push((entity, clamped_size));
    }
    
    if handle_response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNwSe);
    }
    
    // Draw resize indicator
    ui.painter().text(
        handle_rect.center() + egui::vec2(0.0, -15.0),
        egui::Align2::CENTER_CENTER,
        "↘",
        egui::FontId::proportional(12.0),
        egui::Color32::WHITE,
    );
}

// Helper functions for UI Designer
fn get_current_tab_kind(dock_state: &mut DockState<Tab>) -> Option<TabKind> {
    if let Some(tab) = dock_state.find_active_focused() {
        Some(tab.1.kind.clone())
    } else {
        None
    }
}

fn get_next_position_for_tab(world: &mut World, tab_kind: &TabKind) -> usize {
    let mut max_position = 0;
    
    // Query all UI elements for this tab
    let mut button_query = world.query::<&UiElementTab>();
    for tab in button_query.iter(world) {
        if std::mem::discriminant(&tab.tab_kind) == std::mem::discriminant(tab_kind) {
            max_position = max_position.max(tab.position);
        }
    }
    
    max_position + 1
}

fn add_button_to_current_tab(world: &mut World, dock_state: &mut DockState<Tab>) {
    if let Some(current_tab_kind) = get_current_tab_kind(dock_state) {
        let position = get_next_position_for_tab(world, &current_tab_kind);
        let button_count = world.query::<&UiButton>().iter(world).count();
        
        // Generate a position with some offset to avoid overlap
        let mut x_offset = (button_count % 5) as f32 * 120.0 + 20.0;
        let mut y_offset = (button_count / 5) as f32 * 50.0 + 50.0;
        
        // Apply grid snapping to initial position
        {
            let mut grid_query = world.query::<&GridSettings>();
            if let Some(grid_settings) = grid_query.iter(world).next() {
                let snapped_pos = snap_to_grid(egui::Pos2::new(x_offset, y_offset), grid_settings);
                x_offset = snapped_pos.x;
                y_offset = snapped_pos.y;
            }
        }
        
        world.spawn(UiButtonBundle {
            button: UiButton {
                label: format!("Button {}", button_count + 1),
                clicked: false,
                enabled: true,
                click_time: None,
                font_size: 14.0,
            },
            tab: UiElementTab {
                tab_kind: current_tab_kind,
                position,
            },
            position: UiElementPosition {
                x: x_offset,
                y: y_offset,
            },
            size: UiElementSize::default(),
            selected: UiElementSelected::default(),
        });
        
        // Log the action
        add_designer_log(world, &format!("Added Button {} to tab", button_count + 1));
    }
}

fn add_text_input_to_current_tab(world: &mut World, dock_state: &mut DockState<Tab>) {
    if let Some(current_tab_kind) = get_current_tab_kind(dock_state) {
        let position = get_next_position_for_tab(world, &current_tab_kind);
        let input_count = world.query::<&UiTextInput>().iter(world).count();
        
        // Generate a position with some offset to avoid overlap
        let mut x_offset = (input_count % 3) as f32 * 200.0 + 20.0;
        let mut y_offset = (input_count / 3) as f32 * 50.0 + 150.0;
        
        // Apply grid snapping to initial position
        {
            let mut grid_query = world.query::<&GridSettings>();
            if let Some(grid_settings) = grid_query.iter(world).next() {
                let snapped_pos = snap_to_grid(egui::Pos2::new(x_offset, y_offset), grid_settings);
                x_offset = snapped_pos.x;
                y_offset = snapped_pos.y;
            }
        }
        
        world.spawn(UiTextInputBundle {
            input: UiTextInput {
                label: format!("Input {}:", input_count + 1),
                value: String::new(),
                enabled: true,
                font_size: 14.0,
            },
            tab: UiElementTab {
                tab_kind: current_tab_kind,
                position,
            },
            position: UiElementPosition {
                x: x_offset,
                y: y_offset,
            },
            size: UiElementSize::default(),
            selected: UiElementSelected::default(),
        });
        
        // Log the action
        add_designer_log(world, &format!("Added Text Input {} to tab", input_count + 1));
    }
}

fn clear_all_ui_elements(world: &mut World) {
    // Collect entities to despawn
    let mut entities_to_remove = Vec::new();
    
    // Query buttons
    let mut button_query = world.query::<(Entity, &UiButton)>();
    for (entity, _) in button_query.iter(world) {
        entities_to_remove.push(entity);
    }
    
    // Query text inputs
    let mut input_query = world.query::<(Entity, &UiTextInput)>();
    for (entity, _) in input_query.iter(world) {
        entities_to_remove.push(entity);
    }
    
    // Query checkboxes
    let mut checkbox_query = world.query::<(Entity, &UiCheckbox)>();
    for (entity, _) in checkbox_query.iter(world) {
        entities_to_remove.push(entity);
    }
    
    // Query radio buttons
    let mut radio_query = world.query::<(Entity, &UiRadioButton)>();
    for (entity, _) in radio_query.iter(world) {
        entities_to_remove.push(entity);
    }
    
    // Query group boxes
    let mut group_query = world.query::<(Entity, &UiGroupBox)>();
    for (entity, _) in group_query.iter(world) {
        entities_to_remove.push(entity);
    }
    
    // Despawn all UI elements
    for entity in entities_to_remove {
        world.despawn(entity);
    }
    
    add_designer_log(world, "Cleared all UI elements");
}

fn add_checkbox_to_current_tab(world: &mut World, dock_state: &mut DockState<Tab>) {
    if let Some(current_tab_kind) = get_current_tab_kind(dock_state) {
        let position = get_next_position_for_tab(world, &current_tab_kind);
        let checkbox_count = world.query::<&UiCheckbox>().iter(world).count();
        
        // Generate a position with some offset to avoid overlap
        let mut x_offset = (checkbox_count % 4) as f32 * 150.0 + 20.0;
        let mut y_offset = (checkbox_count / 4) as f32 * 50.0 + 250.0;
        
        // Apply grid snapping to initial position
        {
            let mut grid_query = world.query::<&GridSettings>();
            if let Some(grid_settings) = grid_query.iter(world).next() {
                let snapped_pos = snap_to_grid(egui::Pos2::new(x_offset, y_offset), grid_settings);
                x_offset = snapped_pos.x;
                y_offset = snapped_pos.y;
            }
        }
        
        world.spawn(UiCheckboxBundle {
            checkbox: UiCheckbox {
                label: format!("Checkbox {}", checkbox_count + 1),
                checked: false,
                enabled: true,
                font_size: 14.0,
            },
            tab: UiElementTab {
                tab_kind: current_tab_kind,
                position,
            },
            position: UiElementPosition {
                x: x_offset,
                y: y_offset,
            },
            size: UiElementSize::default(),
            selected: UiElementSelected::default(),
        });
        
        add_designer_log(world, &format!("Added Checkbox {} to tab", checkbox_count + 1));
    }
}

fn add_radio_button_to_current_tab(world: &mut World, dock_state: &mut DockState<Tab>) {
    if let Some(current_tab_kind) = get_current_tab_kind(dock_state) {
        let position = get_next_position_for_tab(world, &current_tab_kind);
        let radio_count = world.query::<&UiRadioButton>().iter(world).count();
        
        // Generate a position with some offset to avoid overlap
        let mut x_offset = (radio_count % 4) as f32 * 150.0 + 20.0;
        let mut y_offset = (radio_count / 4) as f32 * 50.0 + 350.0;
        
        // Apply grid snapping to initial position
        {
            let mut grid_query = world.query::<&GridSettings>();
            if let Some(grid_settings) = grid_query.iter(world).next() {
                let snapped_pos = snap_to_grid(egui::Pos2::new(x_offset, y_offset), grid_settings);
                x_offset = snapped_pos.x;
                y_offset = snapped_pos.y;
            }
        }
        
        world.spawn(UiRadioButtonBundle {
            radio: UiRadioButton {
                label: format!("Radio {}", radio_count + 1),
                selected: false,
                enabled: true,
                font_size: 14.0,
                group_id: "default_group".to_string(), // Default group
            },
            tab: UiElementTab {
                tab_kind: current_tab_kind,
                position,
            },
            position: UiElementPosition {
                x: x_offset,
                y: y_offset,
            },
            size: UiElementSize::default(),
            selected: UiElementSelected::default(),
        });
        
        add_designer_log(world, &format!("Added Radio Button {} to tab", radio_count + 1));
    }
}

fn add_group_box_to_current_tab(world: &mut World, dock_state: &mut DockState<Tab>) {
    if let Some(current_tab_kind) = get_current_tab_kind(dock_state) {
        let position = get_next_position_for_tab(world, &current_tab_kind);
        let group_count = world.query::<&UiGroupBox>().iter(world).count();
        
        // Generate a position with some offset to avoid overlap
        let mut x_offset = (group_count % 3) as f32 * 250.0 + 50.0;
        let mut y_offset = (group_count / 3) as f32 * 200.0 + 450.0;
        
        // Apply grid snapping to initial position
        {
            let mut grid_query = world.query::<&GridSettings>();
            if let Some(grid_settings) = grid_query.iter(world).next() {
                let snapped_pos = snap_to_grid(egui::Pos2::new(x_offset, y_offset), grid_settings);
                x_offset = snapped_pos.x;
                y_offset = snapped_pos.y;
            }
        }
        
        world.spawn(UiGroupBoxBundle {
            group_box: UiGroupBox {
                label: format!("Group {}", group_count + 1),
                enabled: true,
                font_size: 14.0,
                contained_widgets: Vec::new(),
            },
            tab: UiElementTab {
                tab_kind: current_tab_kind,
                position,
            },
            position: UiElementPosition {
                x: x_offset,
                y: y_offset,
            },
            size: UiElementSize {
                width: 200.0,  // Default group box size
                height: 150.0,
            },
            selected: UiElementSelected::default(),
            container: UiElementContainer {
                parent_group: None, // Group boxes are not contained in other groups by default
            },
        });
        
        add_designer_log(world, &format!("Added Group Box {} to tab", group_count + 1));
    }
}

fn add_designer_log(world: &mut World, message: &str) {
    let mut query = world.query::<&mut EventLoggerPanel>();
    if let Some(mut logger) = query.iter_mut(world).next() {
        // Insert at front for newest-first display
        logger.entries.insert(0, LogEntry {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            level: LogLevel::Info,
            message: format!("[UI Designer] {}", message),
        });
        
        // Keep within max entries limit
        let max_entries = logger.max_entries;
        if logger.entries.len() > max_entries {
            logger.entries.truncate(max_entries);
        }
    }
}

fn add_button_click_log_message(world: &mut World, message: &str) {
    let mut query = world.query::<&mut EventLoggerPanel>();
    if let Some(mut logger) = query.iter_mut(world).next() {
        // Insert at front for newest-first display
        logger.entries.insert(0, LogEntry {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            level: LogLevel::Debug,
            message: message.to_string(),
        });
        
        // Keep within max entries limit
        let max_entries = logger.max_entries;
        if logger.entries.len() > max_entries {
            logger.entries.truncate(max_entries);
        }
    }
}

fn reset_button_clicks(world: &mut World) {
    let mut query = world.query::<&mut UiButton>();
    let now = std::time::Instant::now();
    
    for mut button in query.iter_mut(world) {
        if button.clicked {
            if let Some(click_time) = button.click_time {
                // Reset after 500ms
                if now.duration_since(click_time).as_millis() > 500 {
                    button.clicked = false;
                    button.click_time = None;
                }
            }
        }
    }
}

// Helper function to add UI elements to the world
fn add_ui_elements_to_world(world: &mut World) {
    // Initialize grid settings with CORRECT values
    world.spawn(GridSettings {
        enabled: false,
        spacing_pixels: 10.0,  // 10px spacing
        dot_size: 0.5,         // 0.5px dots
        snap_enabled: false,
        show_grid: false,
    });
    
    // Add some example buttons to different tabs
    world.spawn(UiButtonBundle {
        button: UiButton {
            label: "Dynamic Button 1".to_string(),
            clicked: false,
            enabled: true,
            click_time: None,
            font_size: 14.0,
        },
        tab: UiElementTab {
            tab_kind: TabKind::MainWork,
            position: 0,
        },
        position: UiElementPosition { x: 20.0, y: 50.0 },
        size: UiElementSize::default(),
        selected: UiElementSelected::default(),
    });
    
    world.spawn(UiButtonBundle {
        button: UiButton {
            label: "Dynamic Button 2".to_string(),
            clicked: false,
            enabled: true,
            click_time: None,
            font_size: 14.0,
        },
        tab: UiElementTab {
            tab_kind: TabKind::MainWork,
            position: 1,
        },
        position: UiElementPosition { x: 140.0, y: 50.0 },
        size: UiElementSize::default(),
        selected: UiElementSelected::default(),
    });
    
    // Add text inputs to Settings tab
    world.spawn(UiTextInputBundle {
        input: UiTextInput {
            label: "Project Name:".to_string(),
            value: "My Project".to_string(),
            enabled: true,
            font_size: 14.0,
        },
        tab: UiElementTab {
            tab_kind: TabKind::Settings,
            position: 0,
        },
        position: UiElementPosition { x: 20.0, y: 20.0 },
        size: UiElementSize::default(),
        selected: UiElementSelected::default(),
    });
    
    world.spawn(UiTextInputBundle {
        input: UiTextInput {
            label: "Author:".to_string(),
            value: "".to_string(),
            enabled: true,
            font_size: 14.0,
        },
        tab: UiElementTab {
            tab_kind: TabKind::Settings,
            position: 1,
        },
        position: UiElementPosition { x: 20.0, y: 70.0 },
        size: UiElementSize::default(),
        selected: UiElementSelected::default(),
    });
    
    // Add controls to Controls tab
    world.spawn(UiButtonBundle {
        button: UiButton {
            label: "Execute Action".to_string(),
            clicked: false,
            enabled: true,
            click_time: None,
            font_size: 14.0,
        },
        tab: UiElementTab {
            tab_kind: TabKind::Controls,
            position: 0,
        },
        position: UiElementPosition { x: 20.0, y: 20.0 },
        size: UiElementSize::default(),
        selected: UiElementSelected::default(),
    });
    
    world.spawn(UiTextInputBundle {
        input: UiTextInput {
            label: "Parameter:".to_string(),
            value: "100".to_string(),
            enabled: true,
            font_size: 14.0,
        },
        tab: UiElementTab {
            tab_kind: TabKind::Controls,
            position: 1,
        },
        position: UiElementPosition { x: 20.0, y: 70.0 },
        size: UiElementSize::default(),
        selected: UiElementSelected::default(),
    });
}

// Helper functions for selection and distribution
fn clear_all_selections(world: &mut World) {
    let mut query = world.query::<&mut UiElementSelected>();
    for mut selected in query.iter_mut(world) {
        selected.selected = false;
    }
}

fn distribute_items_vertically(world: &mut World, selected_entities: &[Entity], spacing: f32) {
    if selected_entities.len() < 2 {
        return;
    }
    
    // Get positions of selected items and sort by Y coordinate
    let mut items: Vec<(Entity, f32, f32)> = Vec::new();
    for &entity in selected_entities {
        if let Some(pos) = world.get::<UiElementPosition>(entity) {
            items.push((entity, pos.x, pos.y));
        }
    }
    
    // Sort by Y coordinate
    items.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    
    // Distribute with specified spacing
    for (i, &(entity, x, _)) in items.iter().enumerate() {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity) {
            pos.y = items[0].2 + (i as f32 * spacing);
        }
    }
}

fn distribute_items_horizontally(world: &mut World, selected_entities: &[Entity], spacing: f32) {
    if selected_entities.len() < 2 {
        return;
    }
    
    // Get positions of selected items and sort by X coordinate
    let mut items: Vec<(Entity, f32, f32)> = Vec::new();
    for &entity in selected_entities {
        if let Some(pos) = world.get::<UiElementPosition>(entity) {
            items.push((entity, pos.x, pos.y));
        }
    }
    
    // Sort by X coordinate
    items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Distribute with specified spacing
    for (i, &(entity, _, y)) in items.iter().enumerate() {
        if let Some(mut pos) = world.get_mut::<UiElementPosition>(entity) {
            pos.x = items[0].1 + (i as f32 * spacing);
        }
    }
}

// Project save functionality
fn save_project_dialog(world: &mut World) {
    // For now, save to a default location - later we can add file dialog
    let project_name = "my_ui_project";
    let project_path = format!("./{}", project_name);
    
    match save_project_to_disk(world, project_name, &project_path) {
        Ok(()) => {
            add_designer_log(world, &format!("Project saved to {}", project_path));
        }
        Err(e) => {
            add_designer_log(world, &format!("Save failed: {}", e));
        }
    }
}

fn save_project_to_disk(world: &mut World, project_name: &str, project_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create project directory
    std::fs::create_dir_all(format!("{}/src", project_path))?;
    
    // Generate Cargo.toml
    let cargo_toml = generate_cargo_toml(project_name);
    std::fs::write(format!("{}/Cargo.toml", project_path), cargo_toml)?;
    
    // Generate main.rs with the designed UI
    let main_rs = generate_main_rs(world)?;
    std::fs::write(format!("{}/src/main.rs", project_path), main_rs)?;
    
    // Generate README.md
    let readme = generate_readme(project_name);
    std::fs::write(format!("{}/README.md", project_path), readme)?;
    
    Ok(())
}

fn generate_cargo_toml(project_name: &str) -> String {
    format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "Generated by Mobius ECS UI Designer"

[dependencies]
eframe = "0.31.1"
egui = "0.31.1"
bevy_ecs = "0.14"
serde = {{ version = "1.0", features = ["derive"] }}
chrono = {{ version = "0.4", features = ["serde"] }}
chrono-tz = "0.10.3"
egui_extras = "0.31.1"
image = "0.25.6"
egui_dock = {{version = "0.16.0", features=["serde"]}}

[dev-dependencies]
env_logger = "0.10"
"#, project_name)
}

fn generate_main_rs(world: &mut World) -> Result<String, Box<dyn std::error::Error>> {
    let mut ui_elements = String::new();
    let mut entity_count = 0;
    
    // Collect all UI buttons
    {
        let mut query = world.query::<(&UiButton, &UiElementTab, &UiElementPosition, &UiElementSize)>();
        for (button, tab, pos, size) in query.iter(world) {
            ui_elements.push_str(&format!(r#"
        // Button: {}
        ui.allocate_ui_at_rect(
            egui::Rect::from_min_size(
                egui::pos2({:.1}, {:.1}),
                egui::vec2({:.1}, {:.1})
            ),
            |ui| {{
                if ui.button("{}").clicked() {{
                    println!("Button '{}' clicked!");
                }}
            }}
        );
"#, button.label, pos.x, pos.y, size.width.max(100.0), size.height.max(25.0), button.label, button.label));
            entity_count += 1;
        }
    }
    
    // Collect all text inputs
    {
        let mut query = world.query::<(&UiTextInput, &UiElementTab, &UiElementPosition, &UiElementSize)>();
        for (input, tab, pos, size) in query.iter(world) {
            ui_elements.push_str(&format!(r#"
        // Text Input: {}
        ui.allocate_ui_at_rect(
            egui::Rect::from_min_size(
                egui::pos2({:.1}, {:.1}),
                egui::vec2({:.1}, {:.1})
            ),
            |ui| {{
                ui.horizontal(|ui| {{
                    ui.label("{}");
                    ui.text_edit_singleline(&mut String::new());
                }});
            }}
        );
"#, input.label, pos.x, pos.y, size.width.max(200.0), size.height.max(25.0), input.label));
            entity_count += 1;
        }
    }
    
    // Collect all checkboxes
    {
        let mut query = world.query::<(&UiCheckbox, &UiElementTab, &UiElementPosition, &UiElementSize)>();
        for (checkbox, tab, pos, size) in query.iter(world) {
            ui_elements.push_str(&format!(r#"
        // Checkbox: {}
        ui.allocate_ui_at_rect(
            egui::Rect::from_min_size(
                egui::pos2({:.1}, {:.1}),
                egui::vec2({:.1}, {:.1})
            ),
            |ui| {{
                let mut checked = {};
                ui.checkbox(&mut checked, "{}");
            }}
        );
"#, checkbox.label, pos.x, pos.y, size.width.max(150.0), size.height.max(25.0), checkbox.checked, checkbox.label));
            entity_count += 1;
        }
    }
    
    // Collect all radio buttons
    {
        let mut query = world.query::<(&UiRadioButton, &UiElementTab, &UiElementPosition, &UiElementSize)>();
        for (radio, tab, pos, size) in query.iter(world) {
            ui_elements.push_str(&format!(r#"
        // Radio Button: {}
        ui.allocate_ui_at_rect(
            egui::Rect::from_min_size(
                egui::pos2({:.1}, {:.1}),
                egui::vec2({:.1}, {:.1})
            ),
            |ui| {{
                let mut selected = {};
                ui.radio_value(&mut selected, true, "{}");
            }}
        );
"#, radio.label, pos.x, pos.y, size.width.max(150.0), size.height.max(25.0), radio.selected, radio.label));
            entity_count += 1;
        }
    }
    
    // Collect all group boxes
    {
        let mut query = world.query::<(&UiGroupBox, &UiElementTab, &UiElementPosition, &UiElementSize)>();
        for (group_box, tab, pos, size) in query.iter(world) {
            ui_elements.push_str(&format!(r#"
        // Group Box: {}
        ui.allocate_ui_at_rect(
            egui::Rect::from_min_size(
                egui::pos2({:.1}, {:.1}),
                egui::vec2({:.1}, {:.1})
            ),
            |ui| {{
                egui::Frame::group(ui.style()).show(ui, |ui| {{
                    ui.heading("{}");
                    ui.separator();
                    ui.label("Group content area");
                }});
            }}
        );
"#, group_box.label, pos.x, pos.y, size.width.max(200.0), size.height.max(150.0), group_box.label));
            entity_count += 1;
        }
    }
    
    if entity_count == 0 {
        ui_elements = r#"
        ui.heading("Welcome to your generated UI!");
        ui.label("Add some UI elements in the designer and save again.");
"#.to_string();
    }
    
    Ok(format!(r#"use eframe::{{egui, App, Frame}};

#[derive(Default)]
pub struct GeneratedApp {{
    // Add your app state here
}}

impl App for GeneratedApp {{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {{
        egui::CentralPanel::default().show(ctx, |ui| {{
            ui.heading("Generated UI from Mobius ECS Designer");
            ui.separator();
            
            // Generated UI elements from designer:{}
        }});
    }}
}}

fn main() -> Result<(), eframe::Error> {{
    env_logger::init();
    
    eframe::run_native(
        "Generated UI Project",
        eframe::NativeOptions {{
            viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
            ..Default::default()
        }},
        Box::new(|_cc| {{
            Ok(Box::new(GeneratedApp::default()))
        }})
    )
}}
"#, ui_elements))
}

fn generate_readme(project_name: &str) -> String {
    format!(r#"# {}

This project was generated by the Mobius ECS UI Designer.

## Features

- Dynamic UI layout designed visually
- Built with egui and Rust
- Fully customizable and extendable

## Running the Project

```bash
cargo run
```

## Customization

Edit `src/main.rs` to:
- Add application state and logic
- Modify UI element behavior
- Add new features and functionality

## Generated with

- [Mobius ECS](https://github.com/saturn77/mobius-ecs) - ECS-based UI framework
- [egui](https://github.com/emilk/egui) - Immediate mode GUI library
- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) - Framework for egui apps

Happy coding! 🦀
"#, project_name)
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Enable logging
    
    eframe::run_native(
        "Mobius ECS Demo - Dynamic UI Designer",
        eframe::NativeOptions {
            viewport: ViewportBuilder::default().with_inner_size([1400.0, 900.0]),
            ..Default::default()
        },
        Box::new(|cc|{
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MobiusToolWindowsDemo::new()))
        }))
}