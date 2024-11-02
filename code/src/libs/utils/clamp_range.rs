pub fn clamp_range(len: usize, start: i64, end: i64) -> (i64, i64) {
    let start = if start < 0 { len as i64 + start } else { start }.max(0);

    let end = if end < 0 {
        len as i64 + end + 1
    } else {
        end + 1
    }
    .min(len as i64);

    (start, end)
}