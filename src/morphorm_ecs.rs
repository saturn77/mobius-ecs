use bevy_ecs::prelude::*;
use bevy_ecs::system::RunSystemOnce;
use egui::{Context, Rect, Vec2};
use morphorm::{LayoutType, Units};
use std::collections::HashMap;

use crate::morphorm_bridge::{LayoutNode, MorphormLayoutBridge};

// ============================================================================
// ECS Components for Morphorm Layout
// ============================================================================

/// Core layout component that maps to Morphorm properties
#[derive(Component, Clone, Debug)]
pub struct LayoutComponent {
    pub layout_type: LayoutType,
    pub width: Units,
    pub height: Units,
}

impl Default for LayoutComponent {
    fn default() -> Self {
        Self {
            layout_type: LayoutType::Column,
            width: Units::Percentage(100.0),
            height: Units::Auto,
        }
    }
}

impl LayoutComponent {
    pub fn row() -> Self {
        Self {
            layout_type: LayoutType::Row,
            ..Default::default()
        }
    }

    pub fn column() -> Self {
        Self {
            layout_type: LayoutType::Column,
            ..Default::default()
        }
    }

    pub fn fixed_size(width: f32, height: f32) -> Self {
        Self {
            layout_type: LayoutType::Column,
            width: Units::Pixels(width),
            height: Units::Pixels(height),
        }
    }

    pub fn stretch() -> Self {
        Self {
            layout_type: LayoutType::Column,
            width: Units::Stretch(1.0),
            height: Units::Stretch(1.0),
        }
    }
}

/// Padding component using Morphorm Units
#[derive(Component, Clone, Debug)]
pub struct PaddingComponent {
    pub left: Units,
    pub right: Units,
    pub top: Units,
    pub bottom: Units,
}

impl Default for PaddingComponent {
    fn default() -> Self {
        Self {
            left: Units::Pixels(0.0),
            right: Units::Pixels(0.0),
            top: Units::Pixels(0.0),
            bottom: Units::Pixels(0.0),
        }
    }
}

impl PaddingComponent {
    pub fn all(padding: f32) -> Self {
        Self {
            left: Units::Pixels(padding),
            right: Units::Pixels(padding),
            top: Units::Pixels(padding),
            bottom: Units::Pixels(padding),
        }
    }

    pub fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self {
            left: Units::Pixels(horizontal),
            right: Units::Pixels(horizontal),
            top: Units::Pixels(vertical),
            bottom: Units::Pixels(vertical),
        }
    }
}

/// Margin component using Morphorm Units
#[derive(Component, Clone, Debug)]
pub struct MarginComponent {
    pub left: Units,
    pub right: Units,
    pub top: Units,
    pub bottom: Units,
}

impl Default for MarginComponent {
    fn default() -> Self {
        Self {
            left: Units::Pixels(0.0),
            right: Units::Pixels(0.0),
            top: Units::Pixels(0.0),
            bottom: Units::Pixels(0.0),
        }
    }
}

impl MarginComponent {
    pub fn all(margin: f32) -> Self {
        Self {
            left: Units::Pixels(margin),
            right: Units::Pixels(margin),
            top: Units::Pixels(margin),
            bottom: Units::Pixels(margin),
        }
    }
}

/// Stores computed bounds after layout calculation
#[derive(Component, Clone, Debug)]
pub struct ComputedBounds {
    pub rect: Rect,
}

impl Default for ComputedBounds {
    fn default() -> Self {
        Self {
            rect: Rect::NOTHING,
        }
    }
}

/// Marks an entity as a layout root with available size
#[derive(Component, Clone, Debug)]
pub struct LayoutRoot {
    pub available_size: Vec2,
}

/// Establishes parent-child relationships for layout
#[derive(Component, Clone, Debug)]
pub struct LayoutParent {
    pub parent: Entity,
}

/// Widget types that can be rendered
#[derive(Component, Clone, Debug)]
pub enum WidgetType {
    Panel { 
        color: egui::Color32,
        stroke: Option<egui::Stroke>,
    },
    Text { 
        content: String,
        size: Option<f32>,
        color: Option<egui::Color32>,
    },
    Button { 
        label: String,
        enabled: bool,
    },
    TextEdit {
        text: String,
        multiline: bool,
    },
}

// ============================================================================
// Resources
// ============================================================================

/// Resource that holds the Morphorm layout bridge and entity mappings
#[derive(Resource, Default)]
pub struct LayoutBridge {
    pub bridge: MorphormLayoutBridge,
    pub entity_to_node_id: HashMap<Entity, usize>,
    pub node_id_counter: usize,
}

// ============================================================================
// Systems
// ============================================================================

/// System that builds the Morphorm layout tree from ECS components
pub fn build_layout_tree_system(
    mut layout_bridge: ResMut<LayoutBridge>,
    root_query: Query<(Entity, &LayoutRoot, Option<&LayoutComponent>)>,
    layout_query: Query<(
        Entity,
        &LayoutComponent,
        Option<&PaddingComponent>,
        Option<&MarginComponent>,
    )>,
    children_query: Query<&Children>,
) {
    // Clear previous layout
    layout_bridge.bridge.clear();
    layout_bridge.entity_to_node_id.clear();
    layout_bridge.node_id_counter = 0;

    // Process each root entity
    for (root_entity, root_layout, root_component) in root_query.iter() {
        // Setup root with available size
        layout_bridge.bridge.setup_root(root_layout.available_size);
        
        // Apply root layout properties if present
        if let Some(layout) = root_component {
            let root_node = &mut layout_bridge.bridge.root;
            root_node.layout_type = layout.layout_type;
            root_node.width = layout.width;
            root_node.height = layout.height;
        }

        // Build tree for root's children
        if let Ok(children) = children_query.get(root_entity) {
            for child in children.iter() {
                build_node_recursive(
                    child,
                    None,
                    &mut layout_bridge,
                    &layout_query,
                    &children_query,
                );
            }
        }
    }
}

/// Recursively builds layout nodes from entities
fn build_node_recursive(
    entity: Entity,
    parent_index: Option<usize>,
    layout_bridge: &mut ResMut<LayoutBridge>,
    layout_query: &Query<(
        Entity,
        &LayoutComponent,
        Option<&PaddingComponent>,
        Option<&MarginComponent>,
    )>,
    children_query: &Query<&Children>,
) -> Option<usize> {
    // Get layout component for this entity
    if let Ok((_, layout, padding, margin)) = layout_query.get(entity) {
        // Create layout node
        let mut node = LayoutNode::new();
        node.layout_type = layout.layout_type;
        node.width = layout.width;
        node.height = layout.height;

        // Apply padding if present
        if let Some(padding) = padding {
            node.padding_left = padding.left;
            node.padding_right = padding.right;
            node.padding_top = padding.top;
            node.padding_bottom = padding.bottom;
        }

        // Add node to parent or root
        if parent_index.is_none() {
            // Add to root
            layout_bridge.bridge.root.add_child(node);
        } else {
            // This would require more complex tree navigation
            // For now, we'll add directly to root
            layout_bridge.bridge.root.add_child(node);
        }

        // Map entity to node
        let node_id = layout_bridge.node_id_counter;
        layout_bridge.entity_to_node_id.insert(entity, node_id);
        layout_bridge.node_id_counter += 1;

        // Process children
        if let Ok(children) = children_query.get(entity) {
            for child in children.iter() {
                build_node_recursive(
                    child,
                    Some(node_id),
                    layout_bridge,
                    layout_query,
                    children_query,
                );
            }
        }

        Some(node_id)
    } else {
        None
    }
}

/// System that computes layout and updates computed bounds
pub fn compute_layout_system(
    mut layout_bridge: ResMut<LayoutBridge>,
    root_query: Query<&LayoutRoot>,
    mut bounds_query: Query<&mut ComputedBounds>,
) {
    // Compute layout for each root
    for root_layout in root_query.iter() {
        if let Err(e) = layout_bridge.bridge.compute_layout(root_layout.available_size) {
            eprintln!("Layout computation error: {}", e);
            continue;
        }

        // Update computed bounds for all entities
        for (entity, &node_id) in layout_bridge.entity_to_node_id.iter() {
            if let Some(rect) = layout_bridge.bridge.get_bounds(node_id) {
                if let Ok(mut bounds) = bounds_query.get_mut(*entity) {
                    bounds.rect = rect;
                }
            }
        }
    }
}

/// System that renders widgets using computed bounds
pub fn render_widgets_system(
    ctx: Context,
    widget_query: Query<(Entity, &ComputedBounds, &WidgetType), Without<LayoutRoot>>,
) {
    for (entity, bounds, widget) in widget_query.iter() {
        // Skip if bounds are invalid
        if bounds.rect.width() <= 0.0 || bounds.rect.height() <= 0.0 {
            continue;
        }

        // Create an area for this widget using the entity ID
        let widget_id = egui::Id::new(entity);
        
        // Determine if this widget should be interactable
        let interactable = matches!(widget, WidgetType::Button { .. } | WidgetType::TextEdit { .. });
        
        egui::Area::new(widget_id)
            .fixed_pos(bounds.rect.min)
            .interactable(interactable)
            .show(&ctx, |ui| {
                ui.set_max_size(bounds.rect.size());
                
                match widget {
                    WidgetType::Panel { color, stroke } => {
                        let mut frame = egui::Frame::new().fill(*color);
                        if let Some(stroke) = stroke {
                            frame = frame.stroke(*stroke);
                        }
                        frame.show(ui, |ui| {
                            ui.set_min_size(bounds.rect.size());
                            // Add some invisible content to prevent area collapse
                            ui.allocate_space(bounds.rect.size());
                        });
                    }
                    WidgetType::Text { content, size, color } => {
                        let mut text = egui::RichText::new(content);
                        if let Some(size) = size {
                            text = text.size(*size);
                        }
                        if let Some(color) = color {
                            text = text.color(*color);
                        }
                        ui.label(text);
                    }
                    WidgetType::Button { label, enabled } => {
                        ui.add_enabled(*enabled, egui::Button::new(label));
                    }
                    WidgetType::TextEdit { text, multiline } => {
                        let mut text_copy = text.clone();
                        if *multiline {
                            ui.text_edit_multiline(&mut text_copy);
                        } else {
                            ui.text_edit_singleline(&mut text_copy);
                        }
                    }
                }
            });
    }
}

// ============================================================================
// Bundles for convenience
// ============================================================================

/// Basic layout bundle
#[derive(Bundle)]
pub struct LayoutBundle {
    pub layout: LayoutComponent,
    pub bounds: ComputedBounds,
}

impl Default for LayoutBundle {
    fn default() -> Self {
        Self {
            layout: LayoutComponent::default(),
            bounds: ComputedBounds::default(),
        }
    }
}

/// Panel widget bundle
#[derive(Bundle)]
pub struct PanelBundle {
    pub layout: LayoutComponent,
    pub bounds: ComputedBounds,
    pub widget: WidgetType,
}

impl PanelBundle {
    pub fn new(color: egui::Color32) -> Self {
        Self {
            layout: LayoutComponent::default(),
            bounds: ComputedBounds::default(),
            widget: WidgetType::Panel { color, stroke: None },
        }
    }

    pub fn with_stroke(mut self, stroke_val: egui::Stroke) -> Self {
        if let WidgetType::Panel { ref mut stroke, .. } = self.widget {
            *stroke = Some(stroke_val);
        }
        self
    }
}

/// Text widget bundle
#[derive(Bundle)]
pub struct TextBundle {
    pub layout: LayoutComponent,
    pub bounds: ComputedBounds,
    pub widget: WidgetType,
}

impl TextBundle {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            layout: LayoutComponent::default(),
            bounds: ComputedBounds::default(),
            widget: WidgetType::Text {
                content: text.into(),
                size: None,
                color: None,
            },
        }
    }
}

// ============================================================================
// Helper functions
// ============================================================================

/// Initialize the Morphorm ECS integration for a bevy App
pub fn setup_morphorm_ecs(world: &mut World) {
    world.insert_resource(LayoutBridge::default());
}

/// Run layout computation for immediate mode usage
pub fn compute_layout_immediate(world: &mut World) {
    let _ = world.run_system_once(build_layout_tree_system);
    let _ = world.run_system_once(compute_layout_system);
}

/// Render all widgets for immediate mode usage
pub fn render_widgets_immediate(world: &mut World, ctx: &Context) {
    // Extract the data we need
    let mut widgets_data = Vec::new();
    
    let mut query = world.query_filtered::<(Entity, &ComputedBounds, &WidgetType), Without<LayoutRoot>>();
    for (entity, bounds, widget) in query.iter(world) {
        widgets_data.push((entity, bounds.clone(), widget.clone()));
    }
    
    // Render widgets
    for (entity, bounds, widget) in widgets_data {
        // Skip if bounds are invalid
        if bounds.rect.width() <= 0.0 || bounds.rect.height() <= 0.0 {
            continue;
        }

        // Create an area for this widget using the entity ID
        let widget_id = egui::Id::new(entity);
        
        // Determine if this widget should be interactable
        let interactable = matches!(widget, WidgetType::Button { .. } | WidgetType::TextEdit { .. });
        
        egui::Area::new(widget_id)
            .fixed_pos(bounds.rect.min)
            .interactable(interactable)
            .show(ctx, |ui| {
                ui.set_max_size(bounds.rect.size());
                
                match widget {
                    WidgetType::Panel { color, stroke } => {
                        let mut frame = egui::Frame::new().fill(color);
                        if let Some(stroke) = stroke {
                            frame = frame.stroke(stroke);
                        }
                        frame.show(ui, |ui| {
                            ui.set_min_size(bounds.rect.size());
                            // Add some invisible content to prevent area collapse
                            ui.allocate_space(bounds.rect.size());
                        });
                    }
                    WidgetType::Text { content, size, color } => {
                        let mut text = egui::RichText::new(content);
                        if let Some(size) = size {
                            text = text.size(size);
                        }
                        if let Some(color) = color {
                            text = text.color(color);
                        }
                        ui.label(text);
                    }
                    WidgetType::Button { label, enabled } => {
                        ui.add_enabled(enabled, egui::Button::new(label));
                    }
                    WidgetType::TextEdit { text, multiline } => {
                        let mut text_copy = text.clone();
                        if multiline {
                            ui.text_edit_multiline(&mut text_copy);
                        } else {
                            ui.text_edit_singleline(&mut text_copy);
                        }
                    }
                }
            });
    }
}