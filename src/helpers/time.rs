use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Local, NaiveDateTime, TimeDelta};

pub fn now_plus_seconds(sec: i64) -> NaiveDateTime {
    (Local::now() + TimeDelta::try_seconds(sec).unwrap()).naive_local()
}

pub fn now_plus_minutes(min: i64) -> NaiveDateTime {
    now_plus_seconds(min * 60)
}

pub fn current_datetime() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch")
        .as_secs()
}
