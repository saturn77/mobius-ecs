use egui_mobius::signals::Signal;
use egui_mobius::slot::Slot;
use egui_mobius::types::Value;
use bevy_ecs::prelude::*;
use crate::integration::TabKind;

/// Events sent from UI thread to code generation thread
#[derive(Debug, Clone)]
pub enum CodeGenEvent {
    /// Request to regenerate code (triggered by UI changes)
    RegenerateCode { tab_kind: TabKind, mode: CodeGenMode },
    /// Request to clear cached code
    ClearCache,
    /// Shutdown signal
    Shutdown,
}

/// Events sent from code generation thread back to UI thread
#[derive(Debug, Clone)]
pub enum CodeGenResponse {
    /// Generated code ready for display
    CodeReady { 
        tab_kind: TabKind, 
        mode: CodeGenMode, 
        code: String,
        generation_time_ms: u64,
    },
    /// Cache cleared confirmation
    CacheCleared,
    /// Error during code generation
    Error { message: String },
}

/// Code generation modes
#[derive(Debug, Clone, PartialEq)]
pub enum CodeGenMode {
    FullApp,
    PanelFunction,
}

/// Shared state for code generation system
pub struct CodeGenState {
    /// Current generated code for each mode
    pub full_app_code: Value<String>,
    pub panel_code: Value<String>,
    
    /// Generation status
    pub is_generating: Value<bool>,
    pub last_generation_time_ms: Value<u64>,
    
    /// Cache management
    pub code_hash: Value<u64>,
    pub needs_update: Value<bool>,
    
    /// Communication channels
    pub signal_to_codegen: Signal<CodeGenEvent>,
    pub slot_for_responses: Slot<CodeGenResponse>,
    
    /// Indicates if the response handler has been started
    response_handler_started: bool,
}

impl CodeGenState {
    pub fn new(
        signal_to_codegen: Signal<CodeGenEvent>,
        mut slot_for_responses: Slot<CodeGenResponse>,
    ) -> Self {
        let full_app_code = Value::new(String::new());
        let panel_code = Value::new(String::new());
        let is_generating = Value::new(false);
        let last_generation_time_ms = Value::new(0);
        
        // Set up response handler in the background
        {
            let full_app_code_clone = full_app_code.clone();
            let panel_code_clone = panel_code.clone();
            let is_generating_clone = is_generating.clone();
            let last_generation_time_ms_clone = last_generation_time_ms.clone();
            
            slot_for_responses.start(move |response: CodeGenResponse| {
                match response {
                    CodeGenResponse::CodeReady { mode, code, generation_time_ms, .. } => {
                        match mode {
                            CodeGenMode::FullApp => {
                                *full_app_code_clone.lock().unwrap() = code;
                            }
                            CodeGenMode::PanelFunction => {
                                *panel_code_clone.lock().unwrap() = code;
                            }
                        }
                        *last_generation_time_ms_clone.lock().unwrap() = generation_time_ms;
                        *is_generating_clone.lock().unwrap() = false;
                    }
                    CodeGenResponse::Error { message } => {
                        eprintln!("Code generation error: {}", message);
                        *is_generating_clone.lock().unwrap() = false;
                    }
                    CodeGenResponse::CacheCleared => {
                        // Cache cleared - no specific action needed
                    }
                }
            });
        }
        
        Self {
            full_app_code,
            panel_code,
            is_generating,
            last_generation_time_ms,
            code_hash: Value::new(0),
            needs_update: Value::new(true),
            signal_to_codegen,
            slot_for_responses,
            response_handler_started: true,
        }
    }

    /// Request code generation for a specific mode
    pub fn request_code_generation(&self, tab_kind: TabKind, mode: CodeGenMode) {
        *self.is_generating.lock().unwrap() = true;
        if let Err(e) = self.signal_to_codegen.send(CodeGenEvent::RegenerateCode { tab_kind, mode }) {
            eprintln!("Failed to send code generation request: {}", e);
            *self.is_generating.lock().unwrap() = false;
        }
    }

    /// Get the current code for a specific mode
    pub fn get_code(&self, mode: &CodeGenMode) -> String {
        match mode {
            CodeGenMode::FullApp => self.full_app_code.lock().unwrap().clone(),
            CodeGenMode::PanelFunction => self.panel_code.lock().unwrap().clone(),
        }
    }

    /// Check if code generation is currently in progress
    pub fn is_generating(&self) -> bool {
        *self.is_generating.lock().unwrap()
    }

    /// Get the last generation time (returns None if 0)
    pub fn get_last_generation_time(&self) -> Option<u64> {
        let time = *self.last_generation_time_ms.lock().unwrap();
        if time > 0 { Some(time) } else { None }
    }
    
    /// Get the last generation time
    pub fn last_generation_time_ms(&self) -> u64 {
        *self.last_generation_time_ms.lock().unwrap()
    }
    
    /// Get generated code for a specific tab kind and mode
    pub fn get_generated_code(&self, _tab_kind: TabKind, mode: CodeGenMode) -> Option<String> {
        let code = self.get_code(&mode);
        if code.is_empty() {
            None
        } else {
            Some(code)
        }
    }
    
}