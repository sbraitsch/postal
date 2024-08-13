pub fn format_size(bytes: usize) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    if size.fract() == 0.0 {
        format!("{:.0} {}", size, units[unit_index])
    } else {
        format!("{:.2} {}", size, units[unit_index])
    }
}
