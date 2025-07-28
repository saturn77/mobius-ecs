use bevy_ecs::prelude::*;
use eframe::egui;
use mobius_ecs::morphorm_ecs::*;
use morphorm::{LayoutType, Units};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("Morphorm + ECS Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "morphorm_ecs_demo",
        options,
        Box::new(|_cc| Ok(Box::new(MorphormEcsDemo::new()))),
    )
}

struct MorphormEcsDemo {
    world: World,
    header_text: String,
    sidebar_visible: bool,
}

impl MorphormEcsDemo {
    fn new() -> Self {
        let mut world = World::new();
        
        // Initialize resources
        world.insert_resource(LayoutBridge::default());
        
        // Create the UI structure
        Self::setup_ui(&mut world);
        
        Self {
            world,
            header_text: "Morphorm + ECS Layout Demo".to_string(),
            sidebar_visible: true,
        }
    }

    fn setup_ui(world: &mut World) {
        // Root entity with available size (will be updated each frame)
        let root = world.spawn((
            LayoutRoot {
                available_size: egui::vec2(900.0, 700.0),
            },
            LayoutComponent::column(),
        )).id();

        // Header panel - fixed height
        let mut header_bundle = PanelBundle::new(egui::Color32::from_rgb(60, 60, 60))
            .with_stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 100)));
        header_bundle.layout = LayoutComponent {
            layout_type: LayoutType::Row,
            width: Units::Percentage(100.0),
            height: Units::Pixels(60.0),
        };
        let header = world.spawn((
            header_bundle,
            PaddingComponent::all(10.0),
        )).id();

        // Header text
        let header_text = world.spawn((
            LayoutComponent {
                layout_type: LayoutType::Column,
                width: Units::Auto,
                height: Units::Auto,
            },
            ComputedBounds::default(),
            WidgetType::Text {
                content: "Header Text".to_string(),
                size: Some(18.0),
                color: Some(egui::Color32::WHITE),
            },
        )).id();

        // Main container - vertical layout (fills remaining space between header in content and status bar)
        let mut main_container_bundle = LayoutBundle::default();
        main_container_bundle.layout = LayoutComponent {
            layout_type: LayoutType::Row, // Horizontal: sidebar + content
            width: Units::Percentage(100.0),
            height: Units::Stretch(1.0),
        };
        let main_container = world.spawn(main_container_bundle).id();

        // Sidebar panel
        let mut sidebar_bundle = PanelBundle::new(egui::Color32::from_rgb(40, 40, 40));
        sidebar_bundle.layout = LayoutComponent {
            layout_type: LayoutType::Column,
            width: Units::Pixels(200.0),
            height: Units::Percentage(100.0),
        };
        let sidebar = world.spawn((
            sidebar_bundle,
            PaddingComponent::all(15.0),
        )).id();

        // Sidebar items
        let sidebar_items = vec![
            "Dashboard",
            "Components",
            "Layout",
            "Settings",
            "About",
        ];

        for item in sidebar_items {
            let mut sidebar_item_bundle = TextBundle::new(item);
            sidebar_item_bundle.layout = LayoutComponent {
                layout_type: LayoutType::Column,
                width: Units::Percentage(100.0),
                height: Units::Pixels(30.0),
            };
            let sidebar_item = world.spawn((
                sidebar_item_bundle,
                MarginComponent {
                    bottom: Units::Pixels(5.0),
                    ..Default::default()
                },
            )).id();
            
            world.entity_mut(sidebar).add_child(sidebar_item);
        }

        // Content area - takes remaining horizontal space, vertical layout for header+content
        let mut content_bundle = PanelBundle::new(egui::Color32::from_rgb(30, 30, 30));
        content_bundle.layout = LayoutComponent {
            layout_type: LayoutType::Column, // Vertical: header + content_header + content_body
            width: Units::Stretch(1.0), // Take remaining space after sidebar
            height: Units::Percentage(100.0),
        };
        let content = world.spawn((
            content_bundle,
            PaddingComponent::all(20.0),
        )).id();

        // Content panels
        let mut content_header_bundle = TextBundle::new("Content Area");
        content_header_bundle.layout = LayoutComponent {
            layout_type: LayoutType::Column,
            width: Units::Percentage(100.0),
            height: Units::Pixels(40.0),
        };
        let content_header = world.spawn(content_header_bundle).id();

        let mut content_body_bundle = PanelBundle::new(egui::Color32::from_rgb(50, 50, 50));
        content_body_bundle.layout = LayoutComponent {
            layout_type: LayoutType::Column,
            width: Units::Percentage(100.0),
            height: Units::Stretch(1.0),
        };
        let content_body = world.spawn((
            content_body_bundle,
            PaddingComponent::all(10.0),
        )).id();

        // Status bar
        let mut status_bar_bundle = PanelBundle::new(egui::Color32::from_rgb(20, 20, 20));
        status_bar_bundle.layout = LayoutComponent {
            layout_type: LayoutType::Row,
            width: Units::Percentage(100.0),
            height: Units::Pixels(30.0),
        };
        let status_bar = world.spawn((
            status_bar_bundle,
            PaddingComponent::symmetric(10.0, 5.0),
        )).id();

        let status_text = world.spawn((
            LayoutComponent {
                layout_type: LayoutType::Column,
                width: Units::Auto,
                height: Units::Auto,
            },
            ComputedBounds::default(),
            WidgetType::Text {
                content: "Ready".to_string(),
                size: Some(12.0),
                color: Some(egui::Color32::LIGHT_GRAY),
            },
        )).id();

        // Build hierarchy with proper layout flow
        // Root contains: main_container (which includes header + body) and status_bar
        world.entity_mut(root).add_child(main_container);
        world.entity_mut(root).add_child(status_bar);
        
        // Main container has sidebar and content side by side
        world.entity_mut(main_container).add_child(sidebar);
        world.entity_mut(main_container).add_child(content);
        
        // Content area contains header and body vertically
        world.entity_mut(content).add_child(header);
        world.entity_mut(content).add_child(content_header);
        world.entity_mut(content).add_child(content_body);
        
        // Add text to their containers
        world.entity_mut(header).add_child(header_text);
        world.entity_mut(status_bar).add_child(status_text);
    }

    fn update_layout(&mut self, available_size: egui::Vec2) {
        // Update root size
        let mut root_query = self.world.query_filtered::<&mut LayoutRoot, With<LayoutRoot>>();
        for mut root in root_query.iter_mut(&mut self.world) {
            root.available_size = available_size;
        }

        // Update header text
        let mut text_query = self.world.query::<&mut WidgetType>();
        for mut widget in text_query.iter_mut(&mut self.world) {
            if let WidgetType::Text { content, size, .. } = &mut *widget {
                if size == &Some(18.0) { // This is our header text
                    *content = self.header_text.clone();
                    break;
                }
            }
        }

        // Toggle sidebar visibility by changing width
        let mut sidebar_query = self.world.query::<(&mut LayoutComponent, &WidgetType)>();
        for (mut layout, widget) in sidebar_query.iter_mut(&mut self.world) {
            if let WidgetType::Panel { color, .. } = widget {
                if *color == egui::Color32::from_rgb(40, 40, 40) {
                    if self.sidebar_visible {
                        layout.width = Units::Pixels(200.0);
                    } else {
                        layout.width = Units::Pixels(0.0);
                    }
                    break;
                }
            }
        }
    }
}

impl eframe::App for MorphormEcsDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Get available size
        let available_rect = ctx.screen_rect();
        let available_size = available_rect.size();

        // Update layout with current size
        self.update_layout(available_size);

        // Run layout systems
        compute_layout_immediate(&mut self.world);

        // Render all widgets
        render_widgets_immediate(&mut self.world, ctx);

        // Control panel
        egui::Window::new("Controls")
            .default_pos(egui::pos2(650.0, 100.0))
            .show(ctx, |ui| {
                ui.heading("Layout Controls");
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.label("Header Text:");
                    ui.text_edit_singleline(&mut self.header_text);
                });

                ui.checkbox(&mut self.sidebar_visible, "Show Sidebar");

                ui.separator();
                
                ui.label("Layout Info:");
                ui.label(format!("Window Size: {:?}", available_size));
                
                // Show entity count
                let entity_count = self.world.query::<Entity>().iter(&self.world).count();
                ui.label(format!("Total Entities: {}", entity_count));

                // Show rendered widget count (entities with ComputedBounds)
                let rendered_count = self.world.query::<(&ComputedBounds, &WidgetType)>().iter(&self.world).count();
                ui.label(format!("Rendered Widgets: {}", rendered_count));

                // Show layout component count
                let layout_count = self.world.query::<&LayoutComponent>().iter(&self.world).count();
                ui.label(format!("Layout Components: {}", layout_count));
                
                // Show sidebar status
                ui.label(format!("Sidebar Visible: {}", self.sidebar_visible));
            });

        // Request repaint
        ctx.request_repaint();
    }
}