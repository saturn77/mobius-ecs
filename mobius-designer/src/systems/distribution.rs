use bevy_ecs::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::utils::*;

pub fn distribute_items_vertically(world: &mut World, settings: &DistributionSettings) {
    let mut selected_entities = Vec::new();
    
    // Collect selected entities with their positions
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            selected_entities.push((entity, pos.y));
        }
    }
    
    if selected_entities.len() < 2 {
        add_designer_log(
            world,
            mobius_ecs::LogLevel::Warning,
            "Need at least 2 selected elements to distribute".to_string(),
        );
        return;
    }
    
    // Sort by Y position
    selected_entities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Calculate distribution
    let start_y = selected_entities[0].1;
    let spacing = settings.vertical_spacing;
    
    // Apply new positions
    for (i, (entity, _)) in selected_entities.iter().enumerate() {
        let new_y = start_y + (i as f32 * spacing);
        
        if let Ok(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.y = new_y;
        }
    }
    
    add_designer_log(
        world,
        mobius_ecs::LogLevel::Info,
        format!("Distributed {} elements vertically", selected_entities.len()),
    );
}

pub fn distribute_items_horizontally(world: &mut World, settings: &DistributionSettings) {
    let mut selected_entities = Vec::new();
    
    // Collect selected entities with their positions
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            selected_entities.push((entity, pos.x));
        }
    }
    
    if selected_entities.len() < 2 {
        add_designer_log(
            world,
            mobius_ecs::LogLevel::Warning,
            "Need at least 2 selected elements to distribute".to_string(),
        );
        return;
    }
    
    // Sort by X position
    selected_entities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Calculate distribution
    let start_x = selected_entities[0].1;
    let spacing = settings.horizontal_spacing;
    
    // Apply new positions
    for (i, (entity, _)) in selected_entities.iter().enumerate() {
        let new_x = start_x + (i as f32 * spacing);
        
        if let Ok(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.x = new_x;
        }
    }
    
    add_designer_log(
        world,
        mobius_ecs::LogLevel::Info,
        format!("Distributed {} elements horizontally", selected_entities.len()),
    );
}

pub fn align_selected_elements_left(world: &mut World) {
    let mut left_most_x = f32::MAX;
    let mut selected_entities = Vec::new();
    
    // Find leftmost position and collect selected entities
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            left_most_x = left_most_x.min(pos.x);
            selected_entities.push(entity);
        }
    }
    
    if selected_entities.is_empty() {
        return;
    }
    
    // Align all selected elements to the leftmost position
    for entity in selected_entities.iter() {
        if let Ok(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.x = left_most_x;
        }
    }
    
    add_designer_log(
        world,
        mobius_ecs::LogLevel::Info,
        format!("Aligned {} elements to left", selected_entities.len()),
    );
}

pub fn align_selected_elements_right(world: &mut World) {
    let mut right_most_x = f32::MIN;
    let mut selected_entities = Vec::new();
    
    // Find rightmost position and collect selected entities
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    for (entity, pos, size, selected) in query.iter(world) {
        if selected.selected {
            right_most_x = right_most_x.max(pos.x + size.width);
            selected_entities.push((entity, size.width));
        }
    }
    
    if selected_entities.is_empty() {
        return;
    }
    
    // Align all selected elements to the rightmost position
    for (entity, width) in selected_entities.iter() {
        if let Ok(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.x = right_most_x - width;
        }
    }
    
    add_designer_log(
        world,
        mobius_ecs::LogLevel::Info,
        format!("Aligned {} elements to right", selected_entities.len()),
    );
}

pub fn align_selected_elements_top(world: &mut World) {
    let mut top_most_y = f32::MAX;
    let mut selected_entities = Vec::new();
    
    // Find topmost position and collect selected entities
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSelected)>();
    for (entity, pos, selected) in query.iter(world) {
        if selected.selected {
            top_most_y = top_most_y.min(pos.y);
            selected_entities.push(entity);
        }
    }
    
    if selected_entities.is_empty() {
        return;
    }
    
    // Align all selected elements to the topmost position
    for entity in selected_entities.iter() {
        if let Ok(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.y = top_most_y;
        }
    }
    
    add_designer_log(
        world,
        mobius_ecs::LogLevel::Info,
        format!("Aligned {} elements to top", selected_entities.len()),
    );
}

pub fn align_selected_elements_bottom(world: &mut World) {
    let mut bottom_most_y = f32::MIN;
    let mut selected_entities = Vec::new();
    
    // Find bottommost position and collect selected entities
    let mut query = world.query::<(Entity, &UiElementPosition, &UiElementSize, &UiElementSelected)>();
    for (entity, pos, size, selected) in query.iter(world) {
        if selected.selected {
            bottom_most_y = bottom_most_y.max(pos.y + size.height);
            selected_entities.push((entity, size.height));
        }
    }
    
    if selected_entities.is_empty() {
        return;
    }
    
    // Align all selected elements to the bottommost position
    for (entity, height) in selected_entities.iter() {
        if let Ok(mut pos) = world.get_mut::<UiElementPosition>(*entity) {
            pos.y = bottom_most_y - height;
        }
    }
    
    add_designer_log(
        world,
        mobius_ecs::LogLevel::Info,
        format!("Aligned {} elements to bottom", selected_entities.len()),
    );
}