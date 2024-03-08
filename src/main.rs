use std::str::FromStr;

use chrono::{Datelike, NaiveDate, Timelike, Utc};

struct TodoItem {
    description: String,
    completed: bool,
    created_at: String,
    completed_at: Option<String>,
    expires_at: Option<NaiveDate>,
}

impl TodoItem {
    fn new(description: String, expiry_date: Option<String>) -> TodoItem {
        let now = Utc::now();
        println!("now {}", now);
        TodoItem {
            description,
            completed: false,
            created_at: String::from("2020-10-10"),
            completed_at: None,
            expires_at: if expiry_date.is_some() {
                Some(NaiveDate::parse_from_str(&expiry_date.unwrap(), "%d-%m-%Y").unwrap())
            } else {
                None
            },
        }
    }

    fn toggle_completed(&mut self) {
        self.set_completed(!self.completed);
    }

    fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
        self.completed_at = if completed {
            Some(String::from("2020-10-10"))
        } else {
            None
        }
    }
}

struct TodoList {
    items: Vec<TodoItem>,
}
impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }
    fn add(&mut self, description: String, expiry_date: String) -> &mut Self {
        let exp_date = if !expiry_date.is_empty() {
            Some(expiry_date)
        } else {
            None
        };
        self.items.push(TodoItem::new(description, exp_date));
        self
    }
    fn remove(&mut self, index: usize) -> &mut Self {
        self.items.remove(index);
        self
    }
    fn complete(&mut self, index: usize) -> &mut Self {
        self.items[index].set_completed(true);
        self
    }

    fn toggle_completed(&mut self, index: usize) -> &mut Self {
        self.items[index].toggle_completed();
        self
    }
    fn show_all(&mut self) {
        self.items.iter().for_each(|item| {
            let completed = if item.completed == true {
                "Completed"
            } else {
                "Pending"
            };
            println!("{} {}", item.description, completed);
        });
    }
}

fn main() {
    let mut list = TodoList::new();
    list.add("Something".to_string(), "".to_string());
    list.add("Another task".to_string(), "10-10-2024".to_string());
    list.show_all();
    list.toggle_completed(0);
    list.show_all();
}
