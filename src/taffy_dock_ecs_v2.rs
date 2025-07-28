use bevy_ecs::prelude::*;
use egui_dock::{DockArea, DockState, Style as DockStyle, TabViewer};
use egui_taffy::{taffy, tui, TuiBuilderLogic};
use egui::{Context, Ui};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ============================================================================
// ECS Components for Taffy Flex Layout
// ============================================================================

#[derive(Component, Clone, Debug)]
pub struct TaffyStyle(pub taffy::Style);

impl Default for TaffyStyle {
    fn default() -> Self {
        Self(taffy::Style::default())
    }
}

#[derive(Component, Clone, Debug)]
pub enum UiElement {
    Label { text: String },
    Button { text: String, clicked: bool },
    TextEdit { value: String, multiline: bool },
    Checkbox { checked: bool, text: String },
    Container,
}

#[derive(Component)]
pub struct TabId(pub String);

#[derive(Component)]
pub struct TabRoot(pub Entity);

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

// Mark TabViewerState as Send and Sync since we manage thread safety manually
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
            // Safety: We ensure the World pointer is valid during the frame
            let world = unsafe { &mut *world_ptr };
            
            // Use egui_taffy tui to layout the tab content
            tui(ui, ui.id().with(tab))
                .style(taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::Percent(1.0),
                        height: taffy::Dimension::Percent(1.0),
                    },
                    flex_direction: taffy::FlexDirection::Column,
                    ..Default::default()
                })
                .show(|tui| {
                    render_entity_tree(tui, world, tab_content.entity);
                });
        }
    }
}

// ============================================================================
// Rendering System
// ============================================================================

fn render_entity_tree(tui: &mut egui_taffy::Tui, world: &mut World, entity: Entity) {
    // First, render this entity's UI element if it has one
    let element_clone = world.get::<UiElement>(entity).cloned();
    if let Some(element) = element_clone {
        match element {
            UiElement::Label { text } => {
                tui.label(text);
            }
            UiElement::Button { text, .. } => {
                if tui.button(|tui| { tui.label(text); }).clicked() {
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
                    tui.ui(|ui| ui.text_edit_multiline(&mut value))
                } else {
                    tui.ui(|ui| ui.text_edit_singleline(&mut value))
                };
                
                if response.changed() {
                    // Update the value in the world
                    if let Some(mut elem) = world.get_mut::<UiElement>(entity) {
                        if let UiElement::TextEdit { value: v, .. } = &mut *elem {
                            *v = value;
                        }
                    }
                }
            }
            UiElement::Checkbox { mut checked, text } => {
                let response = tui.ui(|ui| ui.checkbox(&mut checked, &text));
                
                if response.changed() {
                    // Update the value in the world
                    if let Some(mut elem) = world.get_mut::<UiElement>(entity) {
                        if let UiElement::Checkbox { checked: c, .. } = &mut *elem {
                            *c = checked;
                        }
                    }
                }
            }
            UiElement::Container => {
                // Just a container, render children
            }
        }
    }
    
    // Then render children with their styles
    let children_list: Vec<Entity> = world.get::<Children>(entity)
        .map(|children| children.iter().collect())
        .unwrap_or_default();
    
    for child in children_list {
        // Get child's style if it has one
        let child_style = world.get::<TaffyStyle>(child)
            .map(|s| s.0.clone())
            .unwrap_or_default();
        
        // Create a new tui context with the child's style
        tui.style(child_style)
            .add(|child_tui| {
                render_entity_tree(child_tui, world, child);
            });
    }
}

// ============================================================================
// System to manage dock state
// ============================================================================

pub struct DockResource {
    pub dock_state: DockState<String>,
    pub tab_viewer_state: TabViewerState,
}

pub fn setup_dock_system(world: &mut World) {
    let dock_state = DockState::new(vec!["Main".to_string()]);
    
    let mut tabs = HashMap::new();
    
    // Create main tab root entity
    let main_tab_root = world.spawn((
        TabId("Main".to_string()),
        TaffyStyle(taffy::Style {
            flex_direction: taffy::FlexDirection::Column,
            gap: taffy::Size {
                width: taffy::LengthPercentage::Length(8.0),
                height: taffy::LengthPercentage::Length(8.0),
            },
            padding: taffy::prelude::length(8.0),
            ..Default::default()
        }),
        UiElement::Container,
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

pub fn render_dock_system(ctx: &Context, world: &mut World) {
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
// Helper functions to create UI elements
// ============================================================================

pub fn create_label(world: &mut World, text: String, style: taffy::Style) -> Entity {
    world.spawn((
        UiElement::Label { text },
        TaffyStyle(style),
    )).id()
}

pub fn create_button(world: &mut World, text: String, style: taffy::Style) -> Entity {
    world.spawn((
        UiElement::Button { text, clicked: false },
        TaffyStyle(style),
    )).id()
}

pub fn create_container(world: &mut World, style: taffy::Style) -> Entity {
    world.spawn((
        UiElement::Container,
        TaffyStyle(style),
    )).id()
}

pub fn create_checkbox(world: &mut World, text: String, checked: bool, style: taffy::Style) -> Entity {
    world.spawn((
        UiElement::Checkbox { text, checked },
        TaffyStyle(style),
    )).id()
}

pub fn add_new_tab(world: &mut World, tab_name: String, root_style: taffy::Style) -> Option<Entity> {
    // First create the tab entity
    let tab_root = world.spawn((
        TabId(tab_name.clone()),
        TaffyStyle(root_style),
        UiElement::Container,
    )).id();
    
    // Then update the dock resource
    if let Some(mut dock_resource) = world.get_non_send_resource_mut::<DockResource>() {
        dock_resource.tab_viewer_state.tabs.insert(tab_name.clone(), TabContent {
            entity: tab_root,
        });
        
        dock_resource.dock_state.push_to_focused_leaf(tab_name);
        
        Some(tab_root)
    } else {
        // Clean up the entity if we couldn't add it to the dock
        world.despawn(tab_root);
        None
    }
}