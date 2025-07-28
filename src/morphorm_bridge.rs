use egui::{Context, Rect, Vec2};
use morphorm::{LayoutType, Units};
use std::collections::HashMap;

/// A simple layout node for morphorm integration
#[derive(Default, Clone)]
pub struct LayoutNode {
    pub width: Units,
    pub height: Units,
    pub layout_type: LayoutType,
    pub padding_left: Units,
    pub padding_right: Units,
    pub padding_top: Units,
    pub padding_bottom: Units,
    pub children: Vec<LayoutNode>,
    // Computed values
    pub computed_x: f32,
    pub computed_y: f32,
    pub computed_width: f32,
    pub computed_height: f32,
}

impl LayoutNode {
    /// Create a new layout node
    pub fn new() -> Self {
        Self {
            layout_type: LayoutType::Row,
            ..Default::default()
        }
    }

    /// Set width
    pub fn with_width(mut self, width: Units) -> Self {
        self.width = width;
        self
    }

    /// Set height
    pub fn with_height(mut self, height: Units) -> Self {
        self.height = height;
        self
    }

    /// Set padding
    pub fn with_padding(mut self, padding: f32) -> Self {
        let units = Units::Pixels(padding);
        self.padding_left = units;
        self.padding_right = units;
        self.padding_top = units;
        self.padding_bottom = units;
        self
    }

    /// Add a child node
    pub fn add_child(&mut self, child: LayoutNode) {
        self.children.push(child);
    }

    /// Simple layout computation (for demo purposes)
    pub fn compute_layout(&mut self, available_width: f32, available_height: f32) {
        // Compute this node's size
        match self.width {
            Units::Pixels(px) => self.computed_width = px,
            Units::Percentage(pct) => self.computed_width = available_width * (pct / 100.0),
            Units::Stretch(_) => self.computed_width = available_width,
            _ => self.computed_width = available_width,
        }

        match self.height {
            Units::Pixels(px) => self.computed_height = px,
            Units::Percentage(pct) => self.computed_height = available_height * (pct / 100.0),
            Units::Stretch(_) => self.computed_height = available_height,
            _ => self.computed_height = available_height,
        }

        // Layout children based on layout type
        let mut y_offset = 0.0;
        let mut remaining_height = self.computed_height;

        // Calculate fixed height children first
        for child in &self.children {
            if let Units::Pixels(h) = child.height {
                remaining_height -= h;
            }
        }

        // Layout children
        for child in &mut self.children {
            child.computed_x = self.computed_x;
            child.computed_y = self.computed_y + y_offset;
            
            let child_height = match child.height {
                Units::Pixels(h) => h,
                Units::Stretch(_) => remaining_height.max(0.0),
                _ => self.computed_height,
            };

            child.compute_layout(self.computed_width, child_height);
            y_offset += child.computed_height;
        }
    }
}

/// A bridge between Morphorm layout concepts and egui
pub struct MorphormLayoutBridge {
    /// The root layout node
    root: LayoutNode,
    /// Maps node indices to their computed bounds
    node_bounds: HashMap<usize, Rect>,
    /// Counter for node IDs
    next_id: usize,
}

impl MorphormLayoutBridge {
    /// Create a new morphorm layout bridge
    pub fn new() -> Self {
        Self {
            root: LayoutNode::new(),
            node_bounds: HashMap::new(),
            next_id: 0,
        }
    }

    /// Get or create from egui memory
    pub fn get_or_create(ctx: &Context) -> Self {
        ctx.data_mut(|data| {
            data.get_temp_mut_or_default::<Self>(egui::Id::new("morphorm_layout_bridge"))
                .clone()
        })
    }

    /// Store back to egui memory
    pub fn store(&self, ctx: &Context) {
        ctx.data_mut(|data| {
            data.insert_temp(egui::Id::new("morphorm_layout_bridge"), self.clone());
        });
    }

    /// Set up the root container with the given size
    pub fn setup_root(&mut self, size: Vec2) {
        self.root = LayoutNode::new()
            .with_width(Units::Pixels(size.x))
            .with_height(Units::Pixels(size.y));
        self.root.layout_type = LayoutType::Column;
    }

    /// Add a header panel with fixed height
    pub fn add_header(&mut self, height: f32) -> usize {
        let header = LayoutNode::new()
            .with_width(Units::Percentage(100.0))
            .with_height(Units::Pixels(height))
            .with_padding(10.0);
        
        self.root.add_child(header);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Add a content panel that stretches to fill remaining space
    pub fn add_content(&mut self) -> usize {
        let content = LayoutNode::new()
            .with_width(Units::Percentage(100.0))
            .with_height(Units::Stretch(1.0))
            .with_padding(20.0);
        
        self.root.add_child(content);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Perform layout calculation
    pub fn compute_layout(&mut self, available_size: Vec2) -> Result<(), String> {
        // Compute the layout
        self.root.compute_layout(available_size.x, available_size.y);
        
        // Update node bounds
        self.update_node_bounds();
        
        Ok(())
    }

    /// Update cached node bounds after layout computation
    fn update_node_bounds(&mut self) {
        self.node_bounds.clear();
        
        // Store child bounds
        for (i, child) in self.root.children.iter().enumerate() {
            let rect = Rect::from_min_size(
                egui::pos2(child.computed_x, child.computed_y),
                egui::vec2(child.computed_width, child.computed_height),
            );
            self.node_bounds.insert(i, rect);
        }
    }

    /// Get the computed bounds for a node
    pub fn get_bounds(&self, node_id: usize) -> Option<Rect> {
        self.node_bounds.get(&node_id).copied()
    }

    /// Clear the layout tree
    pub fn clear(&mut self) {
        self.root = LayoutNode::new();
        self.node_bounds.clear();
        self.next_id = 0;
    }
}

impl Clone for MorphormLayoutBridge {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            node_bounds: self.node_bounds.clone(),
            next_id: self.next_id,
        }
    }
}

impl Default for MorphormLayoutBridge {
    fn default() -> Self {
        Self::new()
    }
}