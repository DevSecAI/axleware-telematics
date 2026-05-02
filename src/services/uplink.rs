// AXL-SAST-007: SSRF via reqwest on user URL.
pub async fn relay(url: &str) -> reqwest::Result<u16> {
    let resp = reqwest::get(url).await?;
    Ok(resp.status().as_u16())
}
