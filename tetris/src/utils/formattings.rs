pub fn format_seconds(time: f64) -> String {
    let minutes: u64 = (time.floor() as u64) / 60;
    let seconds = time % 60.0;
    format!("{:02}:{:05.2}", minutes, seconds)
}
