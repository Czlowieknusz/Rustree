use std::cmp;

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn get_depth(&self) -> u32 {
        match (self.left.as_ref(), self.right.as_ref()) {
            (None, None) => 1,
            (Some(left), None) => left.get_depth() + 1,
            (None, Some(right)) => right.get_depth() + 1,
            (Some(left), Some(right)) => cmp::max(left.get_depth(), right.get_depth()) + 1,
        }
    }

    pub fn add_node(&mut self, value: i32) {
        if self.value == value {
            println!("Value already present!");
            return;
        }
        let subtree_root_node = if value > self.value {
            &mut self.right
        } else {
            &mut self.left
        };
        match subtree_root_node {
            &mut Some(ref mut subtree_root_node) => subtree_root_node.add_node(value),
            &mut None => *subtree_root_node = Option::from(Box::new(Node::new(value))),
        }
    }

    pub fn new(value: i32) -> Node {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_node_tree() {
        let root = Box::new(Node {
            value: 1,
            left: None,
            right: None,
        });

        assert_eq!(root.get_depth(), 1);
    }

    #[test]
    fn left_depth_if_bigger() {
        let mut root = Box::new(Node {
            value: 5,
            left: None,
            right: None,
        });
        root.add_node(3);
        root.add_node(2);
        root.add_node(1);
        root.add_node(6);

        assert_eq!(root.get_depth(), 4);
    }

    #[test]
    fn right_depth_if_bigger() {
        let mut root = Box::new(Node {
            value: 3,
            left: None,
            right: None,
        });
        root.add_node(5);
        root.add_node(4);
        root.add_node(6);
        root.add_node(1);

        assert_eq!(root.get_depth(), 3);
    }

    #[test]
    fn depth_is_one_hundred() {
        let mut root = Box::new(Node {
            value: 0,
            left: None,
            right: None,
        });
        for i in 1..100 {
            root.add_node(i);
        }

        assert_eq!(root.get_depth(), 100);
    }
}
