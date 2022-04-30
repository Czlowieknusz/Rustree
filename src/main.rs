use std::cmp;

#[derive(Debug)]
struct Node {
    pub value: i32,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

impl Node {
    fn get_depth(&self) -> u32 {
        match (
            self.left_child.as_ref(),
            self.right_child.as_ref(),
        ) {
            (None, None) => 1,
            (Some(left), None) => left.get_depth() + 1,
            (None, Some(right)) => right.get_depth() + 1,
            (Some(left), Some(right)) => cmp::max(left.get_depth(), right.get_depth()) + 1,
        }
    }

    fn add_node(&mut self, value: i32) {
        // if self.value == value {
        //     println!("Value already present!");
        //     return;
        // }
        // // match (&mut self.left_child, &mut self.right_child) {
        // //     (Some(left), Some(right)) => {
        // //
        // //     },
        // //     (None, _) =>,
        // //     (Some(_), None) =>,
        // // }
        // let parent_node = if value > self.value { &mut self.right_child} else {&mut self.left_child};
        // match parent_node. {
        //     Some (&mut node) => node.
        // //     (Some(mut left), Some(_)) => left.add_node(value),
        //     /*Rc::get_mut(left).unwrap().add_node(value)*/
        //     //
        //     // )left.clone().add_node(value),
        //     // (None, None) => self.left_child.borrow_mut().as_deref_mut() = Option::from(Box::new(Node::new(value))),
        //     // (Some(_), _) => self.right_child.borrow_mut().as_deref_mut() = Option::from(Box::new(Node::new(value))),
        //     // _ => {}
        // }
    }

    fn new(value: i32)  -> Node {
        Node {
            value,
            left_child: None,
            right_child: None,
        }
    }
}

fn main() {
    let mut node = Node {
        value: 3,
        left_child: None,
        right_child: None,
    };
    println!("Before adding Node is {:?} and it's depth is {}.", node, node.get_depth());
    node.add_node(5);
    println!("After adding Node is {:?} and it's depth is {}.", node, node.get_depth());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_node_tree() {
        let parent = Box::new(Node {
            value: 1,
            left_child: None,
            right_child: None,
        });

        assert_eq!(parent.get_depth(), 1);
    }

    #[test]
    fn should_choose_larger_branch() {
        let mut parent = Box::new(Node {
            value: 5,
            left_child: None,
            right_child: None,
        });
        parent.add_node(3);
        parent.add_node(2);
        parent.add_node(1);
        parent.add_node(6);

        assert_eq!(parent.get_depth(), 4);
    }

    #[test]
    fn should_not_use_first_node_depth() {
        let mut parent = Box::new(Node {
            value: 3,
            left_child: None,
            right_child: None,
        });
        parent.add_node(5);
        parent.add_node(4);
        parent.add_node(6);
        parent.add_node(1);

        assert_eq!(parent.get_depth(), 3);
    }

    // #[test]
    // fn add_left_then_right_node() {
    //     let mut parent = Node {
    //         value: 1,
    //         left_child: None,
    //         right_child: None,
    //     };
    //
    //     parent.add_node(3);
    //     parent.add_node(2);
    //
    //     assert_eq!(parent.left_child.as_ref().unwrap().value, 3);
    //     assert_eq!(parent.right_child.as_ref().unwrap().value, 2);
    // }
}
