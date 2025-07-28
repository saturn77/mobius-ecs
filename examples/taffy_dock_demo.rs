use bevy_ecs::prelude::*;
use eframe::egui;
use mobius_ecs::taffy_dock_ecs::*;
use egui_taffy::taffy;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Taffy + Dock ECS Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "taffy_dock_demo",
        options,
        Box::new(|_cc| Ok(Box::new(TaffyDockDemo::new()))),
    )
}

struct TaffyDockDemo {
    world: World,
}

impl TaffyDockDemo {
    fn new() -> Self {
        let mut world = World::new();
        
        // Setup dock system
        setup_dock_system(&mut world);
        
        // Get the main tab entity
        let main_tab_entity = world
            .query_filtered::<Entity, With<TabId>>()
            .iter(&world)
            .next()
            .unwrap();
        
        // Create a flex container for the main tab
        let container = world.spawn((
            FlexContainer {
                direction: taffy::FlexDirection::Column,
                gap: taffy::Size {
                    width: taffy::LengthPercentage::Length(16.0),
                    height: taffy::LengthPercentage::Length(16.0),
                },
                ..Default::default()
            },
            ParentTab(main_tab_entity),
        )).id();
        
        // Add header
        let header_container = world.spawn((
            FlexContainer {
                direction: taffy::FlexDirection::Row,
                justify_content: taffy::JustifyContent::SpaceBetween,
                align_items: Some(taffy::AlignItems::Center),
                ..Default::default()
            },
            ParentTab(main_tab_entity),
        )).id();
        
        create_flex_label(&mut world, main_tab_entity, "Taffy Flex Layout Demo".to_string());
        
        // Add a row of buttons
        let button_row = world.spawn((
            FlexContainer {
                direction: taffy::FlexDirection::Row,
                gap: taffy::Size {
                    width: taffy::LengthPercentage::Length(8.0),
                    height: taffy::LengthPercentage::Length(8.0),
                },
                ..Default::default()
            },
            ParentTab(main_tab_entity),
        )).id();
        
        for i in 1..=3 {
            let btn = world.spawn((
                UiElement::Button { text: format!("Button {}", i) },
                FlexItem {
                    flex_grow: 1.0,
                    ..Default::default()
                },
                ParentTab(main_tab_entity),
            )).id();
            world.entity_mut(button_row).add_child(btn);
        }
        
        // Add a flexible content area
        let content_area = world.spawn((
            UiElement::Panel { title: "Content Area".to_string() },
            FlexItem {
                flex_grow: 1.0,
                padding: taffy::Rect {
                    left: taffy::LengthPercentage::Length(16.0),
                    right: taffy::LengthPercentage::Length(16.0),
                    top: taffy::LengthPercentage::Length(16.0),
                    bottom: taffy::LengthPercentage::Length(16.0),
                },
                ..Default::default()
            },
            FlexContainer {
                direction: taffy::FlexDirection::Column,
                gap: taffy::Size {
                    width: taffy::LengthPercentage::Length(8.0),
                    height: taffy::LengthPercentage::Length(8.0),
                },
                ..Default::default()
            },
            ParentTab(main_tab_entity),
        )).id();
        
        // Add some content to the panel
        for i in 1..=3 {
            let item = world.spawn((
                UiElement::Label { text: format!("Flex item {} - This will stretch horizontally", i) },
                FlexItem {
                    align_self: Some(taffy::AlignSelf::Stretch),
                    ..Default::default()
                },
                ParentTab(main_tab_entity),
            )).id();
            world.entity_mut(content_area).add_child(item);
        }
        
        // Add to main container
        world.entity_mut(container).add_child(header_container);
        world.entity_mut(container).add_child(button_row);
        world.entity_mut(container).add_child(content_area);
        
        // Add a new tab
        if let Some(mut dock_resource) = world.get_non_send_resource_mut::<DockResource>() {
            let settings_tab_entity = world.spawn(TabId("Settings".to_string())).id();
            
            dock_resource.tab_viewer_state.tabs.insert("Settings".to_string(), TabContent {
                entity: settings_tab_entity,
            });
            
            dock_resource.dock_state.push_to_focused_leaf("Settings".to_string());
        }
        
        // Setup settings tab content
        let settings_tab = world
            .query_filtered::<(Entity, &TabId)>()
            .iter(&world)
            .find(|(_, id)| id.0 == "Settings")
            .map(|(e, _)| e)
            .unwrap();
            
        let settings_container = world.spawn((
            FlexContainer {
                direction: taffy::FlexDirection::Column,
                gap: taffy::Size {
                    width: taffy::LengthPercentage::Length(12.0),
                    height: taffy::LengthPercentage::Length(12.0),
                },
                ..Default::default()
            },
            ParentTab(settings_tab),
        )).id();
        
        create_flex_label(&mut world, settings_tab, "Settings".to_string());
        
        world.spawn((
            UiElement::Checkbox { checked: true, text: "Enable flex layout".to_string() },
            FlexItem::default(),
            ParentTab(settings_tab),
        ));
        
        world.spawn((
            UiElement::Checkbox { checked: false, text: "Show debug info".to_string() },
            FlexItem::default(),
            ParentTab(settings_tab),
        ));
        
        Self { world }
    }
}

impl eframe::App for TaffyDockDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render the dock system
        render_dock_system(ctx, &mut self.world);
        
        // Debug panel
        egui::Window::new("Debug Info")
            .default_pos(egui::pos2(800.0, 50.0))
            .show(ctx, |ui| {
                ui.heading("ECS Info");
                ui.label(format!("Total Entities: {}", self.world.entities().len()));
                
                let flex_containers = self.world.query::<&FlexContainer>().iter(&self.world).count();
                ui.label(format!("Flex Containers: {}", flex_containers));
                
                let ui_elements = self.world.query::<&UiElement>().iter(&self.world).count();
                ui.label(format!("UI Elements: {}", ui_elements));
                
                let tabs = self.world.query::<&TabId>().iter(&self.world).count();
                ui.label(format!("Tabs: {}", tabs));
            });
        
        ctx.request_repaint();
    }
}