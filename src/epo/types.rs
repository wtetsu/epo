use super::date;

pub struct Settings {
    pub epochs: Vec<date::EpochInfo>,
    pub dates: Vec<date::DateInfo>,
    pub timezones: Vec<Zone>,
    pub time_mode: TimeMode,
    pub print_mode: PrintMode,
    pub help: bool,
    pub version: bool,
}

pub enum Zone {
    Offset(i32),
    Tzname(String),
}

pub enum TimeMode {
    Seconds,
    Milliseconds,
}

pub enum PrintMode {
    Markdown,
    PlainText,
}
