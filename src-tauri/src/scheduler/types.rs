use crate::simulator::{Key, MouseButton};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    KeyPress(Key),
    KeyDown(Key),
    KeyUp(Key),
    MouseClick(MouseButton),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseMove { x: i32, y: i32 },
    Wait(Duration),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    pub sequence: Vec<Command>,
    pub interval: Duration,
    pub active_duration: Duration,
    pub repeat_after: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleStatus {
    Idle,
    Running,
    Paused,
    Completed,
}
