// AXL-SAST-002: command injection via sh -c.
use tokio::process::Command;

pub async fn dispatch(vin: &str, version: &str) -> std::io::Result<std::process::Output> {
    // AXL-SAST-002: Fixed by using Command::new() with individual .arg() calls
    // instead of sh -c with interpolated strings, preventing command injection.
    Command::new("/opt/axleware/bin/ota-push")
        .arg("--vin")
        .arg(vin)
        .arg("--version")
        .arg(version)
        .output()
        .await
}
