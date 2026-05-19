pub mod types;

use self::types::*;
use crate::simulator::InputSimulator;
use dashmap::DashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration, Instant};
use uuid::Uuid;

pub struct TaskControl {
    pub handle: JoinHandle<()>,
    pub is_paused: Arc<AtomicBool>,
}

pub struct Scheduler {
    pub active_tasks: DashMap<Uuid, TaskControl>,
    simulator: Arc<dyn InputSimulator + Send + Sync>,
}

impl Scheduler {
    pub fn new(simulator: Arc<dyn InputSimulator + Send + Sync>) -> Self {
        Self {
            active_tasks: DashMap::new(),
            simulator,
        }
    }

    pub fn start_schedule(&self, schedule: Schedule, app: AppHandle) -> Result<(), String> {
        let id = schedule.id;
        if self.active_tasks.contains_key(&id) {
            return Err("Schedule already running".into());
        }

        let is_paused = Arc::new(AtomicBool::new(false));
        let is_paused_clone = is_paused.clone();
        let simulator = self.simulator.clone();

        let handle = tokio::spawn(async move {
            run_schedule_loop(schedule, simulator, app, is_paused_clone).await;
        });

        self.active_tasks
            .insert(id, TaskControl { handle, is_paused });
        Ok(())
    }

    pub fn stop_schedule(&self, id: &Uuid) -> bool {
        if let Some((_, control)) = self.active_tasks.remove(id) {
            control.handle.abort();
            true
        } else {
            false
        }
    }

    pub fn pause_schedule(&self, id: &Uuid) -> bool {
        if let Some(control) = self.active_tasks.get(id) {
            control.is_paused.store(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    pub fn resume_schedule(&self, id: &Uuid) -> bool {
        if let Some(control) = self.active_tasks.get(id) {
            control.is_paused.store(false, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    pub fn stop_all_schedules(&self) {
        for entry in self.active_tasks.iter() {
            entry.value().handle.abort();
        }
        self.active_tasks.clear();
    }
}

#[derive(serde::Serialize, Clone)]
struct TickPayload {
    id: Uuid,
    remaining_secs: u64,
    is_paused: bool,
}

async fn run_schedule_loop(
    schedule: Schedule,
    simulator: Arc<dyn InputSimulator + Send + Sync>,
    app: AppHandle,
    is_paused: Arc<AtomicBool>,
) {
    let mut total_remaining = schedule.active_duration;
    let tick_interval = Duration::from_secs(1);

    loop {
        let _ = app.emit(
            "schedule-tick",
            TickPayload {
                id: schedule.id,
                remaining_secs: total_remaining.as_secs(),
                is_paused: is_paused.load(Ordering::SeqCst),
            },
        );

        if total_remaining.is_zero() {
            break;
        }

        if is_paused.load(Ordering::SeqCst) {
            sleep(tick_interval).await;
            continue;
        }

        let cycle_start = Instant::now();

        for command in &schedule.sequence {
            while is_paused.load(Ordering::SeqCst) {
                sleep(Duration::from_millis(100)).await;
            }

            match command {
                Command::KeyPress(k) => {
                    let _ = simulator.key_click(k.clone());
                }
                Command::KeyDown(k) => {
                    let _ = simulator.key_down(k.clone());
                }
                Command::KeyUp(k) => {
                    let _ = simulator.key_up(k.clone());
                }
                Command::MouseClick(b) => {
                    let _ = simulator.mouse_down(b.clone());
                    let _ = simulator.mouse_up(b.clone());
                }
                Command::MouseDown(b) => {
                    let _ = simulator.mouse_down(b.clone());
                }
                Command::MouseUp(b) => {
                    let _ = simulator.mouse_up(b.clone());
                }
                Command::MouseMove { x, y } => {
                    let _ = simulator.mouse_move(*x, *y);
                }
                Command::Wait(d) => {
                    let mut wait_remaining = *d;
                    while !wait_remaining.is_zero() {
                        if is_paused.load(Ordering::SeqCst) {
                            sleep(Duration::from_millis(100)).await;
                            continue;
                        }
                        let step = wait_remaining.min(Duration::from_millis(100));
                        sleep(step).await;
                        wait_remaining -= step;
                    }
                }
            }
        }

        let mut interval_remaining = schedule.interval.saturating_sub(cycle_start.elapsed());
        while !interval_remaining.is_zero() {
            if is_paused.load(Ordering::SeqCst) {
                sleep(Duration::from_millis(100)).await;
                continue;
            }
            let step = interval_remaining.min(Duration::from_millis(100));
            sleep(step).await;
            interval_remaining -= step;
        }

        total_remaining = total_remaining.saturating_sub(cycle_start.elapsed().max(tick_interval));
    }

    app.notification()
        .builder()
        .title("Clickease")
        .body(format!("Schedule '{}' completed", schedule.name))
        .show()
        .unwrap_or_default();

    let _ = app.emit("schedule-completed", schedule.id);

    if let Some(state) = app.try_state::<crate::AppState>() {
        state.scheduler.active_tasks.remove(&schedule.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulator::types::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    struct MockSimulator {
        click_count: AtomicUsize,
    }

    impl MockSimulator {
        fn new() -> Self {
            Self {
                click_count: AtomicUsize::new(0),
            }
        }
    }

    impl InputSimulator for MockSimulator {
        fn key_down(&self, _key: Key) -> Result<(), SimulatorError> {
            Ok(())
        }
        fn key_up(&self, _key: Key) -> Result<(), SimulatorError> {
            self.click_count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        fn mouse_down(&self, _button: MouseButton) -> Result<(), SimulatorError> {
            Ok(())
        }
        fn mouse_up(&self, _button: MouseButton) -> Result<(), SimulatorError> {
            Ok(())
        }
        fn mouse_move(&self, _x: i32, _y: i32) -> Result<(), SimulatorError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_schedule_loop() {
        let simulator = Arc::new(MockSimulator::new());
        let schedule = Schedule {
            id: Uuid::new_v4(),
            name: "test".into(),
            sequence: vec![Command::KeyPress(Key::Char('f'))],
            interval: Duration::from_millis(10),
            active_duration: Duration::from_millis(50),
            repeat_after: None,
        };

        run_schedule_loop(
            schedule,
            simulator.clone(),
            Default::default(),
            Arc::new(AtomicBool::new(false)),
        )
        .await;

        let counts = simulator.click_count.load(Ordering::SeqCst);
        assert!(counts >= 4 && counts <= 6);
    }
}
