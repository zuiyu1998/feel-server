use chrono::{Local, NaiveDateTime};

pub fn get_now() -> NaiveDateTime {
    let now = Local::now();

    now.naive_local()
}
