use bevy_ecs::prelude::*;
use eframe::egui;
use mobius_ecs::taffy_dock_ecs_v3::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Simple Dock ECS Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "simple_dock_demo",
        options,
        Box::new(|_cc| Ok(Box::new(SimpleDockDemo::new()))),
    )
}

struct SimpleDockDemo {
    world: World,
}

impl SimpleDockDemo {
    fn new() -> Self {
        let mut world = World::new();
        
        // Setup dock system
        setup_simple_dock_system(&mut world);
        
        // Get the main tab root
        let main_tab = world
            .query::<(Entity, &TabId)>()
            .iter(&world)
            .find(|(_, id)| id.0 == "Main")
            .map(|(e, _)| e)
            .unwrap();
        
        // Add a header
        let header = create_simple_label(&mut world, "Simple Dock Layout Demo".to_string());
        world.entity_mut(main_tab).add_child(header);
        
        // Create a button row container
        let button_row = create_simple_container(&mut world, FlexDirection::Row);
        
        for i in 1..=3 {
            let btn = create_simple_button(&mut world, format!("Button {}", i));
            world.entity_mut(button_row).add_child(btn);
        }
        
        world.entity_mut(main_tab).add_child(button_row);
        
        // Create content area
        let content_area = create_simple_container(&mut world, FlexDirection::Column);
        
        // Add some items
        for i in 1..=3 {
            let item = create_simple_label(&mut world, format!("Content item {}", i));
            world.entity_mut(content_area).add_child(item);
        }
        
        world.entity_mut(main_tab).add_child(content_area);
        
        // Add a settings tab
        if let Some(settings_tab) = add_simple_tab(&mut world, "Settings".to_string()) {
            let settings_header = create_simple_label(&mut world, "Settings Panel".to_string());
            world.entity_mut(settings_tab).add_child(settings_header);
            
            let checkbox1 = create_simple_checkbox(&mut world, "Enable feature A".to_string(), true);
            world.entity_mut(settings_tab).add_child(checkbox1);
            
            let checkbox2 = create_simple_checkbox(&mut world, "Enable feature B".to_string(), false);
            world.entity_mut(settings_tab).add_child(checkbox2);
        }
        
        Self { world }
    }
}

impl eframe::App for SimpleDockDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render the dock system
        render_simple_dock_system(ctx, &mut self.world);
        
        // Debug panel
        egui::Window::new("Debug Info")
            .default_pos(egui::pos2(800.0, 50.0))
            .show(ctx, |ui| {
                ui.heading("ECS Info");
                ui.label(format!("Total Entities: {}", self.world.entities().len()));
                
                let ui_elements = self.world.query::<&UiElement>().iter(&self.world).count();
                ui.label(format!("UI Elements: {}", ui_elements));
                
                let tabs = self.world.query::<&TabId>().iter(&self.world).count();
                ui.label(format!("Tabs: {}", tabs));
                
                // Check for button clicks
                let mut button_clicks = Vec::new();
                let mut query = self.world.query::<(&UiElement, Entity)>();
                for (element, entity) in query.iter(&self.world) {
                    if let UiElement::Button { text, clicked } = element {
                        if *clicked {
                            button_clicks.push((entity, text.clone()));
                        }
                    }
                }
                
                if !button_clicks.is_empty() {
                    ui.separator();
                    ui.label("Button Clicks:");
                    for (entity, text) in &button_clicks {
                        ui.label(format!("'{}' was clicked!", text));
                        // Reset the clicked state
                        if let Some(mut elem) = self.world.get_mut::<UiElement>(*entity) {
                            if let UiElement::Button { clicked, .. } = &mut *elem {
                                *clicked = false;
                            }
                        }
                    }
                }
            });
        
        ctx.request_repaint();
    }
}