use std::cmp;

#[derive(Debug, Default, PartialEq)]
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
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
            println!("Value {} already present!", value);
            return;
        }
        let subtree_root_node = if value > self.value {
            &mut self.right
        } else {
            &mut self.left
        };
        match *subtree_root_node {
            Some(ref mut subtree_root_node) => subtree_root_node.add_node(value),
            None => *subtree_root_node = Option::from(Box::new(Node::new(value))),
        }
    }

    pub fn del_node(&mut self, _value: i32) {}

    fn balance_tree() {}

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
        let root = Box::new(Node::new(1));

        assert_eq!(root.get_depth(), 1);
    }

    #[test]
    fn left_depth_if_bigger() {
        let mut root = Box::new(Node::new(5));
        root.add_node(3);
        root.add_node(2);
        root.add_node(1);
        root.add_node(6);

        assert_eq!(root.get_depth(), 4);
    }

    #[test]
    fn right_depth_if_bigger() {
        let mut root = Box::new(Node::new(3));
        root.add_node(5);
        root.add_node(4);
        root.add_node(6);
        root.add_node(1);

        assert_eq!(root.get_depth(), 3);
    }

    #[test]
    fn depth_is_one_hundred() {
        let mut root = Box::new(Node::new(0));
        for i in 1..100 {
            root.add_node(i);
        }

        assert_eq!(root.get_depth(), 100);
    }

    #[test]
    fn tree_should_connect_on_middle_node_deleted() {
        let mut root = Box::new(Node::new(0));
        root.add_node(1);
        root.add_node(2);
        assert_eq!(root.get_depth(), 3);

        root.del_node(1);
        assert_eq!(root.get_depth(), 2);
    }

    #[test]
    fn should_delete_root() {
        let mut root = Box::new(Node::new(0));
        assert_eq!(root.get_depth(), 1);

        root.del_node(0);
        assert_eq!(root.get_depth(), 0);
    }
}
