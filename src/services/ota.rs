// AXL-SAST-002: command injection via sh -c.
use tokio::process::Command;

pub async fn dispatch(vin: &str, version: &str) -> std::io::Result<std::process::Output> {
    Command::new("sh")
        .arg("-c")
        // AXL-SAST-002: vin and version interpolated into the shell argv.
        .arg(format!("/opt/axleware/bin/ota-push --vin {} --version {}", vin, version))
        .output()
        .await
}
