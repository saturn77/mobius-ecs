use bevy_ecs::prelude::*;
use crate::resources::{LengthUnit, Tool};

/// Designer-specific settings panel with units
#[derive(Component)]
pub struct DesignerSettingsPanel {
    pub units: LengthUnit,
    pub timezone: String,
}

impl Default for DesignerSettingsPanel {
    fn default() -> Self {
        Self {
            units: LengthUnit::Metric,
            timezone: "UTC".to_string(),
        }
    }
}

/// Designer-specific controls panel with tool selection
#[derive(Component)]
pub struct DesignerControlsPanel {
    pub selected_tool: Tool,
}

impl Default for DesignerControlsPanel {
    fn default() -> Self {
        Self {
            selected_tool: Tool::Select,
        }
    }
}