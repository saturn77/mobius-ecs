use bevy_ecs::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct GridSettings {
    pub enabled: bool,
    pub spacing_pixels: f32,
    pub dot_size: f32,
    pub snap_enabled: bool,
    pub show_grid: bool,
}

impl std::hash::Hash for GridSettings {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.enabled.hash(state);
        self.spacing_pixels.to_bits().hash(state);
        self.dot_size.to_bits().hash(state);
        self.snap_enabled.hash(state);
        self.show_grid.hash(state);
    }
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            spacing_pixels: 20.0,
            dot_size: 2.0,
            snap_enabled: false,
            show_grid: false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum GridStatus {
    TooFine,
    TooCoarse,
    Visible(f32),
}