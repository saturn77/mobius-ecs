use bevy_ecs::prelude::*;
use egui_dock::{DockArea, DockState, Style as DockStyle, TabViewer};
use egui::{Context, Ui};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ============================================================================
// ECS Components for Simple UI Layout
// ============================================================================

#[derive(Component, Clone, Debug)]
pub enum UiElement {
    Label { text: String },
    Button { text: String, clicked: bool },
    TextEdit { value: String, multiline: bool },
    Checkbox { checked: bool, text: String },
    Container { direction: FlexDirection },
}

#[derive(Clone, Debug)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Component)]
pub struct TabId(pub String);

#[derive(Component)]
pub struct TabRoot;

#[derive(Component)]
pub struct FlexGrow(pub f32);

#[derive(Component)]
pub struct Padding(pub f32);

#[derive(Component)]
pub struct Margin(pub f32);

// ============================================================================
// Tab System
// ============================================================================

pub struct TabContent {
    pub entity: Entity,
}

pub struct TabViewerState {
    pub tabs: HashMap<String, TabContent>,
    pub world_ref: Arc<Mutex<*mut World>>,
}

unsafe impl Send for TabViewerState {}
unsafe impl Sync for TabViewerState {}

impl TabViewer for TabViewerState {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.clone().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        if let Some(tab_content) = self.tabs.get(tab) {
            let world_ptr = *self.world_ref.lock().unwrap();
            let world = unsafe { &mut *world_ptr };
            
            // Use simple egui layout instead of taffy
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                render_entity_simple(ui, world, tab_content.entity);
            });
        }
    }
}

// ============================================================================
// Simple Rendering System
// ============================================================================

fn render_entity_simple(ui: &mut Ui, world: &mut World, entity: Entity) {
    // Handle padding
    let padding = world.get::<Padding>(entity).map(|p| p.0).unwrap_or(0.0);
    if padding > 0.0 {
        ui.add_space(padding);
    }
    
    // Render the UI element
    if let Some(element) = world.get::<UiElement>(entity).cloned() {
        match element {
            UiElement::Label { text } => {
                ui.label(text);
            }
            UiElement::Button { text, .. } => {
                if ui.button(text.clone()).clicked() {
                    // Update clicked state
                    if let Some(mut elem) = world.get_mut::<UiElement>(entity) {
                        if let UiElement::Button { clicked, .. } = &mut *elem {
                            *clicked = true;
                        }
                    }
                }
            }
            UiElement::TextEdit { mut value, multiline } => {
                let response = if multiline {
                    ui.text_edit_multiline(&mut value)
                } else {
                    ui.text_edit_singleline(&mut value)
                };
                
                if response.changed() {
                    if let Some(mut elem) = world.get_mut::<UiElement>(entity) {
                        if let UiElement::TextEdit { value: v, .. } = &mut *elem {
                            *v = value;
                        }
                    }
                }
            }
            UiElement::Checkbox { mut checked, text } => {
                let response = ui.checkbox(&mut checked, &text);
                
                if response.changed() {
                    if let Some(mut elem) = world.get_mut::<UiElement>(entity) {
                        if let UiElement::Checkbox { checked: c, .. } = &mut *elem {
                            *c = checked;
                        }
                    }
                }
            }
            UiElement::Container { direction } => {
                let layout = match direction {
                    FlexDirection::Row => egui::Layout::left_to_right(egui::Align::TOP),
                    FlexDirection::Column => egui::Layout::top_down(egui::Align::LEFT),
                };
                
                ui.with_layout(layout, |ui| {
                    render_children_simple(ui, world, entity);
                });
                return; // Don't render children again
            }
        }
    }
    
    // Render children
    render_children_simple(ui, world, entity);
}

fn render_children_simple(ui: &mut Ui, world: &mut World, entity: Entity) {
    let children_list: Vec<Entity> = world.get::<Children>(entity)
        .map(|children| children.iter().collect())
        .unwrap_or_default();
    
    for child in children_list {
        render_entity_simple(ui, world, child);
        
        // Add some spacing between children
        ui.add_space(4.0);
    }
}

// ============================================================================
// Dock System
// ============================================================================

pub struct DockResource {
    pub dock_state: DockState<String>,
    pub tab_viewer_state: TabViewerState,
}

pub fn setup_simple_dock_system(world: &mut World) {
    let dock_state = DockState::new(vec!["Main".to_string()]);
    
    let mut tabs = HashMap::new();
    
    // Create main tab root entity
    let main_tab_root = world.spawn((
        TabId("Main".to_string()),
        TabRoot,
        UiElement::Container { direction: FlexDirection::Column },
        Padding(16.0),
    )).id();
    
    tabs.insert("Main".to_string(), TabContent {
        entity: main_tab_root,
    });
    
    let tab_viewer_state = TabViewerState {
        tabs,
        world_ref: Arc::new(Mutex::new(world as *mut World)),
    };
    
    world.insert_non_send_resource(DockResource {
        dock_state,
        tab_viewer_state,
    });
}

pub fn render_simple_dock_system(ctx: &Context, world: &mut World) {
    if let Some(mut dock_resource) = world.remove_non_send_resource::<DockResource>() {
        // Update world pointer
        *dock_resource.tab_viewer_state.world_ref.lock().unwrap() = world as *mut World;
        
        DockArea::new(&mut dock_resource.dock_state)
            .style(DockStyle::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut dock_resource.tab_viewer_state);
        
        world.insert_non_send_resource(dock_resource);
    }
}

// ============================================================================
// Helper functions
// ============================================================================

pub fn create_simple_label(world: &mut World, text: String) -> Entity {
    world.spawn((
        UiElement::Label { text },
        Margin(4.0),
    )).id()
}

pub fn create_simple_button(world: &mut World, text: String) -> Entity {
    world.spawn((
        UiElement::Button { text, clicked: false },
        Margin(4.0),
    )).id()
}

pub fn create_simple_container(world: &mut World, direction: FlexDirection) -> Entity {
    world.spawn((
        UiElement::Container { direction },
        Padding(8.0),
    )).id()
}

pub fn create_simple_checkbox(world: &mut World, text: String, checked: bool) -> Entity {
    world.spawn((
        UiElement::Checkbox { text, checked },
        Margin(4.0),
    )).id()
}

pub fn add_simple_tab(world: &mut World, tab_name: String) -> Option<Entity> {
    let tab_root = world.spawn((
        TabId(tab_name.clone()),
        TabRoot,
        UiElement::Container { direction: FlexDirection::Column },
        Padding(16.0),
    )).id();
    
    if let Some(mut dock_resource) = world.get_non_send_resource_mut::<DockResource>() {
        dock_resource.tab_viewer_state.tabs.insert(tab_name.clone(), TabContent {
            entity: tab_root,
        });
        
        dock_resource.dock_state.push_to_focused_leaf(tab_name);
        
        Some(tab_root)
    } else {
        world.despawn(tab_root);
        None
    }
}