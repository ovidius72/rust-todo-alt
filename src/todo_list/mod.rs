use uuid::Uuid;

use crate::todo_item::{optional_date_to_str, TodoItem};

pub struct TodoList {
    pub items: Vec<TodoItem>,
}
impl TodoList {
    pub fn new() -> TodoList {
        TodoList { items: Vec::new() }
    }
    pub fn add(&mut self, description: &str, expiry_date: &str) -> &mut Self {
        let exp_date = if !expiry_date.is_empty() {
            Some(expiry_date.to_owned())
        } else {
            None
        };
        self.items
            .push(TodoItem::new(description.to_owned(), exp_date));
        self
    }
    pub fn remove(&mut self, index: usize) -> &mut Self {
        if self.items.len() >= index {
            self.items.remove(index);
        }
        self
    }
    pub fn set_completed(&mut self, index: usize) -> &mut Self {
        self.items[index].set_completed(true);
        self
    }

    pub fn toggle_completed(&mut self, index: usize) -> &mut Self {
        self.items[index].toggle_completed();
        self
    }
    pub fn toggle_completed_by_id(&mut self, id: Uuid) {
        let item = self.items.iter_mut().find(|item| item.id == id);
        // if item.is_some() {
        //     item.unwrap().toggle_completed();
        // }
        match item {
            Some(i) => {
                i.toggle_completed();
            }
            None => (),
        }
    }
    pub fn show_all(&mut self) {
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
