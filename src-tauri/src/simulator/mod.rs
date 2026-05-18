pub mod types;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub use types::*;

pub fn get_simulator() -> Result<Box<dyn InputSimulator + Send + Sync>, SimulatorError> {
    #[cfg(target_os = "windows")]
    return Ok(Box::new(windows::WindowsSimulator::new()));

    #[cfg(target_os = "macos")]
    return Ok(Box::new(macos::MacosSimulator::new()));

    #[cfg(target_os = "linux")]
    return Ok(Box::new(linux::LinuxSimulator::new()));

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return Err(SimulatorError::UnsupportedPlatform(
        "Current OS is not supported".into(),
    ));
}
