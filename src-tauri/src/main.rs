fn main() {
    #[cfg(target_os = "linux")]
    {
        const REEXEC_KEY: &str = "CLICKEASE_REEXEC";
        if std::env::var(REEXEC_KEY).is_err() {
            // Set all critical rendering fallbacks
            std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
            std::env::set_var("GDK_BACKEND", "x11");
            std::env::set_var("GDK_GL", "gles");
            std::env::set_var(REEXEC_KEY, "1");

            let args: Vec<String> = std::env::args().collect();
            let mut cmd = std::process::Command::new(&args[0]);
            cmd.args(&args[1..]);

            // Prepend host library paths to ensure host graphics drivers take precedence
            // over bundled ones in the AppImage.
            if let Ok(old_path) = std::env::var("LD_LIBRARY_PATH") {
                cmd.env(
                    "LD_LIBRARY_PATH",
                    format!("/usr/lib64:/usr/lib:{}", old_path),
                );
            } else {
                cmd.env("LD_LIBRARY_PATH", "/usr/lib64:/usr/lib");
            }

            match cmd.spawn() {
                Ok(mut child) => {
                    let status = child.wait().expect("Failed to wait for child process");
                    std::process::exit(status.code().unwrap_or(0));
                }
                Err(_) => {
                    // Fallback to normal execution if re-exec fails
                }
            }
        }
    }

    clickease_lib::run()
}
