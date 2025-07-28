use bevy_ecs::prelude::*;
use bevy_ecs::system::RunSystemOnce;
use eframe::egui;
use morphorm::{LayoutType, Units};

// ============================================================================
// Simple ECS Components (inlined for this demo)
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
}

#[derive(Component, Clone, Debug)]
pub struct LayoutBounds {
    pub rect: egui::Rect,
}

impl Default for LayoutBounds {
    fn default() -> Self {
        Self {
            rect: egui::Rect::NOTHING,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct LayoutRoot {
    pub size: egui::Vec2,
}

#[derive(Component, Clone, Debug)]
pub enum SimpleWidget {
    Panel { color: egui::Color32 },
    Text { content: String, size: f32, color: egui::Color32 },
}

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

// ============================================================================
// Simple Layout System
// ============================================================================

pub fn simple_layout_system(
    root_query: Query<(Entity, &LayoutRoot), With<LayoutRoot>>,
    mut layout_query: Query<(&mut LayoutBounds, &SimpleLayout), Without<LayoutRoot>>,
    children_query: Query<&Children>,
) {
    for (root_entity, root) in root_query.iter() {
        layout_entity(root_entity, root.size, &mut layout_query, &children_query);
    }
}

fn layout_entity(
    entity: Entity,
    available_size: egui::Vec2,
    layout_query: &mut Query<(&mut LayoutBounds, &SimpleLayout), Without<LayoutRoot>>,
    children_query: &Query<&Children>,
) {
    if let Ok(children) = children_query.get(entity) {
        let mut child_layouts = Vec::new();
        for child in children.iter() {
            if let Ok((_, layout)) = layout_query.get(child) {
                child_layouts.push((child, layout.clone()));
            }
        }

        if child_layouts.is_empty() {
            return;
        }

        // Get the layout type of the container (default to Column if not found)
        let container_layout_type = if let Ok((_, container_layout)) = layout_query.get(entity) {
            container_layout.layout_type
        } else {
            LayoutType::Column
        };

        match container_layout_type {
            LayoutType::Column => layout_column(entity, available_size, child_layouts, layout_query, children_query),
            LayoutType::Row => layout_row(entity, available_size, child_layouts, layout_query, children_query),
            LayoutType::Grid => layout_column(entity, available_size, child_layouts, layout_query, children_query), // Default to column for now
        }
    }
}

fn layout_column(
    _entity: Entity,
    available_size: egui::Vec2,
    child_layouts: Vec<(Entity, SimpleLayout)>,
    layout_query: &mut Query<(&mut LayoutBounds, &SimpleLayout), Without<LayoutRoot>>,
    children_query: &Query<&Children>,
) {
    // Calculate fixed height and count stretch items
    let mut fixed_height = 0.0;
    let mut stretch_count = 0;
    
    for (_, layout) in &child_layouts {
        match layout.height {
            Units::Pixels(h) => fixed_height += h,
            Units::Stretch(_) => stretch_count += 1,
            _ => {}
        }
    }
    
    let remaining_height = (available_size.y - fixed_height).max(0.0);
    let stretch_height = if stretch_count > 0 { remaining_height / stretch_count as f32 } else { 0.0 };
    
    // Layout children vertically
    let mut current_y = 0.0;
    
    for (child, layout) in child_layouts {
        if let Ok((mut bounds, _)) = layout_query.get_mut(child) {
            let width = match layout.width {
                Units::Pixels(w) => w,
                Units::Percentage(pct) => available_size.x * (pct / 100.0),
                Units::Stretch(_) => available_size.x,
                _ => available_size.x,
            };
            
            let height = match layout.height {
                Units::Pixels(h) => h,
                Units::Percentage(pct) => available_size.y * (pct / 100.0),
                Units::Stretch(_) => stretch_height,
                _ => 100.0,
            };

            bounds.rect = egui::Rect::from_min_size(
                egui::pos2(0.0, current_y),
                egui::vec2(width, height),
            );

            current_y += height;
            
            // Recursively layout this child's children
            layout_entity(child, egui::vec2(width, height), layout_query, children_query);
        }
    }
}

fn layout_row(
    _entity: Entity,
    available_size: egui::Vec2,
    child_layouts: Vec<(Entity, SimpleLayout)>,
    layout_query: &mut Query<(&mut LayoutBounds, &SimpleLayout), Without<LayoutRoot>>,
    children_query: &Query<&Children>,
) {
    // Calculate fixed width and count stretch items
    let mut fixed_width = 0.0;
    let mut stretch_count = 0;
    
    for (_, layout) in &child_layouts {
        match layout.width {
            Units::Pixels(w) => fixed_width += w,
            Units::Stretch(_) => stretch_count += 1,
            _ => {}
        }
    }
    
    let remaining_width = (available_size.x - fixed_width).max(0.0);
    let stretch_width = if stretch_count > 0 { remaining_width / stretch_count as f32 } else { 0.0 };
    
    // Layout children horizontally
    let mut current_x = 0.0;
    
    for (child, layout) in child_layouts {
        if let Ok((mut bounds, _)) = layout_query.get_mut(child) {
            let width = match layout.width {
                Units::Pixels(w) => w,
                Units::Percentage(pct) => available_size.x * (pct / 100.0),
                Units::Stretch(_) => stretch_width,
                _ => 100.0,
            };
            
            let height = match layout.height {
                Units::Pixels(h) => h,
                Units::Percentage(pct) => available_size.y * (pct / 100.0),
                Units::Stretch(_) => available_size.y,
                _ => available_size.y,
            };

            bounds.rect = egui::Rect::from_min_size(
                egui::pos2(current_x, 0.0),
                egui::vec2(width, height),
            );

            current_x += width;
            
            // Recursively layout this child's children
            layout_entity(child, egui::vec2(width, height), layout_query, children_query);
        }
    }
}

pub fn run_simple_layout(world: &mut World, ctx: &egui::Context) {
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
            .interactable(false)
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
                }
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Standalone Simple Morphorm Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "standalone_simple_demo",
        options,
        Box::new(|_cc| Ok(Box::new(StandaloneSimpleDemo::new()))),
    )
}

struct StandaloneSimpleDemo {
    world: World,
    header_text: String,
    sidebar_visible: bool,
    header_entity: Option<Entity>,
    sidebar_entity: Option<Entity>,
    content_entity: Option<Entity>,
    footer_entity: Option<Entity>,
}

impl StandaloneSimpleDemo {
    fn new() -> Self {
        let world = World::new();
        
        let mut demo = Self {
            world,
            header_text: "Welcome to Simple Demo".to_string(),
            sidebar_visible: true,
            header_entity: None,
            sidebar_entity: None,
            content_entity: None,
            footer_entity: None,
        };
        
        demo.setup_ui();
        demo
    }

    fn setup_ui(&mut self) {
        // Create root entity
        let root = self.world.spawn(LayoutRoot { 
            size: egui::vec2(800.0, 600.0) 
        }).id();

        // Create header (full width, fixed height)
        let header = self.world.spawn(TextBundle::new(
            "Header Text".to_string(),
            18.0,
            egui::Color32::WHITE,
            SimpleLayout::fixed(800.0, 60.0),
        )).id();
        self.header_entity = Some(header);

        // Create a container for sidebar + content (stretches to fill remaining height)
        let body_container = self.world.spawn((
            SimpleLayout {
                width: Units::Stretch(1.0),
                height: Units::Stretch(1.0),
                layout_type: LayoutType::Row, // Horizontal layout
            },
            LayoutBounds::default(),
        )).id();

        // Create sidebar (fixed width when visible)
        let sidebar = self.world.spawn(PanelBundle::new(
            egui::Color32::from_rgb(60, 60, 60),
            SimpleLayout::fixed(200.0, 0.0), // Height will be set by parent
        )).id();
        self.sidebar_entity = Some(sidebar);

        // Create content area (stretches to fill remaining width)
        let content = self.world.spawn(PanelBundle::new(
            egui::Color32::from_rgb(40, 40, 40),
            SimpleLayout {
                width: Units::Stretch(1.0),
                height: Units::Stretch(1.0),
                layout_type: LayoutType::Column,
            },
        )).id();
        self.content_entity = Some(content);

        // Create footer container with dark background
        let footer_container = self.world.spawn((
            SimpleLayout::fixed(800.0, 30.0),
            LayoutBounds::default(),
            SimpleWidget::Panel { color: egui::Color32::from_rgb(30, 30, 30) },
        )).id();
        
        // Create footer text
        let footer = self.world.spawn(TextBundle::new(
            "Status: Ready | Entities: 0 | FPS: 60".to_string(),
            14.0,
            egui::Color32::LIGHT_GRAY,
            SimpleLayout::fixed(800.0, 30.0),
        )).id();
        self.footer_entity = Some(footer);
        
        // Add footer text to footer container
        self.world.entity_mut(footer_container).add_child(footer);

        // Set up hierarchy
        self.world.entity_mut(root).add_child(header);
        self.world.entity_mut(root).add_child(body_container);
        self.world.entity_mut(root).add_child(footer_container);
        
        // Add sidebar and content to the body container
        if self.sidebar_visible {
            self.world.entity_mut(body_container).add_child(sidebar);
        }
        self.world.entity_mut(body_container).add_child(content);
    }

    fn update_layout(&mut self, available_size: egui::Vec2) {
        // Update root size
        let mut root_query = self.world.query_filtered::<&mut LayoutRoot, With<LayoutRoot>>();
        for mut root in root_query.iter_mut(&mut self.world) {
            root.size = available_size;
        }

        // Update header text
        if let Some(header) = self.header_entity {
            if let Some(mut widget) = self.world.get_mut::<SimpleWidget>(header) {
                if let SimpleWidget::Text { content, .. } = &mut *widget {
                    *content = self.header_text.clone();
                }
            }
        }

        // Toggle sidebar visibility by adjusting its width
        if let Some(sidebar) = self.sidebar_entity {
            if let Some(mut layout) = self.world.get_mut::<SimpleLayout>(sidebar) {
                if self.sidebar_visible {
                    layout.width = Units::Pixels(200.0);
                } else {
                    layout.width = Units::Pixels(0.0);
                }
            }
        }

        // Update footer text with entity count
        if let Some(footer) = self.footer_entity {
            let entity_count = self.world.entities().len();
            if let Some(mut widget) = self.world.get_mut::<SimpleWidget>(footer) {
                if let SimpleWidget::Text { content, .. } = &mut *widget {
                    *content = format!("Status: Ready | Entities: {} | Sidebar: {}", 
                        entity_count,
                        if self.sidebar_visible { "Visible" } else { "Hidden" }
                    );
                }
            }
        }
    }
}

impl eframe::App for StandaloneSimpleDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let available_size = ctx.screen_rect().size();
        
        // Update layout
        self.update_layout(available_size);
        
        // Run layout and rendering
        run_simple_layout(&mut self.world, ctx);

        // Control panel
        egui::Window::new("Controls")
            .default_pos(egui::pos2(600.0, 100.0))
            .show(ctx, |ui| {
                ui.heading("Simple Demo");
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label("Header Text:");
                    ui.text_edit_singleline(&mut self.header_text);
                });

                ui.checkbox(&mut self.sidebar_visible, "Show Sidebar");

                ui.separator();
                
                ui.label("Debug Info:");
                ui.label(format!("Window Size: {:?}", available_size));
                
                let entity_count = self.world.entities().len();
                ui.label(format!("Total Entities: {}", entity_count));

                let widget_count = self.world.query::<&SimpleWidget>().iter(&self.world).count();
                ui.label(format!("Widgets: {}", widget_count));

                let bounds_count = self.world.query::<&LayoutBounds>().iter(&self.world).count();
                ui.label(format!("Layout Bounds: {}", bounds_count));
                
                ui.label(format!("Sidebar Visible: {}", self.sidebar_visible));
            });

        ctx.request_repaint();
    }
}