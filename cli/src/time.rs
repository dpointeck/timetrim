use chrono::offset::Utc;
use chrono::DateTime;

pub fn get_utc_time() -> String {
    let utc: DateTime<Utc> = Utc::now();
    utc.to_rfc3339()
}
