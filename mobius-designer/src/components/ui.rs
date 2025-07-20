use bevy_ecs::prelude::*;

#[derive(Component, Clone)]
pub struct UiButton {
    pub label: String,
    pub clicked: bool,
    pub enabled: bool,
    pub click_time: Option<std::time::Instant>,
    pub font_size: f32,
}

#[derive(Component, Clone)]
pub struct UiTextInput {
    pub label: String,
    pub value: String,
    pub enabled: bool,
    pub font_size: f32,
}

#[derive(Component, Clone)]
pub struct UiCheckbox {
    pub label: String,
    pub checked: bool,
    pub enabled: bool,
    pub font_size: f32,
}

#[derive(Component, Clone)]
pub struct UiRadioButton {
    pub label: String,
    pub selected: bool,
    pub enabled: bool,
    pub font_size: f32,
    pub group_id: String,
}

#[derive(Component, Clone)]
pub struct UiGroupBox {
    pub label: String,
    pub enabled: bool,
    pub font_size: f32,
    pub contained_widgets: Vec<Entity>,
}