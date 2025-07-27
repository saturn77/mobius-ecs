# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 27 July 2025

### Added
- **Mobius Designer** - Complete visual UI designer application for egui
  - Vertical alignment and distribution controls for precise layout
  - Horizontal alignment and distribution controls
  - Real-time code generation for ui_panels in Rust/egui
  - Integration with egui_mobius signals/slots for thread-safe communication
  - Comprehensive control panel with all design tools
  - Grid snapping system for precise component placement
  - Live preview with instant updates
  - Project export functionality to generate complete Cargo projects
  - Syntax highlighting for generated code using syntect
  - File dialog support for project management
  - Component property editing with live updates
  - Drag-and-drop interface building

### Changed
- Restructured project to emphasize mobius-designer as the main application
- Updated README.md to focus on visual designer capabilities
- Reorganized examples to highlight designer features

### Dependencies
- Updated `bevy_ecs` from 0.14 to 0.16.1
- Added `egui_mobius` (0.3.0-alpha.32) for signals/slots functionality
- Added `syntect` (5.1) for syntax highlighting in code generation
- Added `egui-file-dialog` (0.11.0) for file management
- Added `uuid` (1.0) for unique component identifiers
- Added `rand` (0.8) for random generation utilities

## [0.1.0] - 20 July 2025

### Added
- Initial ECS-based UI framework for egui applications
- Core framework with bevy_ecs integration
- Basic templating system
- Example applications (demo, windows, designer)
- Project generation functionality
- Component library including:
  - MobiusApp - Main application container
  - MainWorkArea - Primary workspace
  - SettingsPanel - Configuration panel
  - EventLoggerPanel - Event logging
  - ControlsPanel - Control buttons
  - GenericTab - Tab system
- Docking support via egui_dock
- Timezone and localization support

### Dependencies
- `egui` & `eframe` (0.32)
- `bevy_ecs` (0.14)
- `egui_dock` (0.17.0)
- `serde` (1.0)
- `chrono` (0.4) & `chrono-tz` (0.10.3)
- `image` (0.25.6)
- `egui_tool_windows` (0.1.3)