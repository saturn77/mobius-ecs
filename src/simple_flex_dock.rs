use bevy_ecs::prelude::*;
use egui_dock::{DockArea, DockState, Style as DockStyle, TabViewer};
use egui::{Context, Ui, Layout, Align};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ============================================================================
// Simple Flex Components using native egui layouts
// ============================================================================

#[derive(Component, Clone, Debug)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Component, Clone, Debug)]
pub enum FlexJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

#[derive(Component, Clone, Debug)]
pub enum FlexAlign {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Component, Clone, Debug)]
pub struct FlexGrow(pub f32);

#[derive(Component, Clone, Debug)]
pub struct FlexShrink(pub f32);

#[derive(Component, Clone, Debug)]
pub struct FlexBasis(pub f32);

#[derive(Component, Clone, Debug)]
pub struct Padding(pub f32);

#[derive(Component, Clone, Debug)]
pub struct Margin(pub f32);

#[derive(Component, Clone, Debug)]
pub struct MinSize(pub egui::Vec2);

#[derive(Component, Clone, Debug)]
pub struct MaxSize(pub egui::Vec2);

#[derive(Component, Clone, Debug)]
pub enum SimpleFlexElement {
    Container {
        label: Option<String>,
        show_border: bool,
        background_color: Option<egui::Color32>,
    },
    Label {
        text: String,
        color: Option<egui::Color32>,
        size: Option<f32>,
    },
    Button {
        text: String,
        clicked: bool,
        color: Option<egui::Color32>,
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
        size: egui::Vec2,
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

pub struct SimpleFlexTabViewer {
    pub tabs: HashMap<String, TabContent>,
    pub world_ref: Arc<Mutex<*mut World>>,
}

unsafe impl Send for SimpleFlexTabViewer {}
unsafe impl Sync for SimpleFlexTabViewer {}

impl TabViewer for SimpleFlexTabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.clone().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        if let Some(tab_content) = self.tabs.get(tab) {
            let world_ptr = *self.world_ref.lock().unwrap();
            let world = unsafe { &mut *world_ptr };
            
            // Use simple egui layouts
            render_simple_flex_entity(ui, world, tab_content.entity);
        }
    }
}

// ============================================================================
// Simple Flex Rendering
// ============================================================================

fn render_simple_flex_entity(ui: &mut Ui, world: &mut World, entity: Entity) {
    // Get layout properties
    let direction = world.get::<FlexDirection>(entity).cloned().unwrap_or(FlexDirection::Column);
    let justify = world.get::<FlexJustify>(entity).cloned().unwrap_or(FlexJustify::Start);
    let align = world.get::<FlexAlign>(entity).cloned().unwrap_or(FlexAlign::Start);
    let padding = world.get::<Padding>(entity).map(|p| p.0).unwrap_or(0.0);
    let margin = world.get::<Margin>(entity).map(|m| m.0).unwrap_or(0.0);
    
    // Apply margin
    if margin > 0.0 {
        ui.add_space(margin);
    }
    
    // Create appropriate layout
    let layout = match (direction, justify, align) {
        (FlexDirection::Row, FlexJustify::Start, FlexAlign::Start) => Layout::left_to_right(Align::TOP),
        (FlexDirection::Row, FlexJustify::Start, FlexAlign::Center) => Layout::left_to_right(Align::Center),
        (FlexDirection::Row, FlexJustify::Start, FlexAlign::End) => Layout::left_to_right(Align::BOTTOM),
        (FlexDirection::Row, FlexJustify::Center, _) => Layout::centered_and_justified(egui::Direction::LeftToRight),
        (FlexDirection::Row, FlexJustify::End, FlexAlign::Start) => Layout::right_to_left(Align::TOP),
        (FlexDirection::Row, FlexJustify::End, FlexAlign::Center) => Layout::right_to_left(Align::Center),
        (FlexDirection::Row, FlexJustify::End, FlexAlign::End) => Layout::right_to_left(Align::BOTTOM),
        (FlexDirection::Column, FlexJustify::Start, FlexAlign::Start) => Layout::top_down(Align::LEFT),
        (FlexDirection::Column, FlexJustify::Start, FlexAlign::Center) => Layout::top_down(Align::Center),
        (FlexDirection::Column, FlexJustify::Start, FlexAlign::End) => Layout::top_down(Align::RIGHT),
        (FlexDirection::Column, FlexJustify::Center, _) => Layout::centered_and_justified(egui::Direction::TopDown),
        (FlexDirection::Column, FlexJustify::End, FlexAlign::Start) => Layout::bottom_up(Align::LEFT),
        (FlexDirection::Column, FlexJustify::End, FlexAlign::Center) => Layout::bottom_up(Align::Center),
        (FlexDirection::Column, FlexJustify::End, FlexAlign::End) => Layout::bottom_up(Align::RIGHT),
        _ => Layout::top_down(Align::LEFT),
    };
    
    // Get element
    let element = world.get::<SimpleFlexElement>(entity).cloned();
    
    // Apply padding and render in layout
    let response = if padding > 0.0 {
        ui.group(|ui| {
            ui.style_mut().spacing.indent = padding;
            ui.with_layout(layout, |ui| {
                render_element_content(ui, world, entity, element)
            }).inner
        }).inner
    } else {
        ui.with_layout(layout, |ui| {
            render_element_content(ui, world, entity, element)
        }).inner
    };
    
    // Apply margin after
    if margin > 0.0 {
        ui.add_space(margin);
    }
}

fn render_element_content(ui: &mut Ui, world: &mut World, entity: Entity, element: Option<SimpleFlexElement>) -> egui::Response {
    match element {
        Some(SimpleFlexElement::Container { label, show_border, background_color }) => {
            if let Some(label_text) = label {
                ui.heading(&label_text);
            }
            
            let frame = if show_border {
                egui::Frame::group(ui.style())
            } else {
                egui::Frame::new()
            };
            
            let frame = if let Some(color) = background_color {
                frame.fill(color)
            } else {
                frame
            };
            
            frame.show(ui, |ui| {
                render_children(ui, world, entity);
            }).response
        }
        Some(SimpleFlexElement::Label { text, color, size }) => {
            let mut job = egui::text::LayoutJob::default();
            job.append(&text, 0.0, egui::TextFormat {
                color: color.unwrap_or(ui.style().visuals.text_color()),
                font_id: egui::FontId::proportional(size.unwrap_or(14.0)),
                ..Default::default()
            });
            ui.label(job)
        }
        Some(SimpleFlexElement::Button { text, color, .. }) => {
            let mut button = egui::Button::new(&text);
            if let Some(color) = color {
                button = button.fill(color);
            }
            
            let response = ui.add(button);
            if response.clicked() {
                // Update clicked state
                if let Some(mut elem) = world.get_mut::<SimpleFlexElement>(entity) {
                    if let SimpleFlexElement::Button { clicked, .. } = &mut *elem {
                        *clicked = true;
                    }
                }
            }
            response
        }
        Some(SimpleFlexElement::TextInput { mut value, hint, multiline }) => {
            let response = if multiline {
                ui.text_edit_multiline(&mut value)
            } else {
                ui.add(egui::TextEdit::singleline(&mut value).hint_text(&hint))
            };
            
            if response.changed() {
                if let Some(mut elem) = world.get_mut::<SimpleFlexElement>(entity) {
                    if let SimpleFlexElement::TextInput { value: v, .. } = &mut *elem {
                        *v = value;
                    }
                }
            }
            response
        }
        Some(SimpleFlexElement::Slider { mut value, range, label }) => {
            let response = ui.add(egui::Slider::new(&mut value, range).text(&label));
            
            if response.changed() {
                if let Some(mut elem) = world.get_mut::<SimpleFlexElement>(entity) {
                    if let SimpleFlexElement::Slider { value: v, .. } = &mut *elem {
                        *v = value;
                    }
                }
            }
            response
        }
        Some(SimpleFlexElement::ProgressBar { progress, label }) => {
            ui.add(egui::ProgressBar::new(progress).text(&label))
        }
        Some(SimpleFlexElement::Separator) => {
            ui.separator()
        }
        Some(SimpleFlexElement::Spacer { size }) => {
            ui.allocate_response(size, egui::Sense::hover())
        }
        None => {
            // Just render children
            render_children(ui, world, entity);
            ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover())
        }
    }
}

fn render_children(ui: &mut Ui, world: &mut World, entity: Entity) {
    let children_list: Vec<Entity> = world.get::<Children>(entity)
        .map(|children| children.iter().collect())
        .unwrap_or_default();
    
    for child in children_list {
        render_simple_flex_entity(ui, world, child);
    }
}

// ============================================================================
// Dock System
// ============================================================================

pub struct SimpleFlexDockResource {
    pub dock_state: DockState<String>,
    pub tab_viewer: SimpleFlexTabViewer,
}

pub fn setup_simple_flex_dock(world: &mut World) {
    let dock_state = DockState::new(vec!["Main".to_string()]);
    
    let mut tabs = HashMap::new();
    
    // Create main tab root
    let main_tab_root = world.spawn((
        TabId("Main".to_string()),
        FlexDirection::Column,
        FlexJustify::Start,
        FlexAlign::Stretch,
        Padding(16.0),
        SimpleFlexElement::Container {
            label: None,
            show_border: false,
            background_color: None,
        },
    )).id();
    
    tabs.insert("Main".to_string(), TabContent {
        entity: main_tab_root,
    });
    
    let tab_viewer = SimpleFlexTabViewer {
        tabs,
        world_ref: Arc::new(Mutex::new(world as *mut World)),
    };
    
    world.insert_non_send_resource(SimpleFlexDockResource {
        dock_state,
        tab_viewer,
    });
}

pub fn render_simple_flex_dock(ctx: &Context, world: &mut World) {
    if let Some(mut dock_resource) = world.remove_non_send_resource::<SimpleFlexDockResource>() {
        // Update world pointer
        *dock_resource.tab_viewer.world_ref.lock().unwrap() = world as *mut World;
        
        DockArea::new(&mut dock_resource.dock_state)
            .style(DockStyle::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut dock_resource.tab_viewer);
        
        world.insert_non_send_resource(dock_resource);
    }
}

// ============================================================================
// Helper functions
// ============================================================================

pub fn create_flex_container_simple(
    world: &mut World,
    direction: FlexDirection,
    justify: FlexJustify,
    align: FlexAlign,
    label: Option<String>,
    show_border: bool,
    background_color: Option<egui::Color32>,
) -> Entity {
    world.spawn((
        direction,
        justify,
        align,
        Padding(8.0),
        SimpleFlexElement::Container {
            label,
            show_border,
            background_color,
        },
    )).id()
}

pub fn create_flex_label_simple(world: &mut World, text: String, color: Option<egui::Color32>, size: Option<f32>) -> Entity {
    world.spawn((
        SimpleFlexElement::Label { text, color, size },
        Margin(4.0),
    )).id()
}

pub fn create_flex_button_simple(world: &mut World, text: String, color: Option<egui::Color32>) -> Entity {
    world.spawn((
        SimpleFlexElement::Button { text, clicked: false, color },
        Margin(4.0),
    )).id()
}

pub fn create_flex_spacer_simple(world: &mut World, size: egui::Vec2) -> Entity {
    world.spawn((
        SimpleFlexElement::Spacer { size },
    )).id()
}

pub fn create_flex_slider_simple(world: &mut World, value: f32, range: std::ops::RangeInclusive<f32>, label: String) -> Entity {
    world.spawn((
        SimpleFlexElement::Slider { value, range, label },
        Margin(4.0),
    )).id()
}

pub fn add_simple_flex_tab(world: &mut World, tab_name: String, direction: FlexDirection) -> Option<Entity> {
    let tab_root = world.spawn((
        TabId(tab_name.clone()),
        direction,
        FlexJustify::Start,
        FlexAlign::Start,
        Padding(16.0),
        SimpleFlexElement::Container {
            label: None,
            show_border: false,
            background_color: None,
        },
    )).id();
    
    if let Some(mut dock_resource) = world.get_non_send_resource_mut::<SimpleFlexDockResource>() {
        dock_resource.tab_viewer.tabs.insert(tab_name.clone(), TabContent {
            entity: tab_root,
        });
        
        dock_resource.dock_state.push_to_focused_leaf(tab_name);
        
        Some(tab_root)
    } else {
        world.despawn(tab_root);
        None
    }
}