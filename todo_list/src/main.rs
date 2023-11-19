mod list;
use crate::list::List;

// add a basic interface and done!

fn main() {
    let mut list = List::new();

    list.push("First Task".to_string());
    list.push("Second Task".to_string());
    list.push("3 Task".to_string());

    list.remove("Second Task".to_string());

    list.status_toggle("First Task".to_string());
    
    list.print();
}
