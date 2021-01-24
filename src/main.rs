pub mod lists;

use lists::*;

impl<T> Drop for node::Node<T> {
    fn drop(&mut self) {
        println!("Node has been dropped");
    }
}

fn main() {
    let values = [0, 1, 2, 4, 5];

    let mut linked_list = LinkedList::new_empty();

    // Push head
    for val in values.iter() {
        linked_list.push_head(val.to_string());
    }

    println!("Pushed head : {} values", values.len());
    println!("---");

    // Pop head
    for val in values.iter().rev() {
        assert_eq!(linked_list.pop_head().unwrap(), val.to_string());
    }

    println!("---");
    println!("Popped head : {} values", values.len());

    println!("                       ");
    println!("-----------------------");
    println!("-----------------------");
    println!("                       ");

    // Push tail
    for val in values.iter() {
        linked_list.push_tail(val.to_string());
    }

    println!("Pushed tail : {} values", values.len());
    println!("---");

    // Pop tail
    for val in values.iter().rev() {
        assert_eq!(linked_list.pop_tail().unwrap(), val.to_string());
    }

    println!("---");
    println!("Popped tail : {} values", values.len());

    println!("                       ");
    println!("-----------------------");
    println!("-----------------------");
    println!("                       ");

    // Push tail
    for val in values.iter() {
        linked_list.push_tail(val.to_string());
    }

    println!("Pushed {} values", values.len());
    println!("---");

    // Node dropping when clearing
    linked_list.clear();

    println!("---");
    println!("Cleared linked list");
}
