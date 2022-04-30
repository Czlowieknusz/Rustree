use std::cell::RefCell;
use std::cmp;
use std::rc::{Rc, Weak};

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    pub value: i32,
    parent: RefCell<Weak<Node>>,
    left_child: RefCell<Option<Rc<Node>>>,
    right_child: RefCell<Option<Rc<Node>>>,
}

impl Node {
    fn get_depth(&self) -> u32 {
        match (
            self.left_child.borrow().as_ref(),
            self.right_child.borrow().as_ref(),
        ) {
            (None, None) => 1,
            (Some(left), None) => left.get_depth() + 1,
            (None, Some(right)) => right.get_depth() + 1,
            (Some(left), Some(right)) => cmp::max(left.get_depth(), right.get_depth()) + 1,
        }
    }

    fn add_node(&self, value: i32) {
        // let new_node = Rc::new(Node {
        //     value: 5,
        //     parent: RefCell::new(Weak::new()),
        //     left_child: RefCell::new(None),
        //     right_child: RefCell::new(None),
        // });
        //
        // parent.
        // let younger_child = Rc::new(Node {
        //     value: 4,
        //     parent: RefCell::new(Weak::new()),
        //     left_child: RefCell::new(Option::from(Rc::clone(&even_younger_child))),
        //     right_child: RefCell::new(None),
        // });
        //
    }
}

fn main() {
    let node = Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        left_child: RefCell::new(None),
        right_child: RefCell::new(None),
    };
    node.add_node(5);
    println!("Node is {:?} and it's depth is {}.", node, node.get_depth());
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use super::*;

    #[test]
    fn single_node_tree() {
        let parent = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(None),
            right_child: RefCell::new(None),
        });

        assert_eq!(parent.get_depth(), 1);
    }

    #[test]
    fn should_choose_larger_branch() {
        let even_younger_child = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(None),
            right_child: RefCell::new(None),
        });
        let younger_child = Rc::new(Node {
            value: 4,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(Option::from(Rc::clone(&even_younger_child))),
            right_child: RefCell::new(None),
        });
        let older_child_1 = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(None),
            right_child: RefCell::new(Option::from(Rc::clone(&younger_child))),
        });
        let older_child_2 = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(None),
            right_child: RefCell::new(None),
        });
        let parent = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(Option::from(Rc::clone(&older_child_1))),
            right_child: RefCell::new(Option::from(Rc::clone(&older_child_2))),
        });

        assert_eq!(parent.get_depth(), 4);
        assert_eq!(older_child_2.get_depth(), 1);
        assert_eq!(older_child_1.get_depth(), 3);
        assert_eq!(younger_child.get_depth(), 2);
        assert_eq!(even_younger_child.get_depth(), 1);
    }

    #[test]
    fn should_not_use_first_node_depth() {
        let younger_child = Rc::new(Node {
            value: 4,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(None),
            right_child: RefCell::new(None),
        });
        let older_child_1 = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(None),
            right_child: RefCell::new(None),
        });
        let older_child_2 = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(None),
            right_child: RefCell::new(Option::from(Rc::clone(&older_child_1))),
        });
        let parent = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(Option::from(Rc::clone(&older_child_1))),
            right_child: RefCell::new(Option::from(Rc::clone(&older_child_2))),
        });

        assert_eq!(parent.get_depth(), 3);
        assert_eq!(older_child_2.get_depth(), 2);
        assert_eq!(older_child_1.get_depth(), 1);
        assert_eq!(younger_child.get_depth(), 1);
    }

    #[test]
    fn add_left_then_right_node() {
        let parent = Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            left_child: RefCell::new(None),
            right_child: RefCell::new(None),
        };

        parent.add_node(3);
        parent.add_node(2);

        assert_eq!(parent.left_child.into_inner().unwrap().as_ref().value, 3);
        assert_eq!(parent.right_child.into_inner().unwrap().as_ref().value, 2);
    }
}
