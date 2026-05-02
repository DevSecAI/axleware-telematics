// AXL-SAST-004: unsafe pointer arithmetic on attacker-controlled length.
// AXL-SAST-009: division by zero on attacker-supplied frame.
pub unsafe fn parse_frame(buf: *const u8, len: usize) -> u32 {
    let declared_len = *buf.add(0) as usize;     // attacker-controlled
    // AXL-SAST-004: read past end of buf if declared_len > len.
    let payload_byte = *buf.add(declared_len);
    // AXL-SAST-009: divisor sourced from frame, can be zero.
    let scale = *buf.add(2) as u32;
    (payload_byte as u32) / scale
}

pub fn parse_frame_safe(buf: &[u8]) -> Option<u32> {
    if buf.len() < 3 { return None; }
    unsafe { Some(parse_frame(buf.as_ptr(), buf.len())) }
}
