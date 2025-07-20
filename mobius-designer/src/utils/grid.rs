use egui::Pos2;

/// Snap a position to the nearest grid point
pub fn snap_to_grid(pos: Pos2, grid_spacing: f32) -> Pos2 {
    if grid_spacing <= 0.0 {
        return pos;
    }
    
    Pos2::new(
        (pos.x / grid_spacing).round() * grid_spacing,
        (pos.y / grid_spacing).round() * grid_spacing,
    )
}