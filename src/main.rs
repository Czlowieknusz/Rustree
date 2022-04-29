use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn get_depth(&self) -> u32 {
        if self.children.borrow().is_empty() {
            return 1
        }
        let mut depths = Vec::new();
        for child in self.children.borrow().iter() {
            depths.push(child.get_depth() + 1);
        }
        match depths.iter().max() {
            Some(max) => *max,
            None => 0
        }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_node_tree() {
        let parent = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        assert_eq!(parent.get_depth(), 1);
    }

    #[test]
    fn should_choose_larger_branch() {
        let even_younger_child = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        let younger_child = Rc::new(Node {
            value: 4,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&even_younger_child)]),
        });
        let older_child_1 = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&younger_child)]),
        });
        let older_child_2 = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        let parent = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&older_child_1), Rc::clone(&older_child_2)]),
        });

        assert_eq!(parent.get_depth(), 4);
    }

    #[test]
    fn should_not_use_first_node_depth() {
        let younger_child = Rc::new(Node {
            value: 4,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        let older_child_1 = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        let older_child_2 = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&younger_child)]),
        });
        let parent = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&older_child_1), Rc::clone(&older_child_2)]),
        });

        assert_eq!(parent.get_depth(), 3);
    }
}