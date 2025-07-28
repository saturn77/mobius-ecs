use bevy_ecs::prelude::*;
use bevy_ecs::system::RunSystemOnce;
use egui::{Context, Rect, Vec2};
use morphorm::{LayoutType, Units};

// ============================================================================
// Simple ECS Components
// ============================================================================

#[derive(Component, Clone, Debug)]
pub struct SimpleLayout {
    pub width: Units,
    pub height: Units,
    pub layout_type: LayoutType,
}

impl SimpleLayout {
    pub fn fixed(width: f32, height: f32) -> Self {
        Self {
            width: Units::Pixels(width),
            height: Units::Pixels(height),
            layout_type: LayoutType::Column,
        }
    }

    pub fn stretch_horizontal() -> Self {
        Self {
            width: Units::Stretch(1.0),
            height: Units::Pixels(100.0),
            layout_type: LayoutType::Column,
        }
    }

    pub fn stretch_vertical() -> Self {
        Self {
            width: Units::Pixels(200.0),
            height: Units::Stretch(1.0),
            layout_type: LayoutType::Column,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct LayoutBounds {
    pub rect: Rect,
}

impl Default for LayoutBounds {
    fn default() -> Self {
        Self {
            rect: Rect::NOTHING,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct LayoutRoot {
    pub size: Vec2,
}

#[derive(Component, Clone, Debug)]
pub enum SimpleWidget {
    Panel { color: egui::Color32 },
    Text { content: String, size: f32, color: egui::Color32 },
    Button { label: String },
}

// ============================================================================
// Simple Layout System
// ============================================================================

pub fn simple_layout_system(
    root_query: Query<(Entity, &LayoutRoot), With<LayoutRoot>>,
    mut layout_query: Query<(&mut LayoutBounds, &SimpleLayout), Without<LayoutRoot>>,
    children_query: Query<&Children>,
) {
    for (root_entity, root) in root_query.iter() {
        // Simple manual layout for demonstration
        let mut current_y = 0.0;
        
        if let Ok(children) = children_query.get(root_entity) {
            for child in children.iter() {
                if let Ok((mut bounds, layout)) = layout_query.get_mut(child) {
                    let width = match layout.width {
                        Units::Pixels(w) => w,
                        Units::Percentage(pct) => root.size.x * (pct / 100.0),
                        Units::Stretch(_) => root.size.x,
                        _ => 100.0,
                    };
                    
                    let height = match layout.height {
                        Units::Pixels(h) => h,
                        Units::Percentage(pct) => root.size.y * (pct / 100.0),
                        Units::Stretch(_) => root.size.y - current_y,
                        _ => 100.0,
                    };

                    bounds.rect = Rect::from_min_size(
                        egui::pos2(0.0, current_y),
                        egui::vec2(width, height),
                    );

                    current_y += height;
                }
            }
        }
    }
}

pub fn simple_render_system(
    ctx: &Context,
    widget_query: Query<(Entity, &LayoutBounds, &SimpleWidget)>,
) {
    for (entity, bounds, widget) in widget_query.iter() {
        if bounds.rect.width() <= 0.0 || bounds.rect.height() <= 0.0 {
            continue;
        }

        let widget_id = egui::Id::new(entity);
        
        egui::Area::new(widget_id)
            .fixed_pos(bounds.rect.min)
            .interactable(matches!(widget, SimpleWidget::Button { .. }))
            .show(ctx, |ui| {
                ui.set_max_size(bounds.rect.size());
                
                match widget {
                    SimpleWidget::Panel { color } => {
                        egui::Frame::new()
                            .fill(*color)
                            .show(ui, |ui| {
                                ui.set_min_size(bounds.rect.size());
                            });
                    }
                    SimpleWidget::Text { content, size, color } => {
                        let text = egui::RichText::new(content)
                            .size(*size)
                            .color(*color);
                        ui.label(text);
                    }
                    SimpleWidget::Button { label } => {
                        ui.button(label);
                    }
                }
            });
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

pub fn setup_simple_layout(_world: &mut World) {
    // Nothing special needed for setup
}

pub fn run_simple_layout(world: &mut World, ctx: &Context) {
    // Run layout system
    let _ = world.run_system_once(simple_layout_system);
    
    // Extract data for rendering
    let mut render_data = Vec::new();
    let mut query = world.query::<(Entity, &LayoutBounds, &SimpleWidget)>();
    for (entity, bounds, widget) in query.iter(world) {
        render_data.push((entity, bounds.clone(), widget.clone()));
    }
    
    // Render widgets
    for (entity, bounds, widget) in render_data {
        if bounds.rect.width() <= 0.0 || bounds.rect.height() <= 0.0 {
            continue;
        }

        let widget_id = egui::Id::new(entity);
        
        egui::Area::new(widget_id)
            .fixed_pos(bounds.rect.min)
            .interactable(matches!(widget, SimpleWidget::Button { .. }))
            .show(ctx, |ui| {
                ui.set_max_size(bounds.rect.size());
                
                match widget {
                    SimpleWidget::Panel { color } => {
                        egui::Frame::new()
                            .fill(color)
                            .show(ui, |ui| {
                                ui.set_min_size(bounds.rect.size());
                            });
                    }
                    SimpleWidget::Text { content, size, color } => {
                        let text = egui::RichText::new(&content)
                            .size(size)
                            .color(color);
                        ui.label(text);
                    }
                    SimpleWidget::Button { label } => {
                        ui.button(&label);
                    }
                }
            });
    }
}

// ============================================================================
// Bundles
// ============================================================================

#[derive(Bundle)]
pub struct PanelBundle {
    pub layout: SimpleLayout,
    pub bounds: LayoutBounds,
    pub widget: SimpleWidget,
}

impl PanelBundle {
    pub fn new(color: egui::Color32, layout: SimpleLayout) -> Self {
        Self {
            layout,
            bounds: LayoutBounds::default(),
            widget: SimpleWidget::Panel { color },
        }
    }
}

#[derive(Bundle)]
pub struct TextBundle {
    pub layout: SimpleLayout,
    pub bounds: LayoutBounds,
    pub widget: SimpleWidget,
}

impl TextBundle {
    pub fn new(content: String, size: f32, color: egui::Color32, layout: SimpleLayout) -> Self {
        Self {
            layout,
            bounds: LayoutBounds::default(),
            widget: SimpleWidget::Text { content, size, color },
        }
    }
}