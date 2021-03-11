use {
    chrono::{
        DateTime,
        Utc,
        Duration,
    },
};

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start.checked_add_signed(Duration::seconds(10i64.pow(9)))
        .expect("Overflow")
}
