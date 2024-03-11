use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use uuid::Uuid;

fn format_date(date: DateTime<Utc>) -> String {
    date.format("%d-%m-%Y").to_string()
}

pub fn optional_date_to_str(date: Option<DateTime<Utc>>, alt_str: Option<String>) -> String {
    if date.is_some() {
        format_date(date.unwrap())
    } else {
        let ret_str = alt_str.or(Some("N.A.".to_string())).unwrap();
        ret_str
    }
}

pub fn optional_str_to_date(str_date: Option<String>) -> Option<DateTime<Utc>> {
    if str_date.is_some() {
        let str_date_unwrapped = str_date.as_ref().unwrap();
        let hour: &str = " 00:00";
        let concat = str_date_unwrapped.to_owned() + hour;
        let date = NaiveDateTime::parse_from_str(&concat, "%d-%m-%Y %H:%M").ok();
        let tz = FixedOffset::east_opt(1 * 3600).unwrap();
        let dt_with_tz: DateTime<FixedOffset> = tz.from_utc_datetime(&date.unwrap());
        let res: DateTime<Utc> = Utc.from_utc_datetime(&dt_with_tz.naive_utc());
        Some(res)
    } else {
        None
    }
}

pub struct TodoItem {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl TodoItem {
    pub fn new(description: String, expiry_date: Option<String>) -> TodoItem {
        let now = Utc::now();
        let expires_at = optional_str_to_date(expiry_date);
        TodoItem {
            id: Uuid::new_v4(),
            description,
            completed: false,
            created_at: now,
            completed_at: None,
            expires_at,
        }
    }

    pub fn toggle_completed(&mut self) -> &mut Self {
        self.set_completed(!self.completed);
        self
    }

    pub fn set_completed(&mut self, completed: bool) -> &mut Self {
        self.completed = completed;
        let now = Utc::now();
        self.completed_at = Some(now);
        self
    }
}
