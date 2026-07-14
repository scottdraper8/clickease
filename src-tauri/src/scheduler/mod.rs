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
    let pause_poll_interval = Duration::from_millis(100);

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
            sleep(pause_poll_interval).await;
            continue;
        }

        let mut cycle_active_elapsed = Duration::ZERO;

        for command in &schedule.sequence {
            wait_until_resumed(&is_paused, pause_poll_interval).await;

            let mut command_active_elapsed = Duration::ZERO;
            match command {
                Command::KeyPress(k) => {
                    let command_start = Instant::now();
                    let _ = simulator.key_click(k.clone());
                    command_active_elapsed += command_start.elapsed();
                }
                Command::KeyDown(k) => {
                    let command_start = Instant::now();
                    let _ = simulator.key_down(k.clone());
                    command_active_elapsed += command_start.elapsed();
                }
                Command::KeyUp(k) => {
                    let command_start = Instant::now();
                    let _ = simulator.key_up(k.clone());
                    command_active_elapsed += command_start.elapsed();
                }
                Command::MouseClick(b) => {
                    let command_start = Instant::now();
                    let _ = simulator.mouse_down(b.clone());
                    let _ = simulator.mouse_up(b.clone());
                    command_active_elapsed += command_start.elapsed();
                }
                Command::MouseDown(b) => {
                    let command_start = Instant::now();
                    let _ = simulator.mouse_down(b.clone());
                    command_active_elapsed += command_start.elapsed();
                }
                Command::MouseUp(b) => {
                    let command_start = Instant::now();
                    let _ = simulator.mouse_up(b.clone());
                    command_active_elapsed += command_start.elapsed();
                }
                Command::MouseMove { x, y } => {
                    let command_start = Instant::now();
                    let _ = simulator.mouse_move(*x, *y);
                    command_active_elapsed += command_start.elapsed();
                }
                Command::Wait(d) => {
                    command_active_elapsed +=
                        sleep_active(*d, &is_paused, pause_poll_interval).await;
                }
            }
            cycle_active_elapsed += command_active_elapsed;
        }

        let interval_remaining = schedule.interval.saturating_sub(cycle_active_elapsed);
        cycle_active_elapsed +=
            sleep_active(interval_remaining, &is_paused, pause_poll_interval).await;

        if cycle_active_elapsed.is_zero() {
            cycle_active_elapsed +=
                sleep_active(Duration::from_millis(1), &is_paused, pause_poll_interval).await;
        }

        total_remaining = total_remaining.saturating_sub(cycle_active_elapsed);
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

async fn wait_until_resumed(is_paused: &AtomicBool, poll_interval: Duration) {
    while is_paused.load(Ordering::SeqCst) {
        sleep(poll_interval).await;
    }
}

async fn sleep_active(
    duration: Duration,
    is_paused: &AtomicBool,
    poll_interval: Duration,
) -> Duration {
    let mut remaining = duration;
    let mut active_elapsed = Duration::ZERO;

    while !remaining.is_zero() {
        if is_paused.load(Ordering::SeqCst) {
            sleep(poll_interval).await;
            continue;
        }

        let step = remaining.min(poll_interval);
        let step_started_at = Instant::now();
        sleep(step).await;
        let elapsed = step_started_at.elapsed().min(step);
        active_elapsed += elapsed;
        remaining = remaining.saturating_sub(elapsed);
    }

    active_elapsed
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    #[tokio::test]
    async fn sleep_active_does_not_count_paused_wall_time() {
        let is_paused = Arc::new(AtomicBool::new(true));
        let resume_flag = is_paused.clone();

        tokio::spawn(async move {
            sleep(Duration::from_millis(30)).await;
            resume_flag.store(false, Ordering::SeqCst);
        });

        let wall_start = Instant::now();
        let active_elapsed = sleep_active(
            Duration::from_millis(20),
            &is_paused,
            Duration::from_millis(5),
        )
        .await;
        let wall_elapsed = wall_start.elapsed();

        assert_eq!(active_elapsed, Duration::from_millis(20));
        assert!(wall_elapsed >= Duration::from_millis(45));
    }
}
