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
        if self.value == value {
            println!("Value already present!");
            return;
        }
        let parent_node = if value > self.value { &mut self.right_child} else {&mut self.left_child};
        match parent_node {
            &mut Some (ref mut node) => node.add_node(value),
            &mut None => *parent_node = Option::from(Box::new(Node::new(value))),
        }
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
    fn left_depth_if_bigger() {
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
    fn right_depth_if_bigger() {
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

    #[test]
    fn depth_is_one_hundred() {
        let mut parent = Box::new(Node {
            value: 0,
            left_child: None,
            right_child: None,
        });
        for i in 1..100 {
            parent.add_node(i);
        }

        assert_eq!(parent.get_depth(), 100);
    }
}
