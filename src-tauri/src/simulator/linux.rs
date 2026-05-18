use crate::simulator::types::*;
use mouse_keyboard_input::VirtualDevice;
use std::sync::Mutex;

pub struct LinuxSimulator {
    uinput_device: Mutex<Option<VirtualDevice>>,
}

impl LinuxSimulator {
    pub fn new() -> Self {
        Self {
            uinput_device: Mutex::new(None),
        }
    }

    fn ensure_uinput(&self) -> Result<(), SimulatorError> {
        let mut dev = self
            .uinput_device
            .lock()
            .map_err(|e| SimulatorError::InternalError(e.to_string()))?;
        if dev.is_none() {
            *dev = Some(VirtualDevice::default().map_err(|e| {
                SimulatorError::OSError(format!("Failed to create uinput device: {}", e))
            })?);
        }
        Ok(())
    }
}

impl InputSimulator for LinuxSimulator {
    fn key_down(&self, key: Key) -> Result<(), SimulatorError> {
        // Try ydotool first
        // ydotool key keycode:1 (1 for down, 0 for up)
        // Note: ydotool key codes are often different or require translation

        // For now, let's focus on the uinput fallback as it's more reliable if permissions are set
        self.ensure_uinput()?;
        let mut dev = self
            .uinput_device
            .lock()
            .map_err(|e| SimulatorError::InternalError(e.to_string()))?;
        if let Some(d) = dev.as_mut() {
            if let Some(code) = map_key(key) {
                d.press(code)
                    .map_err(|e| SimulatorError::OSError(e.to_string()))?;
                d.synchronize()
                    .map_err(|e| SimulatorError::OSError(e.to_string()))?;
            }
        }
        Ok(())
    }

    fn key_up(&self, key: Key) -> Result<(), SimulatorError> {
        self.ensure_uinput()?;
        let mut dev = self
            .uinput_device
            .lock()
            .map_err(|e| SimulatorError::InternalError(e.to_string()))?;
        if let Some(d) = dev.as_mut() {
            if let Some(code) = map_key(key) {
                d.release(code)
                    .map_err(|e| SimulatorError::OSError(e.to_string()))?;
                d.synchronize()
                    .map_err(|e| SimulatorError::OSError(e.to_string()))?;
            }
        }
        Ok(())
    }

    fn mouse_down(&self, button: MouseButton) -> Result<(), SimulatorError> {
        self.ensure_uinput()?;
        let mut dev = self
            .uinput_device
            .lock()
            .map_err(|e| SimulatorError::InternalError(e.to_string()))?;
        if let Some(d) = dev.as_mut() {
            let code = match button {
                MouseButton::Left => mouse_keyboard_input::key_codes::BTN_LEFT,
                MouseButton::Right => mouse_keyboard_input::key_codes::BTN_RIGHT,
                MouseButton::Middle => mouse_keyboard_input::key_codes::BTN_MIDDLE,
            };
            d.press(code)
                .map_err(|e| SimulatorError::OSError(e.to_string()))?;
            d.synchronize()
                .map_err(|e| SimulatorError::OSError(e.to_string()))?;
        }
        Ok(())
    }

    fn mouse_up(&self, button: MouseButton) -> Result<(), SimulatorError> {
        self.ensure_uinput()?;
        let mut dev = self
            .uinput_device
            .lock()
            .map_err(|e| SimulatorError::InternalError(e.to_string()))?;
        if let Some(d) = dev.as_mut() {
            let code = match button {
                MouseButton::Left => mouse_keyboard_input::key_codes::BTN_LEFT,
                MouseButton::Right => mouse_keyboard_input::key_codes::BTN_RIGHT,
                MouseButton::Middle => mouse_keyboard_input::key_codes::BTN_MIDDLE,
            };
            d.release(code)
                .map_err(|e| SimulatorError::OSError(e.to_string()))?;
            d.synchronize()
                .map_err(|e| SimulatorError::OSError(e.to_string()))?;
        }
        Ok(())
    }

    fn mouse_move(&self, x: i32, y: i32) -> Result<(), SimulatorError> {
        self.ensure_uinput()?;
        let mut dev = self
            .uinput_device
            .lock()
            .map_err(|e| SimulatorError::InternalError(e.to_string()))?;
        if let Some(d) = dev.as_mut() {
            d.move_mouse(x, y)
                .map_err(|e| SimulatorError::OSError(e.to_string()))?;
            d.synchronize()
                .map_err(|e| SimulatorError::OSError(e.to_string()))?;
        }
        Ok(())
    }
}

fn map_key(key: Key) -> Option<u16> {
    use mouse_keyboard_input::key_codes::*;
    match key {
        Key::Char('a') | Key::Char('A') => Some(KEY_A),
        Key::Char('b') | Key::Char('B') => Some(KEY_B),
        Key::Char('c') | Key::Char('C') => Some(KEY_C),
        Key::Char('d') | Key::Char('D') => Some(KEY_D),
        Key::Char('e') | Key::Char('E') => Some(KEY_E),
        Key::Char('f') | Key::Char('F') => Some(KEY_F),
        Key::Char('g') | Key::Char('G') => Some(KEY_G),
        Key::Char('h') | Key::Char('H') => Some(KEY_H),
        Key::Char('i') | Key::Char('I') => Some(KEY_I),
        Key::Char('j') | Key::Char('J') => Some(KEY_J),
        Key::Char('k') | Key::Char('K') => Some(KEY_K),
        Key::Char('l') | Key::Char('L') => Some(KEY_L),
        Key::Char('m') | Key::Char('M') => Some(KEY_M),
        Key::Char('n') | Key::Char('N') => Some(KEY_N),
        Key::Char('o') | Key::Char('O') => Some(KEY_O),
        Key::Char('p') | Key::Char('P') => Some(KEY_P),
        Key::Char('q') | Key::Char('Q') => Some(KEY_Q),
        Key::Char('r') | Key::Char('R') => Some(KEY_R),
        Key::Char('s') | Key::Char('S') => Some(KEY_S),
        Key::Char('t') | Key::Char('T') => Some(KEY_T),
        Key::Char('u') | Key::Char('U') => Some(KEY_U),
        Key::Char('v') | Key::Char('V') => Some(KEY_V),
        Key::Char('w') | Key::Char('W') => Some(KEY_W),
        Key::Char('x') | Key::Char('X') => Some(KEY_X),
        Key::Char('y') | Key::Char('Y') => Some(KEY_Y),
        Key::Char('z') | Key::Char('Z') => Some(KEY_Z),
        Key::Char('0') => Some(KEY_10),
        Key::Char('1') => Some(KEY_1),
        Key::Char('2') => Some(KEY_2),
        Key::Char('3') => Some(KEY_3),
        Key::Char('4') => Some(KEY_4),
        Key::Char('5') => Some(KEY_5),
        Key::Char('6') => Some(KEY_6),
        Key::Char('7') => Some(KEY_7),
        Key::Char('8') => Some(KEY_8),
        Key::Char('9') => Some(KEY_9),
        Key::Control => Some(KEY_LEFTCTRL),
        Key::Shift => Some(KEY_LEFTSHIFT),
        Key::Alt => Some(KEY_LEFTALT),
        Key::Meta => Some(KEY_LEFTMETA),
        Key::Escape => Some(KEY_ESC),
        Key::Enter => Some(KEY_ENTER),
        Key::Backspace => Some(KEY_BACKSPACE),
        Key::Tab => Some(KEY_TAB),
        Key::Space => Some(KEY_SPACE),
        Key::Up => Some(KEY_UP),
        Key::Down => Some(KEY_DOWN),
        Key::Left => Some(KEY_LEFT),
        Key::Right => Some(KEY_RIGHT),
        Key::F1 => Some(KEY_F1),
        Key::F2 => Some(KEY_F2),
        Key::F3 => Some(KEY_F3),
        Key::F4 => Some(KEY_F4),
        Key::F5 => Some(KEY_F5),
        Key::F6 => Some(KEY_F6),
        Key::F7 => Some(KEY_F7),
        Key::F8 => Some(KEY_F8),
        Key::F9 => Some(KEY_F9),
        Key::F10 => Some(KEY_F10),
        Key::F11 => Some(KEY_F11),
        Key::F12 => Some(KEY_F12),
        _ => None,
    }
}
