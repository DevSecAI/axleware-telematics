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
    
    // Canonicalize and verify the resolved path starts with the intended base directory
    let canonical = p.canonicalize().await?;
    let base_canonical = PathBuf::from(ROOT).canonicalize().await?;
    
    if !canonical.starts_with(&base_canonical) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Path traversal attempt detected: resolved path outside base directory"
        ));
    }
    
    let mut f = File::open(canonical).await?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).await?;
    Ok(buf)
}
