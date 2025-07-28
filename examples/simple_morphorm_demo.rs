use bevy_ecs::prelude::*;
use eframe::egui;
use morphorm::Units;

// Include simple morphorm module contents directly
mod simple_morphorm {
    pub use crate::mobius_ecs::simple_morphorm::*;
}
use simple_morphorm::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Simple Morphorm + ECS Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "simple_morphorm_demo",
        options,
        Box::new(|_cc| Ok(Box::new(SimpleMorphormDemo::new()))),
    )
}

struct SimpleMorphormDemo {
    world: World,
    header_text: String,
    sidebar_visible: bool,
    header_entity: Option<Entity>,
    sidebar_entity: Option<Entity>,
    content_entity: Option<Entity>,
    status_entity: Option<Entity>,
}

impl SimpleMorphormDemo {
    fn new() -> Self {
        let mut world = World::new();
        setup_simple_layout(&mut world);
        
        let mut demo = Self {
            world,
            header_text: "Welcome to Simple Morphorm Demo".to_string(),
            sidebar_visible: true,
            header_entity: None,
            sidebar_entity: None,
            content_entity: None,
            status_entity: None,
        };
        
        demo.setup_ui();
        demo
    }

    fn setup_ui(&mut self) {
        // Create root entity
        let root = self.world.spawn((
            LayoutRoot { size: egui::vec2(800.0, 600.0) },
        )).id();

        // Create header (fixed height)
        let header = self.world.spawn(TextBundle::new(
            "Header Text".to_string(),
            18.0,
            egui::Color32::WHITE,
            SimpleLayout::fixed(800.0, 60.0),
        )).id();
        self.header_entity = Some(header);

        // Create sidebar (fixed width, stretch height)
        let sidebar = self.world.spawn(PanelBundle::new(
            egui::Color32::from_rgb(60, 60, 60),
            SimpleLayout::fixed(200.0, 540.0), // 600 - 60 header
        )).id();
        self.sidebar_entity = Some(sidebar);

        // Create content area (stretch to fill)
        let content = self.world.spawn(PanelBundle::new(
            egui::Color32::from_rgb(40, 40, 40),
            SimpleLayout::stretch_horizontal(),
        )).id();
        self.content_entity = Some(content);

        // Create status bar (fixed height)
        let status = self.world.spawn(TextBundle::new(
            "Ready".to_string(),
            12.0,
            egui::Color32::LIGHT_GRAY,
            SimpleLayout::fixed(800.0, 30.0),
        )).id();
        self.status_entity = Some(status);

        // Set up hierarchy
        self.world.entity_mut(root).add_child(header);
        if self.sidebar_visible {
            self.world.entity_mut(root).add_child(sidebar);
        }
        self.world.entity_mut(root).add_child(content);
        self.world.entity_mut(root).add_child(status);
    }

    fn update_layout(&mut self, available_size: egui::Vec2) {
        // Update root size
        let mut root_query = self.world.query_filtered::<&mut LayoutRoot, With<LayoutRoot>>();
        for mut root in root_query.iter_mut(&mut self.world) {
            root.size = available_size;
        }

        // Update header text
        if let Some(header) = self.header_entity {
            if let Ok(mut widget) = self.world.get_mut::<SimpleWidget>(header) {
                if let SimpleWidget::Text { content, .. } = &mut *widget {
                    *content = self.header_text.clone();
                }
            }
        }

        // Toggle sidebar visibility
        if let Some(sidebar) = self.sidebar_entity {
            if self.sidebar_visible {
                // Make sure sidebar is rendered
                if !self.world.entity(sidebar).contains::<LayoutBounds>() {
                    self.world.entity_mut(sidebar).insert(LayoutBounds::default());
                }
                // Update layout to fixed size
                if let Ok(mut layout) = self.world.get_mut::<SimpleLayout>(sidebar) {
                    layout.width = Units::Pixels(200.0);
                }
            } else {
                // Hide sidebar by setting width to 0
                if let Ok(mut layout) = self.world.get_mut::<SimpleLayout>(sidebar) {
                    layout.width = Units::Pixels(0.0);
                }
            }
        }

        // Update content area width based on sidebar
        if let Some(content) = self.content_entity {
            if let Ok(mut layout) = self.world.get_mut::<SimpleLayout>(content) {
                if self.sidebar_visible {
                    layout.width = Units::Pixels(available_size.x - 200.0);
                } else {
                    layout.width = Units::Pixels(available_size.x);
                }
            }
        }
    }
}

impl eframe::App for SimpleMorphormDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let available_size = ctx.screen_rect().size();
        
        // Update layout
        self.update_layout(available_size);
        
        // Run layout and rendering
        run_simple_layout(&mut self.world, ctx);

        // Control panel
        egui::Window::new("Simple Controls")
            .default_pos(egui::pos2(600.0, 100.0))
            .show(ctx, |ui| {
                ui.heading("Simple Morphorm Demo");
                
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