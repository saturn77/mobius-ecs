use bevy_ecs::prelude::*;
use egui_dock::{DockArea, DockState, Style as DockStyle, TabViewer};
use egui_taffy::{taffy, tui, TuiBuilderLogic};
use egui::{Context, Ui};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ============================================================================
// ECS Components for Flex Layout
// ============================================================================

#[derive(Component, Clone, Debug)]
pub struct FlexStyle {
    pub flex_direction: taffy::FlexDirection,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: taffy::Dimension,
    pub align_self: Option<taffy::AlignSelf>,
    pub justify_content: Option<taffy::JustifyContent>,
    pub align_items: Option<taffy::AlignItems>,
    pub gap: taffy::Size<taffy::LengthPercentage>,
    pub padding: taffy::Rect<taffy::LengthPercentageAuto>,
    pub margin: taffy::Rect<taffy::LengthPercentageAuto>,
    pub min_size: taffy::Size<taffy::Dimension>,
    pub max_size: taffy::Size<taffy::Dimension>,
}

impl Default for FlexStyle {
    fn default() -> Self {
        Self {
            flex_direction: taffy::FlexDirection::Column,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: taffy::Dimension::Auto,
            align_self: None,
            justify_content: None,
            align_items: None,
            gap: taffy::Size::zero(),
            padding: taffy::Rect::zero(),
            margin: taffy::Rect::zero(),
            min_size: taffy::Size::auto(),
            max_size: taffy::Size::auto(),
        }
    }
}

impl FlexStyle {
    pub fn to_taffy_style(&self) -> taffy::Style {
        taffy::Style {
            flex_direction: self.flex_direction,
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            flex_basis: self.flex_basis,
            align_self: self.align_self,
            justify_content: self.justify_content,
            align_items: self.align_items,
            gap: self.gap,
            padding: taffy::Rect {
                left: match self.padding.left {
                    taffy::LengthPercentageAuto::Length(v) => taffy::LengthPercentage::Length(v),
                    taffy::LengthPercentageAuto::Percent(v) => taffy::LengthPercentage::Percent(v),
                    _ => taffy::LengthPercentage::Length(0.0),
                },
                right: match self.padding.right {
                    taffy::LengthPercentageAuto::Length(v) => taffy::LengthPercentage::Length(v),
                    taffy::LengthPercentageAuto::Percent(v) => taffy::LengthPercentage::Percent(v),
                    _ => taffy::LengthPercentage::Length(0.0),
                },
                top: match self.padding.top {
                    taffy::LengthPercentageAuto::Length(v) => taffy::LengthPercentage::Length(v),
                    taffy::LengthPercentageAuto::Percent(v) => taffy::LengthPercentage::Percent(v),
                    _ => taffy::LengthPercentage::Length(0.0),
                },
                bottom: match self.padding.bottom {
                    taffy::LengthPercentageAuto::Length(v) => taffy::LengthPercentage::Length(v),
                    taffy::LengthPercentageAuto::Percent(v) => taffy::LengthPercentage::Percent(v),
                    _ => taffy::LengthPercentage::Length(0.0),
                },
            },
            margin: self.margin,
            min_size: self.min_size,
            max_size: self.max_size,
            ..Default::default()
        }
    }
}

#[derive(Component, Clone, Debug)]
pub enum FlexElement {
    Container { 
        label: Option<String>,
        show_border: bool,
    },
    Label { 
        text: String,
        color: Option<egui::Color32>,
    },
    Button { 
        text: String, 
        clicked: bool,
        min_size: Option<egui::Vec2>,
    },
    TextInput { 
        value: String, 
        hint: String,
        multiline: bool,
    },
    Slider { 
        value: f32,
        range: std::ops::RangeInclusive<f32>,
        label: String,
    },
    ProgressBar {
        progress: f32,
        label: String,
    },
    Separator,
    Spacer { 
        min_size: egui::Vec2,
    },
}

#[derive(Component)]
pub struct TabId(pub String);

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
            
            // Use egui_taffy for proper flex layout
            tui(ui, ui.id().with(tab))
                .style(taffy::Style {
                    size: taffy::Size {
                        width: taffy::Dimension::Percent(1.0),
                        height: taffy::Dimension::Percent(1.0),
                    },
                    ..Default::default()
                })
                .show(|tui| {
                    render_flex_entity(tui, world, tab_content.entity);
                });
        }
    }
}

// ============================================================================
// Flex Rendering System
// ============================================================================

fn render_flex_entity(tui: &mut egui_taffy::Tui, world: &mut World, entity: Entity) {
    // Get the flex style for this entity
    let flex_style = world.get::<FlexStyle>(entity)
        .map(|s| s.to_taffy_style())
        .unwrap_or_default();
    
    // Get the element type
    let element = world.get::<FlexElement>(entity).cloned();
    
    // Apply the style and render
    tui.style(flex_style).add(|styled_tui| {
        match element {
            Some(FlexElement::Container { label, show_border }) => {
                if let Some(label_text) = label {
                    styled_tui.label(label_text);
                }
                
                if show_border {
                    styled_tui.ui(|ui| {
                        ui.group(|_ui| {
                            // We can't render children here due to borrow checker
                            // This is a limitation we'll need to work around
                        });
                    });
                }
                render_flex_children(styled_tui, world, entity);
            }
            Some(FlexElement::Label { text, color }) => {
                if let Some(color) = color {
                    styled_tui.ui(|ui| {
                        ui.colored_label(color, text);
                    });
                } else {
                    styled_tui.label(text);
                }
            }
            Some(FlexElement::Button { text, min_size, .. }) => {
                let clicked = styled_tui.ui(|ui| {
                    let mut button = egui::Button::new(text.clone());
                    if let Some(size) = min_size {
                        button = button.min_size(size);
                    }
                    ui.add(button).clicked()
                });
                
                if clicked {
                    // Update clicked state
                    if let Some(mut elem) = world.get_mut::<FlexElement>(entity) {
                        if let FlexElement::Button { clicked, .. } = &mut *elem {
                            *clicked = true;
                        }
                    }
                }
            }
            Some(FlexElement::TextInput { mut value, hint, multiline }) => {
                let response = styled_tui.ui(|ui| {
                    if multiline {
                        ui.text_edit_multiline(&mut value)
                    } else {
                        ui.add(egui::TextEdit::singleline(&mut value).hint_text(hint))
                    }
                });
                
                if response.changed() {
                    if let Some(mut elem) = world.get_mut::<FlexElement>(entity) {
                        if let FlexElement::TextInput { value: v, .. } = &mut *elem {
                            *v = value;
                        }
                    }
                }
            }
            Some(FlexElement::Slider { mut value, range, label }) => {
                let response = styled_tui.ui(|ui| {
                    ui.add(egui::Slider::new(&mut value, range).text(label))
                });
                
                if response.changed() {
                    if let Some(mut elem) = world.get_mut::<FlexElement>(entity) {
                        if let FlexElement::Slider { value: v, .. } = &mut *elem {
                            *v = value;
                        }
                    }
                }
            }
            Some(FlexElement::ProgressBar { progress, label }) => {
                styled_tui.ui(|ui| {
                    ui.add(egui::ProgressBar::new(progress).text(label));
                });
            }
            Some(FlexElement::Separator) => {
                styled_tui.ui(|ui| {
                    ui.separator();
                });
            }
            Some(FlexElement::Spacer { min_size }) => {
                styled_tui.ui(|ui| {
                    ui.allocate_space(min_size);
                });
            }
            None => {
                // Just a container without specific element type
                render_flex_children(styled_tui, world, entity);
            }
        }
    });
}

fn render_flex_children(tui: &mut egui_taffy::Tui, world: &mut World, entity: Entity) {
    let children_list: Vec<Entity> = world.get::<Children>(entity)
        .map(|children| children.iter().collect())
        .unwrap_or_default();
    
    for child in children_list {
        render_flex_entity(tui, world, child);
    }
}

// ============================================================================
// Dock System
// ============================================================================

pub struct FlexDockResource {
    pub dock_state: DockState<String>,
    pub tab_viewer_state: TabViewerState,
}

pub fn setup_flex_dock_system(world: &mut World) {
    let dock_state = DockState::new(vec!["Main".to_string()]);
    
    let mut tabs = HashMap::new();
    
    // Create main tab root entity with flex container
    let main_tab_root = world.spawn((
        TabId("Main".to_string()),
        FlexStyle {
            flex_direction: taffy::FlexDirection::Column,
            gap: taffy::Size {
                width: taffy::LengthPercentage::Length(8.0),
                height: taffy::LengthPercentage::Length(8.0),
            },
            padding: taffy::Rect::length(16.0),
            ..Default::default()
        },
        FlexElement::Container { 
            label: None,
            show_border: false,
        },
    )).id();
    
    tabs.insert("Main".to_string(), TabContent {
        entity: main_tab_root,
    });
    
    let tab_viewer_state = TabViewerState {
        tabs,
        world_ref: Arc::new(Mutex::new(world as *mut World)),
    };
    
    world.insert_non_send_resource(FlexDockResource {
        dock_state,
        tab_viewer_state,
    });
}

pub fn render_flex_dock_system(ctx: &Context, world: &mut World) {
    if let Some(mut dock_resource) = world.remove_non_send_resource::<FlexDockResource>() {
        // Update world pointer
        *dock_resource.tab_viewer_state.world_ref.lock().unwrap() = world as *mut World;
        
        DockArea::new(&mut dock_resource.dock_state)
            .style(DockStyle::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut dock_resource.tab_viewer_state);
        
        world.insert_non_send_resource(dock_resource);
    }
}

// ============================================================================
// Helper functions for creating flex elements
// ============================================================================

pub fn create_flex_container(
    world: &mut World, 
    direction: taffy::FlexDirection,
    flex_grow: f32,
    label: Option<String>,
    show_border: bool,
) -> Entity {
    world.spawn((
        FlexStyle {
            flex_direction: direction,
            flex_grow,
            gap: taffy::Size::length(8.0),
            padding: taffy::Rect::length(8.0),
            ..Default::default()
        },
        FlexElement::Container { label, show_border },
    )).id()
}

pub fn create_flex_label(world: &mut World, text: String, flex_grow: f32, color: Option<egui::Color32>) -> Entity {
    world.spawn((
        FlexStyle {
            flex_grow,
            align_self: Some(taffy::AlignSelf::Stretch),
            ..Default::default()
        },
        FlexElement::Label { text, color },
    )).id()
}

pub fn create_flex_button(world: &mut World, text: String, flex_grow: f32, min_size: Option<egui::Vec2>) -> Entity {
    world.spawn((
        FlexStyle {
            flex_grow,
            ..Default::default()
        },
        FlexElement::Button { text, clicked: false, min_size },
    )).id()
}

pub fn create_flex_spacer(world: &mut World, flex_grow: f32, min_size: egui::Vec2) -> Entity {
    world.spawn((
        FlexStyle {
            flex_grow,
            min_size: taffy::Size {
                width: taffy::Dimension::Length(min_size.x),
                height: taffy::Dimension::Length(min_size.y),
            },
            ..Default::default()
        },
        FlexElement::Spacer { min_size },
    )).id()
}

pub fn create_flex_slider(world: &mut World, value: f32, range: std::ops::RangeInclusive<f32>, label: String, flex_grow: f32) -> Entity {
    world.spawn((
        FlexStyle {
            flex_grow,
            ..Default::default()
        },
        FlexElement::Slider { value, range, label },
    )).id()
}

pub fn add_flex_tab(world: &mut World, tab_name: String, direction: taffy::FlexDirection) -> Option<Entity> {
    let tab_root = world.spawn((
        TabId(tab_name.clone()),
        FlexStyle {
            flex_direction: direction,
            gap: taffy::Size::length(12.0),
            padding: taffy::Rect::length(16.0),
            ..Default::default()
        },
        FlexElement::Container { 
            label: None,
            show_border: false,
        },
    )).id();
    
    if let Some(mut dock_resource) = world.get_non_send_resource_mut::<FlexDockResource>() {
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