// AXL-SAST-008: path traversal in firmware bundle download.
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const ROOT: &str = "/var/axleware/firmware";

pub async fn read(name: &str) -> std::io::Result<Vec<u8>> {
    let mut p = PathBuf::from(ROOT);
    p.push(name);                 // AXL-SAST-008: ../ escapes ROOT.
    let mut f = File::open(p).await?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).await?;
    Ok(buf)
}
