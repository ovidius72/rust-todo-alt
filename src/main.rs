mod todo_item;
mod todo_list;

use crate::todo_list::TodoList;

fn main() {
    let mut list = TodoList::new();
    list.add("Something", "");
    list.add("Another task", "10-10-2024");
    list.add("Another task2", "10-10-2024");
    list.show_all();
    list.set_completed(0);
    list.toggle_completed(0);
    list.show_all();
    list.remove(0);
    list.remove(1);
    list.show_all();
}
