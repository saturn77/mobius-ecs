use bevy_ecs::prelude::*;

#[derive(Default, Resource, Clone)]
pub struct DistributionSettings {
    pub vertical_spacing: f32,
    pub horizontal_spacing: f32,
}

impl DistributionSettings {
    pub fn new() -> Self {
        Self {
            vertical_spacing: 50.0,
            horizontal_spacing: 100.0,
        }
    }
}