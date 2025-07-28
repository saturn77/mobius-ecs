use bevy_ecs::prelude::*;
use eframe::egui;
use mobius_ecs::simple_flex_dock::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Clean Flex Layout Demo - ECS"),
        ..Default::default()
    };

    eframe::run_native(
        "clean_flex_demo",
        options,
        Box::new(|_cc| Ok(Box::new(CleanFlexDemo::new()))),
    )
}

struct CleanFlexDemo {
    world: World,
}

impl CleanFlexDemo {
    fn new() -> Self {
        let mut world = World::new();
        
        // Setup dock system
        setup_simple_flex_dock(&mut world);
        
        // Get the main tab root
        let main_tab = world
            .query::<(Entity, &TabId)>()
            .iter(&world)
            .find(|(_, id)| id.0 == "Main")
            .map(|(e, _)| e)
            .unwrap();
        
        Self::setup_main_tab(&mut world, main_tab);
        Self::setup_layout_demo_tab(&mut world);
        Self::setup_controls_tab(&mut world);
        
        Self { world }
    }
    
    fn setup_main_tab(world: &mut World, main_tab: Entity) {
        // Header
        let header = create_flex_label_simple(
            world,
            "🎨 Clean Flex Layout Showcase".to_string(),
            Some(egui::Color32::from_rgb(100, 150, 255)),
            Some(20.0)
        );
        world.entity_mut(main_tab).add_child(header);
        
        // Main horizontal layout (sidebar + content)
        let main_layout = create_flex_container_simple(
            world,
            FlexDirection::Row,
            FlexJustify::Start,
            FlexAlign::Stretch,
            None,
            true,
            Some(egui::Color32::from_rgba_premultiplied(40, 40, 40, 50))
        );
        
        // Sidebar - fixed width
        let sidebar = create_flex_container_simple(
            world,
            FlexDirection::Column,
            FlexJustify::Start,
            FlexAlign::Start,
            Some("Navigation".to_string()),
            true,
            Some(egui::Color32::from_rgba_premultiplied(60, 60, 80, 100))
        );
        
        for i in 1..=4 {
            let nav_btn = create_flex_button_simple(
                world,
                format!("Nav Item {}", i),
                Some(egui::Color32::from_rgb(80, 120, 160))
            );
            world.entity_mut(sidebar).add_child(nav_btn);
        }
        
        // Spacer to push footer button to bottom
        let spacer = create_flex_spacer_simple(world, egui::Vec2::new(10.0, 20.0));
        world.entity_mut(sidebar).add_child(spacer);
        
        let footer_btn = create_flex_button_simple(
            world,
            "Settings".to_string(),
            Some(egui::Color32::from_rgb(100, 100, 120))
        );
        world.entity_mut(sidebar).add_child(footer_btn);
        
        // Content area - takes remaining space
        let content_area = create_flex_container_simple(
            world,
            FlexDirection::Column,
            FlexJustify::Start,
            FlexAlign::Start,
            Some("Main Content Area".to_string()),
            true,
            Some(egui::Color32::from_rgba_premultiplied(50, 60, 50, 100))
        );
        
        let content_text = create_flex_label_simple(
            world,
            "This demonstrates responsive flex layouts!\n\n• Left sidebar has fixed content\n• This area adapts to available space\n• Resize window to see flex behavior".to_string(),
            Some(egui::Color32::WHITE),
            Some(14.0)
        );
        world.entity_mut(content_area).add_child(content_text);
        
        // Button row demonstrating horizontal flex
        let button_row = create_flex_container_simple(
            world,
            FlexDirection::Row,
            FlexJustify::SpaceBetween,
            FlexAlign::Center,
            Some("Equal Width Buttons".to_string()),
            true,
            None
        );
        
        for i in 1..=3 {
            let btn = create_flex_button_simple(
                world,
                format!("Action {}", i),
                Some(egui::Color32::from_rgb(60, 150, 60))
            );
            world.entity_mut(button_row).add_child(btn);
        }
        
        world.entity_mut(content_area).add_child(button_row);
        
        world.entity_mut(main_layout).add_child(sidebar);
        world.entity_mut(main_layout).add_child(content_area);
        world.entity_mut(main_tab).add_child(main_layout);
        
        // Footer status bar
        let footer = create_flex_label_simple(
            world,
            "Status: All layouts working • Responsive design active".to_string(),
            Some(egui::Color32::GRAY),
            Some(12.0)
        );
        world.entity_mut(main_tab).add_child(footer);
    }
    
    fn setup_layout_demo_tab(world: &mut World) {
        if let Some(tab) = add_simple_flex_tab(world, "Layout Demo".to_string(), FlexDirection::Column) {
            let title = create_flex_label_simple(
                world,
                "Layout Direction Examples".to_string(),
                Some(egui::Color32::YELLOW),
                Some(18.0)
            );
            world.entity_mut(tab).add_child(title);
            
            // Row layout example
            let row_demo = create_flex_container_simple(
                world,
                FlexDirection::Row,
                FlexJustify::Start,
                FlexAlign::Center,
                Some("Horizontal Row Layout".to_string()),
                true,
                Some(egui::Color32::from_rgba_premultiplied(100, 50, 50, 80))
            );
            
            for i in 1..=4 {
                let item = create_flex_label_simple(
                    world,
                    format!("Item {}", i),
                    Some(egui::Color32::WHITE),
                    Some(14.0)
                );
                world.entity_mut(row_demo).add_child(item);
            }
            world.entity_mut(tab).add_child(row_demo);
            
            // Center justified row
            let center_demo = create_flex_container_simple(
                world,
                FlexDirection::Row,
                FlexJustify::Center,
                FlexAlign::Center,
                Some("Centered Layout".to_string()),
                true,
                Some(egui::Color32::from_rgba_premultiplied(50, 100, 50, 80))
            );
            
            for i in 1..=3 {
                let btn = create_flex_button_simple(
                    world,
                    format!("Centered {}", i),
                    Some(egui::Color32::from_rgb(80, 150, 80))
                );
                world.entity_mut(center_demo).add_child(btn);
            }
            world.entity_mut(tab).add_child(center_demo);
            
            // Space between layout
            let space_demo = create_flex_container_simple(
                world,
                FlexDirection::Row,
                FlexJustify::SpaceBetween,
                FlexAlign::Center,
                Some("Space Between Layout".to_string()),
                true,
                Some(egui::Color32::from_rgba_premultiplied(50, 50, 100, 80))
            );
            
            let left_btn = create_flex_button_simple(world, "Left".to_string(), None);
            let center_btn = create_flex_button_simple(world, "Center".to_string(), None);
            let right_btn = create_flex_button_simple(world, "Right".to_string(), None);
            
            world.entity_mut(space_demo).add_child(left_btn);
            world.entity_mut(space_demo).add_child(center_btn);
            world.entity_mut(space_demo).add_child(right_btn);
            world.entity_mut(tab).add_child(space_demo);
        }
    }
    
    fn setup_controls_tab(world: &mut World) {
        if let Some(tab) = add_simple_flex_tab(world, "Controls".to_string(), FlexDirection::Column) {
            let title = create_flex_label_simple(
                world,
                "Interactive Controls".to_string(),
                Some(egui::Color32::LIGHT_BLUE),
                Some(18.0)
            );
            world.entity_mut(tab).add_child(title);
            
            // Settings panel
            let settings_panel = create_flex_container_simple(
                world,
                FlexDirection::Column,
                FlexJustify::Start,
                FlexAlign::Start,
                Some("Settings Panel".to_string()),
                true,
                Some(egui::Color32::from_rgba_premultiplied(60, 60, 80, 100))
            );
            
            let volume_slider = create_flex_slider_simple(world, 0.7, 0.0..=1.0, "Volume".to_string());
            let brightness_slider = create_flex_slider_simple(world, 0.5, 0.0..=1.0, "Brightness".to_string());
            let contrast_slider = create_flex_slider_simple(world, 0.8, 0.0..=1.0, "Contrast".to_string());
            
            world.entity_mut(settings_panel).add_child(volume_slider);
            world.entity_mut(settings_panel).add_child(brightness_slider);
            world.entity_mut(settings_panel).add_child(contrast_slider);
            
            world.entity_mut(tab).add_child(settings_panel);
            
            // Action buttons
            let actions_panel = create_flex_container_simple(
                world,
                FlexDirection::Row,
                FlexJustify::SpaceAround,
                FlexAlign::Center,
                Some("Actions".to_string()),
                true,
                None
            );
            
            let save_btn = create_flex_button_simple(world, "Save Settings".to_string(), Some(egui::Color32::from_rgb(60, 150, 60)));
            let reset_btn = create_flex_button_simple(world, "Reset to Default".to_string(), Some(egui::Color32::from_rgb(150, 150, 60)));
            let cancel_btn = create_flex_button_simple(world, "Cancel".to_string(), Some(egui::Color32::from_rgb(150, 60, 60)));
            
            world.entity_mut(actions_panel).add_child(save_btn);
            world.entity_mut(actions_panel).add_child(reset_btn);
            world.entity_mut(actions_panel).add_child(cancel_btn);
            
            world.entity_mut(tab).add_child(actions_panel);
            
            // Status area
            let status_area = create_flex_container_simple(
                world,
                FlexDirection::Column,
                FlexJustify::Start,
                FlexAlign::Start,
                Some("Status & Information".to_string()),
                true,
                Some(egui::Color32::from_rgba_premultiplied(40, 60, 40, 100))
            );
            
            let status_text = create_flex_label_simple(
                world,
                "✅ All systems operational\n📊 Layout engine: Working\n🎨 Flex layouts: Active\n🔧 Controls: Responsive".to_string(),
                Some(egui::Color32::LIGHT_GREEN),
                Some(14.0)
            );
            world.entity_mut(status_area).add_child(status_text);
            world.entity_mut(tab).add_child(status_area);
        }
    }
}

impl eframe::App for CleanFlexDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render the dock system
        render_simple_flex_dock(ctx, &mut self.world);
        
        // Compact debug panel
        egui::Window::new("📊 Debug")
            .default_pos(egui::pos2(10.0, 10.0))
            .default_size(egui::Vec2::new(200.0, 200.0))
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(format!("Entities: {}", self.world.entities().len()));
                
                let elements = self.world.query::<&SimpleFlexElement>().iter(&self.world).count();
                ui.label(format!("UI Elements: {}", elements));
                
                let tabs = self.world.query::<&TabId>().iter(&self.world).count();
                ui.label(format!("Tabs: {}", tabs));
                
                ui.separator();
                ui.label("Interactions:");
                
                // Check for button clicks
                let mut button_clicks = Vec::new();
                let mut query = self.world.query::<(&SimpleFlexElement, Entity)>();
                for (element, entity) in query.iter(&self.world) {
                    if let SimpleFlexElement::Button { text, clicked, .. } = element {
                        if *clicked {
                            button_clicks.push((entity, text.clone()));
                        }
                    }
                }
                
                if !button_clicks.is_empty() {
                    for (entity, text) in &button_clicks {
                        ui.small(format!("• {}", text));
                        // Reset clicked state
                        if let Some(mut elem) = self.world.get_mut::<SimpleFlexElement>(*entity) {
                            if let SimpleFlexElement::Button { clicked, .. } = &mut *elem {
                                *clicked = false;
                            }
                        }
                    }
                } else {
                    ui.small("(no recent clicks)");
                }
                
                ui.separator();
                ui.small("💡 Resize window to see\nflex responsiveness!");
            });
        
        ctx.request_repaint();
    }
}