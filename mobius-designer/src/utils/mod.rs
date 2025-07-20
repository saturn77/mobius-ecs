pub mod grid;

use bevy_ecs::prelude::*;
use crate::components::*;

pub use grid::snap_to_grid;

pub fn get_grid_status(zoom: f32, spacing: f32) -> GridStatus {
    let screen_spacing = spacing * zoom;
    
    if screen_spacing < 8.0 {
        GridStatus::TooFine
    } else if screen_spacing > 100.0 {
        GridStatus::TooCoarse
    } else {
        GridStatus::Visible(screen_spacing)
    }
}

pub fn clear_all_selections(world: &mut World) {
    let mut query = world.query::<&mut UiElementSelected>();
    for mut selected in query.iter_mut(world) {
        selected.selected = false;
    }
}

pub fn reset_button_clicks(world: &mut World) {
    let mut query = world.query::<&mut UiButton>();
    for mut button in query.iter_mut(world) {
        if button.clicked {
            if let Some(click_time) = button.click_time {
                if click_time.elapsed().as_millis() > 100 {
                    button.clicked = false;
                    button.click_time = None;
                }
            }
        }
    }
}

pub fn add_designer_log(
    world: &mut World,
    message: &str,
) {
    let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
    let log_entry = mobius_ecs::LogEntry {
        timestamp,
        level: mobius_ecs::LogLevel::Info,
        message: message.to_string(),
    };

    let mut query = world.query::<&mut mobius_ecs::EventLoggerPanel>();
    for mut logger in query.iter_mut(world) {
        // Insert at front for newest-first display
        logger.entries.insert(0, log_entry.clone());
        
        // Keep within max entries limit
        let max_entries = logger.max_entries;
        if logger.entries.len() > max_entries {
            logger.entries.truncate(max_entries);
        }
    }
}

pub fn clear_all_ui_elements(world: &mut World) {
    // Collect entities to despawn
    let mut entities_to_remove = Vec::new();
    
    // Query buttons
    let mut button_query = world.query::<(Entity, &UiButton)>();
    for (entity, _) in button_query.iter(world) {
        entities_to_remove.push(entity);
    }
    
    // Query text inputs
    let mut input_query = world.query::<(Entity, &UiTextInput)>();
    for (entity, _) in input_query.iter(world) {
        entities_to_remove.push(entity);
    }
    
    // Query checkboxes
    let mut checkbox_query = world.query::<(Entity, &UiCheckbox)>();
    for (entity, _) in checkbox_query.iter(world) {
        entities_to_remove.push(entity);
    }
    
    // Query radio buttons
    let mut radio_query = world.query::<(Entity, &UiRadioButton)>();
    for (entity, _) in radio_query.iter(world) {
        entities_to_remove.push(entity);
    }
    
    // Query group boxes
    let mut group_query = world.query::<(Entity, &UiGroupBox)>();
    for (entity, _) in group_query.iter(world) {
        entities_to_remove.push(entity);
    }
    
    // Despawn all collected entities
    for entity in entities_to_remove {
        world.despawn(entity);
    }
    
    add_designer_log(world, "Cleared all UI elements");
}