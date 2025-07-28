use bevy_ecs::prelude::*;
use eframe::egui;
use mobius_ecs::taffy_dock_ecs_v4::*;
use egui_taffy::taffy;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_title("Flex Layout Showcase - ECS + egui-taffy"),
        ..Default::default()
    };

    eframe::run_native(
        "flex_showcase_demo",
        options,
        Box::new(|_cc| Ok(Box::new(FlexShowcaseDemo::new()))),
    )
}

struct FlexShowcaseDemo {
    world: World,
}

impl FlexShowcaseDemo {
    fn new() -> Self {
        let mut world = World::new();
        
        // Setup dock system
        setup_flex_dock_system(&mut world);
        
        // Get the main tab root
        let main_tab = world
            .query::<(Entity, &TabId)>()
            .iter(&world)
            .find(|(_, id)| id.0 == "Main")
            .map(|(e, _)| e)
            .unwrap();
        
        Self::setup_main_tab(&mut world, main_tab);
        Self::setup_flex_demo_tab(&mut world);
        Self::setup_responsive_tab(&mut world);
        Self::setup_controls_tab(&mut world);
        
        Self { world }
    }
    
    fn setup_main_tab(world: &mut World, main_tab: Entity) {
        // Header section - fixed height
        let header = create_flex_label(
            world, 
            "🚀 Flex Layout Showcase".to_string(), 
            0.0,
            Some(egui::Color32::from_rgb(100, 200, 255))
        );
        world.entity_mut(main_tab).add_child(header);
        
        // Main content area - grows to fill space
        let content_area = create_flex_container(
            world, 
            taffy::FlexDirection::Row, 
            1.0,
            None,
            true
        );
        
        // Left sidebar - fixed width with flex content
        let sidebar = create_flex_container(
            world, 
            taffy::FlexDirection::Column, 
            0.0,
            Some("Navigation".to_string()),
            true
        );
        
        // Set fixed width for sidebar
        if let Some(mut style) = world.get_mut::<FlexStyle>(sidebar) {
            style.min_size.width = taffy::Dimension::Length(200.0);
            style.max_size.width = taffy::Dimension::Length(200.0);
        }
        
        for i in 1..=4 {
            let nav_item = create_flex_button(
                world, 
                format!("Nav Item {}", i), 
                0.0,
                Some(egui::Vec2::new(180.0, 30.0))
            );
            world.entity_mut(sidebar).add_child(nav_item);
        }
        
        // Add flexible spacer
        let spacer = create_flex_spacer(world, 1.0, egui::Vec2::new(10.0, 10.0));
        world.entity_mut(sidebar).add_child(spacer);
        
        let footer_btn = create_flex_button(
            world, 
            "Settings".to_string(), 
            0.0,
            Some(egui::Vec2::new(180.0, 25.0))
        );
        world.entity_mut(sidebar).add_child(footer_btn);
        
        // Main content - grows to fill remaining space
        let main_content = create_flex_container(
            world, 
            taffy::FlexDirection::Column, 
            1.0,
            Some("Main Content".to_string()),
            true
        );
        
        let content_text = create_flex_label(
            world, 
            "This area demonstrates flex-grow: 1.0\nIt expands to fill available space!".to_string(), 
            0.0,
            None
        );
        world.entity_mut(main_content).add_child(content_text);
        
        // Button row that demonstrates equal distribution
        let button_row = create_flex_container(
            world, 
            taffy::FlexDirection::Row, 
            0.0,
            None,
            false
        );
        
        for i in 1..=3 {
            let btn = create_flex_button(
                world, 
                format!("Flex Button {}", i), 
                1.0,  // Equal flex-grow means equal distribution
                None
            );
            world.entity_mut(button_row).add_child(btn);
        }
        
        world.entity_mut(main_content).add_child(button_row);
        
        world.entity_mut(content_area).add_child(sidebar);
        world.entity_mut(content_area).add_child(main_content);
        world.entity_mut(main_tab).add_child(content_area);
        
        // Footer - fixed height
        let footer = create_flex_label(
            world, 
            "📊 Status: All flex layouts working • Resize window to see responsiveness".to_string(), 
            0.0,
            Some(egui::Color32::GRAY)
        );
        world.entity_mut(main_tab).add_child(footer);
    }
    
    fn setup_flex_demo_tab(world: &mut World) {
        if let Some(tab) = add_flex_tab(world, "Flex Demo".to_string(), taffy::FlexDirection::Column) {
            let title = create_flex_label(
                world, 
                "Flex Properties Demonstration".to_string(), 
                0.0,
                Some(egui::Color32::WHITE)
            );
            world.entity_mut(tab).add_child(title);
            
            // Demo flex-grow with different values
            let grow_demo = create_flex_container(
                world, 
                taffy::FlexDirection::Row, 
                0.0,
                Some("Flex-Grow Demo (1:2:3 ratio)".to_string()),
                true
            );
            
            let item1 = create_flex_label(world, "Grow: 1.0".to_string(), 1.0, Some(egui::Color32::from_rgb(255, 100, 100)));
            let item2 = create_flex_label(world, "Grow: 2.0".to_string(), 2.0, Some(egui::Color32::from_rgb(100, 255, 100)));
            let item3 = create_flex_label(world, "Grow: 3.0".to_string(), 3.0, Some(egui::Color32::from_rgb(100, 100, 255)));
            
            world.entity_mut(grow_demo).add_child(item1);
            world.entity_mut(grow_demo).add_child(item2);
            world.entity_mut(grow_demo).add_child(item3);
            world.entity_mut(tab).add_child(grow_demo);
            
            // Demo mixed content
            let mixed_demo = create_flex_container(
                world, 
                taffy::FlexDirection::Column, 
                1.0,
                Some("Mixed Content".to_string()),
                true
            );
            
            let slider = create_flex_slider(world, 0.5, 0.0..=1.0, "Flex Factor".to_string(), 0.0);
            world.entity_mut(mixed_demo).add_child(slider);
            
            // Horizontal layout with different alignments
            let align_demo = create_flex_container(
                world, 
                taffy::FlexDirection::Row, 
                0.0,
                None,
                false
            );
            
            let left_btn = create_flex_button(world, "Left".to_string(), 0.0, None);
            let center_spacer = create_flex_spacer(world, 1.0, egui::Vec2::new(10.0, 10.0));
            let right_btn = create_flex_button(world, "Right".to_string(), 0.0, None);
            
            world.entity_mut(align_demo).add_child(left_btn);
            world.entity_mut(align_demo).add_child(center_spacer);
            world.entity_mut(align_demo).add_child(right_btn);
            
            world.entity_mut(mixed_demo).add_child(align_demo);
            world.entity_mut(tab).add_child(mixed_demo);
        }
    }
    
    fn setup_responsive_tab(world: &mut World) {
        if let Some(tab) = add_flex_tab(world, "Responsive".to_string(), taffy::FlexDirection::Column) {
            let title = create_flex_label(
                world, 
                "Responsive Layout Demo".to_string(), 
                0.0,
                Some(egui::Color32::YELLOW)
            );
            world.entity_mut(tab).add_child(title);
            
            let description = create_flex_label(
                world, 
                "Resize the window to see how flex layouts adapt automatically!".to_string(), 
                0.0,
                None
            );
            world.entity_mut(tab).add_child(description);
            
            // Card layout that wraps responsively
            let card_container = create_flex_container(
                world, 
                taffy::FlexDirection::Row, 
                1.0,
                Some("Flexible Card Layout".to_string()),
                true
            );
            
            for i in 1..=4 {
                let card = create_flex_container(
                    world, 
                    taffy::FlexDirection::Column, 
                    1.0,  // Equal distribution
                    Some(format!("Card {}", i)),
                    true
                );
                
                // Set minimum width for cards 
                if let Some(mut style) = world.get_mut::<FlexStyle>(card) {
                    style.min_size.width = taffy::Dimension::Length(150.0);
                    style.padding = taffy::Rect::length(12.0);
                }
                
                let card_title = create_flex_label(
                    world, 
                    format!("Title {}", i), 
                    0.0,
                    Some(egui::Color32::WHITE)
                );
                
                let card_content = create_flex_label(
                    world, 
                    "This content grows to fill the card space.".to_string(), 
                    1.0,
                    None
                );
                
                let card_btn = create_flex_button(
                    world, 
                    "Action".to_string(), 
                    0.0,
                    None
                );
                
                world.entity_mut(card).add_child(card_title);
                world.entity_mut(card).add_child(card_content);
                world.entity_mut(card).add_child(card_btn);
                world.entity_mut(card_container).add_child(card);
            }
            
            world.entity_mut(tab).add_child(card_container);
        }
    }
    
    fn setup_controls_tab(world: &mut World) {
        if let Some(tab) = add_flex_tab(world, "Controls".to_string(), taffy::FlexDirection::Column) {
            let title = create_flex_label(
                world, 
                "Interactive Controls".to_string(), 
                0.0,
                Some(egui::Color32::LIGHT_BLUE)
            );
            world.entity_mut(tab).add_child(title);
            
            // Control panel
            let controls = create_flex_container(
                world, 
                taffy::FlexDirection::Column, 
                0.0,
                Some("Settings Panel".to_string()),
                true
            );
            
            let volume_slider = create_flex_slider(world, 0.7, 0.0..=1.0, "Volume".to_string(), 0.0);
            let brightness_slider = create_flex_slider(world, 0.8, 0.0..=1.0, "Brightness".to_string(), 0.0);
            let contrast_slider = create_flex_slider(world, 0.5, 0.0..=1.0, "Contrast".to_string(), 0.0);
            
            world.entity_mut(controls).add_child(volume_slider);
            world.entity_mut(controls).add_child(brightness_slider);
            world.entity_mut(controls).add_child(contrast_slider);
            
            // Action buttons in a row
            let action_row = create_flex_container(
                world, 
                taffy::FlexDirection::Row, 
                0.0,
                None,
                false
            );
            
            let reset_btn = create_flex_button(world, "Reset".to_string(), 1.0, None);
            let apply_btn = create_flex_button(world, "Apply".to_string(), 1.0, None);
            let cancel_btn = create_flex_button(world, "Cancel".to_string(), 1.0, None);
            
            world.entity_mut(action_row).add_child(reset_btn);
            world.entity_mut(action_row).add_child(apply_btn);
            world.entity_mut(action_row).add_child(cancel_btn);
            
            world.entity_mut(controls).add_child(action_row);
            world.entity_mut(tab).add_child(controls);
            
            // Status area that grows to fill remaining space
            let status_area = create_flex_container(
                world, 
                taffy::FlexDirection::Column, 
                1.0,
                Some("Status & Logs".to_string()),
                true
            );
            
            let status_text = create_flex_label(
                world, 
                "Ready • All systems operational\nLog messages would appear here...".to_string(), 
                1.0,
                Some(egui::Color32::LIGHT_GREEN)
            );
            world.entity_mut(status_area).add_child(status_text);
            world.entity_mut(tab).add_child(status_area);
        }
    }
}

impl eframe::App for FlexShowcaseDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Enable smooth animations
        ctx.options_mut(|options| {
            options.max_passes = std::num::NonZeroUsize::new(3).unwrap();
        });
        
        // Render the dock system
        render_flex_dock_system(ctx, &mut self.world);
        
        // Debug panel
        egui::Window::new("🔧 Debug Info")
            .default_pos(egui::pos2(10.0, 10.0))
            .default_size(egui::Vec2::new(250.0, 300.0))
            .show(ctx, |ui| {
                ui.heading("ECS Statistics");
                ui.separator();
                ui.label(format!("📊 Total Entities: {}", self.world.entities().len()));
                
                let flex_elements = self.world.query::<&FlexElement>().iter(&self.world).count();
                ui.label(format!("🎨 Flex Elements: {}", flex_elements));
                
                let flex_styles = self.world.query::<&FlexStyle>().iter(&self.world).count();
                ui.label(format!("📐 Flex Styles: {}", flex_styles));
                
                let tabs = self.world.query::<&TabId>().iter(&self.world).count();
                ui.label(format!("📑 Tabs: {}", tabs));
                
                ui.separator();
                ui.heading("Interactions");
                
                // Check for button clicks
                let mut button_clicks = Vec::new();
                let mut query = self.world.query::<(&FlexElement, Entity)>();
                for (element, entity) in query.iter(&self.world) {
                    if let FlexElement::Button { text, clicked, .. } = element {
                        if *clicked {
                            button_clicks.push((entity, text.clone()));
                        }
                    }
                }
                
                if !button_clicks.is_empty() {
                    ui.label("Recent Clicks:");
                    for (entity, text) in &button_clicks {
                        ui.label(format!("• '{}'", text));
                        // Reset the clicked state
                        if let Some(mut elem) = self.world.get_mut::<FlexElement>(*entity) {
                            if let FlexElement::Button { clicked, .. } = &mut *elem {
                                *clicked = false;
                            }
                        }
                    }
                } else {
                    ui.label("No recent clicks");
                }
                
                ui.separator();
                ui.small("💡 Tip: Resize window to see\nresponsive flex layouts!");
            });
        
        ctx.request_repaint();
    }
}