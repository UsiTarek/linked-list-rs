pub mod node;

use node::*;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct LinkedList<T> {
    head: NodeOption<T>,
    tail: NodeOption<T>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new_empty() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn new(head_val: T) -> Self {
        LinkedList {
            head: Some(Node::<T>::new(head_val)),
            tail: None,
            length: 1,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn push_head(&mut self, val: T) {
        let new_head = Node::new(val);

        if let Some(old_head) = self.head.take() {
            new_head.borrow_mut().next = Some(Rc::clone(&old_head));

            if self.tail.is_none() {
                self.tail = Some(Rc::clone(&old_head));
            }
        }

        self.head = Some(new_head);
        self.length += 1;
    }

    pub fn push_tail(&mut self, val: T) {
        if let Some(head) = &self.head {
            let new_tail = Node::new(val);

            if let Some(old_tail) = self.tail.take() {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
            } else {
                head.borrow_mut().next = Some(Rc::clone(&new_tail));
            }

            self.tail = Some(new_tail);
            self.length += 1;
        } else {
            self.push_head(val);
        }
    }

    pub fn pop_head(&mut self) -> Option<T>
    where
        T: Clone,
    {
        self.head.take().map(|old_head| {
            if let Some(new_head) = old_head.borrow_mut().next.take() {
                self.head = Some(Rc::clone(&new_head));
            }

            if self.length() == 1 {
                self.tail = None;
            }

            self.length -= 1;
            old_head.borrow().data.clone()
        })
    }

    pub fn pop_tail(&mut self) -> Option<T>
    where
        T: Clone,
    {
        self.tail.take().map(|popped_tail| {
            let mut last_node = None;

            if self.length() == 1 {
                self.head.take();
            } else {
                for (idx, node) in self.iter().enumerate() {
                    if idx == self.length - 2 {
                        last_node = Some(Rc::clone(&node));
                        break;
                    }
                }

                let node = last_node.unwrap();
                node.borrow_mut().next = None;
                self.tail = Some(Rc::clone(&node));
            }

            self.length -= 1;
            popped_tail.borrow_mut().data.clone()
        })
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.length = 0;
    }

    pub fn iter(&self) -> NodeIterator<T> {
        if let Some(ref head) = self.head {
            NodeIterator::<T>::new(Some(Rc::clone(head)))
        } else {
            NodeIterator::<T>::new(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_push_head() {
        let values = ["Something.", "Something more.", "Something even more."];
        assert!(values.len() >= 3); // Guarantees a middle node

        let mut linked_list = LinkedList::new_empty();
        for idx in 0..values.len() {
            linked_list.push_head(values[idx]);
        }

        for (idx, node) in linked_list.iter().enumerate() {
            let current_node = node.borrow();

            assert_eq!(current_node.data, values[values.len() - 1 - idx]);

            // Testing 'next' values
            if idx != values.len() - 1 {
                let next_node = current_node.next.as_ref().unwrap().borrow();
                assert_eq!(next_node.data, values[values.len() - 2 - idx])
            }
        }

        assert_eq!(values.len(), linked_list.length());
    }

    #[test]
    fn linked_list_push_tail() {
        let values = ["Something.", "Something more.", "Something even more."];
        assert!(values.len() >= 3, 0); // Guarantees a middle node

        let mut linked_list = LinkedList::new_empty();
        for idx in 0..values.len() {
            linked_list.push_tail(values[idx]);
        }

        for (idx, node) in linked_list.iter().enumerate() {
            let current_node = node.borrow();
            assert_eq!(current_node.data, values[idx]);

            // Testing 'next' values
            if idx != values.len() - 1 {
                let next_node = current_node.next.as_ref().unwrap().borrow();
                assert_eq!(next_node.data, values[idx + 1])
            }
        }

        assert_eq!(values.len(), linked_list.length());
    }

    #[test]
    fn linked_list_pop_head() {
        let values = ["Something.", "Something more.", "Something even more."];

        let mut linked_list = LinkedList::new_empty();
        for idx in 0..values.len() {
            linked_list.push_tail(values[idx]);
        }

        // Popping the head until linked list is free
        for idx in 0..linked_list.length() {
            let popped_head = linked_list.pop_head();
            assert_eq!(popped_head.unwrap(), values[idx]);

            for (node_idx, node) in linked_list.iter().enumerate() {
                assert_eq!(node.borrow().data, values[idx + node_idx + 1])
            }

            assert_eq!(linked_list.length(), values.len() - 1 - idx)
        }

        assert_eq!(linked_list, LinkedList::new_empty());
    }

    #[test]
    fn linked_list_pop_tail() {
        let values = ["Something.", "Something more.", "Something even more."];

        let mut linked_list = LinkedList::new_empty();
        for idx in 0..values.len() {
            linked_list.push_tail(values[idx]);
        }

        //Popping the tail until linked list is free
        for idx in 0..linked_list.length() {
            let popped_tail = linked_list.pop_tail();
            assert_eq!(popped_tail.unwrap(), values[values.len() - 1 - idx]);

            for (node_idx, node) in linked_list.iter().enumerate() {
                assert_eq!(node.borrow().data, values[node_idx])
            }

            assert_eq!(linked_list.length(), values.len() - 1 - idx)
        }

        assert_eq!(linked_list, LinkedList::new_empty());
    }
}
