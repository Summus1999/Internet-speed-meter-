use chrono::Local;

/// Format current timestamp
#[allow(dead_code)]
pub fn get_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Convert bytes to human readable format
#[allow(dead_code)]
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Calculate speed from bytes and duration (in seconds)
#[allow(dead_code)]
pub fn calculate_speed(bytes: u64, duration_secs: f64) -> f64 {
    if duration_secs <= 0.0 {
        return 0.0;
    }
    // Convert to megabits per second
    (bytes as f64 * 8.0) / (duration_secs * 1_000_000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512.00 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
    }

    #[test]
    fn test_calculate_speed() {
        // 1MB in 1 second = 8 Mbps
        assert_eq!(calculate_speed(1_000_000, 1.0), 8.0);
    }
}
