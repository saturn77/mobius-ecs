use bevy_ecs::prelude::*;
use eframe::egui;
use mobius_ecs::taffy_dock_ecs_v2::*;
use egui_taffy::taffy;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Taffy + Dock ECS V2 Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "taffy_dock_v2_demo",
        options,
        Box::new(|_cc| Ok(Box::new(TaffyDockV2Demo::new()))),
    )
}

struct TaffyDockV2Demo {
    world: World,
}

impl TaffyDockV2Demo {
    fn new() -> Self {
        let mut world = World::new();
        
        // Setup dock system
        setup_dock_system(&mut world);
        
        // Get the main tab root
        let main_tab = world
            .query::<(Entity, &TabId)>()
            .iter(&world)
            .find(|(_, id)| id.0 == "Main")
            .map(|(e, _)| e)
            .unwrap();
        
        // Add a header
        let header = create_label(&mut world, "Taffy Flex Layout Demo".to_string(), taffy::Style {
            margin: taffy::prelude::length(8.0),
            ..Default::default()
        });
        world.entity_mut(main_tab).add_child(header);
        
        // Create a button row
        let button_row = create_container(&mut world, taffy::Style {
            flex_direction: taffy::FlexDirection::Row,
            gap: taffy::Size {
                width: taffy::LengthPercentage::Length(8.0),
                height: taffy::LengthPercentage::Length(0.0),
            },
            ..Default::default()
        });
        
        for i in 1..=3 {
            let btn = create_button(&mut world, format!("Button {}", i), taffy::Style {
                flex_grow: 1.0,
                padding: taffy::prelude::length(8.0),
                ..Default::default()
            });
            world.entity_mut(button_row).add_child(btn);
        }
        
        world.entity_mut(main_tab).add_child(button_row);
        
        // Create content area
        let content_area = create_container(&mut world, taffy::Style {
            flex_grow: 1.0,
            flex_direction: taffy::FlexDirection::Column,
            padding: taffy::prelude::length(16.0),
            gap: taffy::prelude::length(8.0),
            border: taffy::prelude::length(1.0),
            ..Default::default()
        });
        
        // Add some flex items
        for i in 1..=3 {
            let item = create_label(&mut world, 
                format!("Flex item {} - This stretches horizontally", i), 
                taffy::Style {
                    align_self: Some(taffy::AlignSelf::Stretch),
                    padding: taffy::prelude::length(4.0),
                    ..Default::default()
                }
            );
            world.entity_mut(content_area).add_child(item);
        }
        
        world.entity_mut(main_tab).add_child(content_area);
        
        // Add a settings tab
        if let Some(settings_tab) = add_new_tab(&mut world, "Settings".to_string(), taffy::Style {
            flex_direction: taffy::FlexDirection::Column,
            gap: taffy::prelude::length(12.0),
            padding: taffy::prelude::length(16.0),
            ..Default::default()
        }) {
            let settings_header = create_label(&mut world, "Settings".to_string(), taffy::Style::default());
            world.entity_mut(settings_tab).add_child(settings_header);
            
            let checkbox1 = create_checkbox(&mut world, "Enable flex layout".to_string(), true, taffy::Style {
                margin: taffy::prelude::length(4.0),
                ..Default::default()
            });
            world.entity_mut(settings_tab).add_child(checkbox1);
            
            let checkbox2 = create_checkbox(&mut world, "Show debug info".to_string(), false, taffy::Style {
                margin: taffy::prelude::length(4.0),
                ..Default::default()
            });
            world.entity_mut(settings_tab).add_child(checkbox2);
        }
        
        Self { world }
    }
}

impl eframe::App for TaffyDockV2Demo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Enable multipass rendering
        ctx.options_mut(|options| {
            options.max_passes = std::num::NonZeroUsize::new(3).unwrap();
        });
        
        // Render the dock system
        render_dock_system(ctx, &mut self.world);
        
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