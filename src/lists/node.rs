use std::cell::RefCell;
use std::rc::Rc;

pub type NodeRef<T> = Rc<RefCell<Node<T>>>;
pub type NodeOption<T> = Option<NodeRef<T>>;

#[derive(PartialEq, Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: NodeOption<T>,
}

impl<T> Default for Node<T>
where
    T: Default,
{
    fn default() -> Self {
        Node {
            data: T::default(),
            next: None,
        }
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> NodeRef<T> {
        Rc::new(RefCell::new(Node {
            data: value,
            next: None,
        }))
    }
}

pub struct NodeIterator<T> {
    current: NodeOption<T>,
}

impl<T> NodeIterator<T> {
    pub fn new(start: NodeOption<T>) -> Self {
        Self { current: start }
    }
}

impl<T> Iterator for NodeIterator<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> NodeOption<T> {
        let mut result = None;
        self.current = if let Some(ref current) = self.current {
            result = Some(Rc::clone(current));
            if let Some(ref next_node) = current.borrow_mut().next {
                Some(Rc::clone(next_node))
            } else {
                None
            }
        } else {
            None
        };

        result
    }
}

mod tests {
    #![allow(unused_imports)]

    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn new_node() {
        let node = Node::new("Something".to_string());

        assert_eq!(
            node,
            Rc::new(RefCell::new(Node {
                data: "Something".to_string(),
                next: None
            }))
        )
    }

    #[test]
    fn iterator_next() {
        let values = ["Something.", "Something more.", "Something even more."];

        // Array of nodes used to compare with the iterator's output
        let mut nodes = vec![];

        // Fill nodes and an array of nodes
        for idx in 0..values.len() {
            nodes.push(Rc::new(RefCell::new(Node {
                data: values[idx],
                next: None,
            })));

            // If not head, fill 'next' reference of previous node
            if idx != 0 {
                nodes[idx - 1].borrow_mut().next = Some(Rc::clone(&nodes[idx]));
            }
        }

        // Iterate through the nodes and compare with the array
        let node_iterator = NodeIterator::new(Some(Rc::clone(&nodes[0])));
        for (idx, node) in node_iterator.enumerate() {
            assert_eq!(node, nodes[idx]);
        }
    }
}
