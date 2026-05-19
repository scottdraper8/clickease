use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Key {
    Char(char),
    Control,
    Shift,
    Alt,
    Meta, // Command on macOS, Windows key on Windows
    Escape,
    Enter,
    Backspace,
    Tab,
    Space,
    Up,
    Down,
    Left,
    Right,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum SimulatorError {
    #[error("Platform not supported: {0}")]
    UnsupportedPlatform(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("OS error: {0}")]
    OSError(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait InputSimulator {
    fn key_down(&self, key: Key) -> Result<(), SimulatorError>;
    fn key_up(&self, key: Key) -> Result<(), SimulatorError>;
    fn mouse_down(&self, button: MouseButton) -> Result<(), SimulatorError>;
    fn mouse_up(&self, button: MouseButton) -> Result<(), SimulatorError>;
    fn mouse_move(&self, x: i32, y: i32) -> Result<(), SimulatorError>;

    // Helper for a full click/tap
    fn key_click(&self, key: Key) -> Result<(), SimulatorError> {
        self.key_down(key.clone())?;
        // Small delay may be needed for OS registration; injection does not require it.
        self.key_up(key)
    }
}
