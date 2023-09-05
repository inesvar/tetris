pub fn format_seconds(seconds: f64) -> String {
    let minutes: u64 = seconds as u64 / 60;
    let seconds = seconds % 60.0;
    format!("{:}m {:.2}s", minutes, seconds)
}