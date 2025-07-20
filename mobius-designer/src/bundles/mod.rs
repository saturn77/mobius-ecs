use bevy_ecs::prelude::*;
use crate::components::*;

#[derive(Bundle)]
pub struct UiButtonBundle {
    pub button: UiButton,
    pub position: UiElementPosition,
    pub size: UiElementSize,
    pub tab: UiElementTab,
    pub selected: UiElementSelected,
    pub container: UiElementContainer,
}

#[derive(Bundle)]
pub struct UiTextInputBundle {
    pub text_input: UiTextInput,
    pub position: UiElementPosition,
    pub size: UiElementSize,
    pub tab: UiElementTab,
    pub selected: UiElementSelected,
    pub container: UiElementContainer,
}

#[derive(Bundle)]
pub struct UiCheckboxBundle {
    pub checkbox: UiCheckbox,
    pub position: UiElementPosition,
    pub size: UiElementSize,
    pub tab: UiElementTab,
    pub selected: UiElementSelected,
    pub container: UiElementContainer,
}

#[derive(Bundle)]
pub struct UiRadioButtonBundle {
    pub radio_button: UiRadioButton,
    pub position: UiElementPosition,
    pub size: UiElementSize,
    pub tab: UiElementTab,
    pub selected: UiElementSelected,
    pub container: UiElementContainer,
}

#[derive(Bundle)]
pub struct UiGroupBoxBundle {
    pub group_box: UiGroupBox,
    pub position: UiElementPosition,
    pub size: UiElementSize,
    pub tab: UiElementTab,
    pub selected: UiElementSelected,
    pub container: UiElementContainer,
}