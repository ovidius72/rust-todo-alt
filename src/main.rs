use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};

fn format_date(date: DateTime<Utc>) -> String {
    date.format("%d-%m-%Y").to_string()
}

fn optional_date_to_str(date: Option<DateTime<Utc>>, alt_str: Option<String>) -> String {
    if date.is_some() {
        format_date(date.unwrap())
    } else {
        let ret_str = alt_str.or(Some("N.A.".to_string())).unwrap();
        ret_str
    }
}

fn optional_str_to_date(str_date: Option<String>) -> Option<DateTime<Utc>> {
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

struct TodoItem {
    description: String,
    completed: bool,
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
}

impl TodoItem {
    fn new(description: String, expiry_date: Option<String>) -> TodoItem {
        let now = Utc::now();
        let expires_at = optional_str_to_date(expiry_date);
        TodoItem {
            description,
            completed: false,
            created_at: now,
            completed_at: None,
            expires_at,
        }
    }

    fn toggle_completed(&mut self) {
        self.set_completed(!self.completed);
    }

    fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
        let now = Utc::now();
        self.completed_at = Some(now);
    }
}

struct TodoList {
    items: Vec<TodoItem>,
}
impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }
    fn add(&mut self, description: &str, expiry_date: &str) -> &mut Self {
        let exp_date = if !expiry_date.is_empty() {
            Some(expiry_date.to_owned())
        } else {
            None
        };
        self.items
            .push(TodoItem::new(description.to_owned(), exp_date));
        self
    }
    fn remove(&mut self, index: usize) -> &mut Self {
        if self.items.len() >= index {
            self.items.remove(index);
        }
        self
    }
    fn set_completed(&mut self, index: usize) -> &mut Self {
        self.items[index].set_completed(true);
        self
    }

    fn toggle_completed(&mut self, index: usize) -> &mut Self {
        self.items[index].toggle_completed();
        self
    }
    fn show_all(&mut self) {
        println!("");
        println!(
            "{0:<20} | {1:<10} | {2:<10} | {3:<10}",
            "Name", "Created At", "Done", "Expiry Date"
        );
        println!(
            "{0:<50}",
            "-------------------------------------------------------------"
        );
        self.items.iter().for_each(|item| {
            let created_at_str = optional_date_to_str(Some(item.created_at), None);
            let expire_at_str =
                optional_date_to_str(item.expires_at, Some("No due date".to_string()));
            let completed = if item.completed == true {
                "Completed"
            } else {
                "Pending"
            };
            println!(
                "{0:<20} | {1:<10} | {2:<10} | {3:<10}",
                item.description, created_at_str, completed, expire_at_str
            );
        });
    }
}

fn main() {
    let mut list = TodoList::new();
    list.add("Something", "");
    list.add("Another task", "10-10-2024");
    list.add("Another task2", "10-10-2024");
    list.show_all();
    list.toggle_completed(0);
    list.show_all();
    list.remove(0);
    list.remove(1);
    list.show_all();
}
