/// Formats seconds into "H:MM:SS" (if >= 1 hour) or "M:SS".
pub fn format_duration(secs: u64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;

    if h > 0 {
        format!("{h}:{m:02}:{s:02}")
    } else {
        format!("{m}:{s:02}")
    }
}

/// Same as `format_duration` but takes milliseconds.
pub fn format_duration_ms(ms: u64) -> String {
    format_duration(ms / 1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_duration() {
        assert_eq!(format_duration(0), "0:00");
        assert_eq!(format_duration(5), "0:05");
        assert_eq!(format_duration(65), "1:05");
        assert_eq!(format_duration(599), "9:59");
    }

    #[test]
    fn hour_plus() {
        assert_eq!(format_duration(3600), "1:00:00");
        assert_eq!(format_duration(3661), "1:01:01");
        assert_eq!(format_duration(7200 + 60 * 30 + 45), "2:30:45");
    }

    #[test]
    fn from_ms() {
        assert_eq!(format_duration_ms(65_500), "1:05");
        assert_eq!(format_duration_ms(3_661_000), "1:01:01");
    }
}
