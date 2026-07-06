// AXL-SAST-008: path traversal in firmware bundle download.
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const ROOT: &str = "/var/axleware/firmware";

pub async fn read(name: &str) -> std::io::Result<Vec<u8>> {
    // Extract only the filename component, rejecting any path with directory separators
    let filename = Path::new(name)
        .file_name()
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid filename: path traversal attempt detected"
        ))?;
    
    let mut p = PathBuf::from(ROOT);
    p.push(filename);
    let mut f = File::open(p).await?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).await?;
    Ok(buf)
}
