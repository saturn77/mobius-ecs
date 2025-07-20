use serde::{Deserialize, Serialize};

/// Units of measurement for the designer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LengthUnit {
    Metric,
    Imperial,
}

impl Default for LengthUnit {
    fn default() -> Self {
        LengthUnit::Metric
    }
}

/// Tools available in the designer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tool {
    Select,
    Draw,
    Measure,
    Place,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Select
    }
}