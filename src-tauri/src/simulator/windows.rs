use crate::simulator::types::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub struct WindowsSimulator;

impl WindowsSimulator {
    pub fn new() -> Self {
        Self
    }

    fn send_input(&self, input: INPUT) -> Result<(), SimulatorError> {
        let inputs = [input];
        unsafe {
            if SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) == 0 {
                return Err(SimulatorError::OSError("SendInput failed".into()));
            }
        }
        Ok(())
    }
}

impl InputSimulator for WindowsSimulator {
    fn key_down(&self, key: Key) -> Result<(), SimulatorError> {
        if let Some(vk) = map_key(key) {
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: vk,
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            self.send_input(input)
        } else {
            Ok(())
        }
    }

    fn key_up(&self, key: Key) -> Result<(), SimulatorError> {
        if let Some(vk) = map_key(key) {
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: vk,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            self.send_input(input)
        } else {
            Ok(())
        }
    }

    fn mouse_down(&self, button: MouseButton) -> Result<(), SimulatorError> {
        let flag = match button {
            MouseButton::Left => MOUSEEVENTF_LEFTDOWN,
            MouseButton::Right => MOUSEEVENTF_RIGHTDOWN,
            MouseButton::Middle => MOUSEEVENTF_MIDDLEDOWN,
        };
        let input = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: flag,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        self.send_input(input)
    }

    fn mouse_up(&self, button: MouseButton) -> Result<(), SimulatorError> {
        let flag = match button {
            MouseButton::Left => MOUSEEVENTF_LEFTUP,
            MouseButton::Right => MOUSEEVENTF_RIGHTUP,
            MouseButton::Middle => MOUSEEVENTF_MIDDLEUP,
        };
        let input = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: flag,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        self.send_input(input)
    }

    fn mouse_move(&self, x: i32, y: i32) -> Result<(), SimulatorError> {
        let input = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: x,
                    dy: y,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        self.send_input(input)
    }
}

fn map_key(key: Key) -> Option<VIRTUAL_KEY> {
    match key {
        Key::Char('a') | Key::Char('A') => Some(VIRTUAL_KEY(0x41)),
        Key::Char('b') | Key::Char('B') => Some(VIRTUAL_KEY(0x42)),
        Key::Char('c') | Key::Char('C') => Some(VIRTUAL_KEY(0x43)),
        Key::Char('d') | Key::Char('D') => Some(VIRTUAL_KEY(0x44)),
        Key::Char('e') | Key::Char('E') => Some(VIRTUAL_KEY(0x45)),
        Key::Char('f') | Key::Char('F') => Some(VIRTUAL_KEY(0x46)),
        Key::Char('g') | Key::Char('G') => Some(VIRTUAL_KEY(0x47)),
        Key::Char('h') | Key::Char('H') => Some(VIRTUAL_KEY(0x48)),
        Key::Char('i') | Key::Char('I') => Some(VIRTUAL_KEY(0x49)),
        Key::Char('j') | Key::Char('J') => Some(VIRTUAL_KEY(0x4A)),
        Key::Char('k') | Key::Char('K') => Some(VIRTUAL_KEY(0x4B)),
        Key::Char('l') | Key::Char('L') => Some(VIRTUAL_KEY(0x4C)),
        Key::Char('m') | Key::Char('M') => Some(VIRTUAL_KEY(0x4D)),
        Key::Char('n') | Key::Char('N') => Some(VIRTUAL_KEY(0x4E)),
        Key::Char('o') | Key::Char('O') => Some(VIRTUAL_KEY(0x4F)),
        Key::Char('p') | Key::Char('P') => Some(VIRTUAL_KEY(0x50)),
        Key::Char('q') | Key::Char('Q') => Some(VIRTUAL_KEY(0x51)),
        Key::Char('r') | Key::Char('R') => Some(VIRTUAL_KEY(0x52)),
        Key::Char('s') | Key::Char('S') => Some(VIRTUAL_KEY(0x53)),
        Key::Char('t') | Key::Char('T') => Some(VIRTUAL_KEY(0x54)),
        Key::Char('u') | Key::Char('U') => Some(VIRTUAL_KEY(0x55)),
        Key::Char('v') | Key::Char('V') => Some(VIRTUAL_KEY(0x56)),
        Key::Char('w') | Key::Char('W') => Some(VIRTUAL_KEY(0x57)),
        Key::Char('x') | Key::Char('X') => Some(VIRTUAL_KEY(0x58)),
        Key::Char('y') | Key::Char('Y') => Some(VIRTUAL_KEY(0x59)),
        Key::Char('z') | Key::Char('Z') => Some(VIRTUAL_KEY(0x5A)),
        Key::Char('0') => Some(VIRTUAL_KEY(0x30)),
        Key::Char('1') => Some(VIRTUAL_KEY(0x31)),
        Key::Char('2') => Some(VIRTUAL_KEY(0x32)),
        Key::Char('3') => Some(VIRTUAL_KEY(0x33)),
        Key::Char('4') => Some(VIRTUAL_KEY(0x34)),
        Key::Char('5') => Some(VIRTUAL_KEY(0x35)),
        Key::Char('6') => Some(VIRTUAL_KEY(0x36)),
        Key::Char('7') => Some(VIRTUAL_KEY(0x37)),
        Key::Char('8') => Some(VIRTUAL_KEY(0x38)),
        Key::Char('9') => Some(VIRTUAL_KEY(0x39)),
        Key::Control => Some(VK_CONTROL),
        Key::Shift => Some(VK_SHIFT),
        Key::Alt => Some(VK_MENU),
        Key::Meta => Some(VK_LWIN),
        Key::Escape => Some(VK_ESCAPE),
        Key::Enter => Some(VK_RETURN),
        Key::Backspace => Some(VK_BACK),
        Key::Tab => Some(VK_TAB),
        Key::Space => Some(VK_SPACE),
        Key::Up => Some(VK_UP),
        Key::Down => Some(VK_DOWN),
        Key::Left => Some(VK_LEFT),
        Key::Right => Some(VK_RIGHT),
        Key::F1 => Some(VK_F1),
        Key::F2 => Some(VK_F2),
        Key::F3 => Some(VK_F3),
        Key::F4 => Some(VK_F4),
        Key::F5 => Some(VK_F5),
        Key::F6 => Some(VK_F6),
        Key::F7 => Some(VK_F7),
        Key::F8 => Some(VK_F8),
        Key::F9 => Some(VK_F9),
        Key::F10 => Some(VK_F10),
        Key::F11 => Some(VK_F11),
        Key::F12 => Some(VK_F12),
        _ => None,
    }
}
