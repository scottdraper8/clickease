use crate::simulator::types::*;
use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::geometry::CGPoint;

pub struct MacosSimulator;

impl MacosSimulator {
    pub fn new() -> Self {
        Self {}
    }

    fn get_source() -> CGEventSource {
        CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
            .expect("Failed to create event source")
    }
}

impl InputSimulator for MacosSimulator {
    fn key_down(&self, key: Key) -> Result<(), SimulatorError> {
        if let Some(code) = map_key(key) {
            let event = CGEvent::new_keyboard_event(Self::get_source(), code, true)
                .map_err(|_| SimulatorError::OSError("Failed to create key down event".into()))?;
            event.post(CGEventTapLocation::HID);
        }
        Ok(())
    }

    fn key_up(&self, key: Key) -> Result<(), SimulatorError> {
        if let Some(code) = map_key(key) {
            let event = CGEvent::new_keyboard_event(Self::get_source(), code, false)
                .map_err(|_| SimulatorError::OSError("Failed to create key up event".into()))?;
            event.post(CGEventTapLocation::HID);
        }
        Ok(())
    }

    fn mouse_down(&self, button: MouseButton) -> Result<(), SimulatorError> {
        let (cg_button, cg_type) = match button {
            MouseButton::Left => (CGMouseButton::Left, CGEventType::LeftMouseDown),
            MouseButton::Right => (CGMouseButton::Right, CGEventType::RightMouseDown),
            MouseButton::Middle => (CGMouseButton::Center, CGEventType::OtherMouseDown),
        };
        let event = CGEvent::new_mouse_event(
            Self::get_source(),
            cg_type,
            CGPoint::new(0.0, 0.0),
            cg_button,
        )
        .map_err(|_| SimulatorError::OSError("Failed to create mouse down event".into()))?;
        event.post(CGEventTapLocation::HID);
        Ok(())
    }

    fn mouse_up(&self, button: MouseButton) -> Result<(), SimulatorError> {
        let (cg_button, cg_type) = match button {
            MouseButton::Left => (CGMouseButton::Left, CGEventType::LeftMouseUp),
            MouseButton::Right => (CGMouseButton::Right, CGEventType::RightMouseUp),
            MouseButton::Middle => (CGMouseButton::Center, CGEventType::OtherMouseUp),
        };
        let event = CGEvent::new_mouse_event(
            Self::get_source(),
            cg_type,
            CGPoint::new(0.0, 0.0),
            cg_button,
        )
        .map_err(|_| SimulatorError::OSError("Failed to create mouse up event".into()))?;
        event.post(CGEventTapLocation::HID);
        Ok(())
    }

    fn mouse_move(&self, x: i32, y: i32) -> Result<(), SimulatorError> {
        let event = CGEvent::new_mouse_event(
            Self::get_source(),
            CGEventType::MouseMoved,
            CGPoint::new(x as f64, y as f64),
            CGMouseButton::Left,
        )
        .map_err(|_| SimulatorError::OSError("Failed to create mouse move event".into()))?;
        event.post(CGEventTapLocation::HID);
        Ok(())
    }
}

fn map_key(key: Key) -> Option<u16> {
    match key {
        Key::Char('a') | Key::Char('A') => Some(0),
        Key::Char('b') | Key::Char('B') => Some(11),
        Key::Char('c') | Key::Char('C') => Some(8),
        Key::Char('d') | Key::Char('D') => Some(2),
        Key::Char('e') | Key::Char('E') => Some(14),
        Key::Char('f') | Key::Char('F') => Some(3),
        Key::Char('g') | Key::Char('G') => Some(5),
        Key::Char('h') | Key::Char('H') => Some(4),
        Key::Char('i') | Key::Char('I') => Some(34),
        Key::Char('j') | Key::Char('J') => Some(38),
        Key::Char('k') | Key::Char('K') => Some(40),
        Key::Char('l') | Key::Char('L') => Some(37),
        Key::Char('m') | Key::Char('M') => Some(46),
        Key::Char('n') | Key::Char('N') => Some(45),
        Key::Char('o') | Key::Char('O') => Some(31),
        Key::Char('p') | Key::Char('P') => Some(35),
        Key::Char('q') | Key::Char('Q') => Some(12),
        Key::Char('r') | Key::Char('R') => Some(15),
        Key::Char('s') | Key::Char('S') => Some(1),
        Key::Char('t') | Key::Char('T') => Some(17),
        Key::Char('u') | Key::Char('U') => Some(32),
        Key::Char('v') | Key::Char('V') => Some(9),
        Key::Char('w') | Key::Char('W') => Some(13),
        Key::Char('x') | Key::Char('X') => Some(7),
        Key::Char('y') | Key::Char('Y') => Some(16),
        Key::Char('z') | Key::Char('Z') => Some(6),
        Key::Char('0') => Some(29),
        Key::Char('1') => Some(18),
        Key::Char('2') => Some(19),
        Key::Char('3') => Some(20),
        Key::Char('4') => Some(21),
        Key::Char('5') => Some(23),
        Key::Char('6') => Some(22),
        Key::Char('7') => Some(26),
        Key::Char('8') => Some(28),
        Key::Char('9') => Some(25),
        Key::Control => Some(59),
        Key::Shift => Some(56),
        Key::Alt => Some(58),
        Key::Meta => Some(55),
        Key::Escape => Some(53),
        Key::Enter => Some(36),
        Key::Backspace => Some(51),
        Key::Tab => Some(48),
        Key::Space => Some(49),
        Key::Up => Some(126),
        Key::Down => Some(125),
        Key::Left => Some(123),
        Key::Right => Some(124),
        Key::F1 => Some(122),
        Key::F2 => Some(120),
        Key::F3 => Some(99),
        Key::F4 => Some(118),
        Key::F5 => Some(96),
        Key::F6 => Some(97),
        Key::F7 => Some(98),
        Key::F8 => Some(100),
        Key::F9 => Some(101),
        Key::F10 => Some(109),
        Key::F11 => Some(103),
        Key::F12 => Some(111),
        _ => None,
    }
}

// Implement Send and Sync for MacosSimulator to enable use in Managed State.
// Thread-safe because event sources are created on-demand and HID event
// posting is thread-safe at the OS level.
unsafe impl Send for MacosSimulator {}
unsafe impl Sync for MacosSimulator {}
