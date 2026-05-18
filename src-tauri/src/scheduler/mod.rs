pub mod types;

use self::types::*;
use crate::simulator::InputSimulator;
use dashmap::DashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Instant};
use uuid::Uuid;

pub struct Scheduler {
    pub active_tasks: DashMap<Uuid, JoinHandle<()>>,
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

        let simulator = self.simulator.clone();
        let handle = tokio::spawn(async move {
            run_schedule_loop(schedule, simulator, app).await;
        });

        self.active_tasks.insert(id, handle);
        Ok(())
    }

    pub fn stop_schedule(&self, id: &Uuid) -> bool {
        if let Some((_, handle)) = self.active_tasks.remove(id) {
            handle.abort();
            true
        } else {
            false
        }
    }

    pub fn stop_all_schedules(&self) {
        for entry in self.active_tasks.iter() {
            entry.value().abort();
        }
        self.active_tasks.clear();
    }
}

async fn run_schedule_loop(
    schedule: Schedule,
    simulator: Arc<dyn InputSimulator + Send + Sync>,
    app: AppHandle,
) {
    // Outer loop for "repeat_after" logic (Example A: repeat after 15s pause)
    loop {
        let active_window_start = Instant::now();

        // Middle loop for "active_duration" (Example A: 10 minutes window)
        while active_window_start.elapsed() < schedule.active_duration {
            let cycle_start = Instant::now();

            // Execution of the command sequence
            for command in &schedule.sequence {
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
                        sleep(*d).await;
                    }
                }
            }

            // Interval handling (Example A: 0.9s between presses)
            let elapsed = cycle_start.elapsed();
            if elapsed < schedule.interval {
                sleep(schedule.interval - elapsed).await;
            }

            // Check if we should stop early (if active_duration reached)
            if active_window_start.elapsed() >= schedule.active_duration {
                break;
            }
        }

        // After active window, handle repeat_after
        if let Some(pause) = schedule.repeat_after {
            sleep(pause).await;
        } else {
            // No repeat_after means we just finish
            break;
        }
    }

    // Notify completion
    app.notification()
        .builder()
        .title("Clickease")
        .body(format!("Schedule '{}' completed", schedule.name))
        .show()
        .unwrap_or_default();

    let _ = app.emit("schedule-completed", schedule.id);

    // Clean up task from active_tasks
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

        run_schedule_loop(schedule, simulator.clone()).await;

        // Should have run at least a few times in 50ms with 10ms interval
        let counts = simulator.click_count.load(Ordering::SeqCst);
        assert!(counts >= 4 && counts <= 6);
    }
}
