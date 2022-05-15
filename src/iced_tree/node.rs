use std::cmp;

use queues::*;

#[derive(Debug, Default, PartialEq)]
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Successor<'a> {
    LeftNode(&'a Node),
    RightNode(&'a Node),
    None,
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

    pub fn del_node(&mut self, value: i32) /* -> Option<&Node> */
    {
        // find node for deletion
        let mut n_for_del = self.find_node(value);
        if n_for_del.is_none() {
            println!("Node with {} does not exist", value);
            return;
        }
        let succ_parent = n_for_del.unwrap().find_successor_parent();
    }

    fn find_successor_parent(&self) -> Successor {
        if self.right.is_none() {
            if self.left.is_none() {
                return Successor::None; // single node tree
            }
            return Successor::LeftNode(self); // replece deleted node with left node
        }
        if self.left.is_none() {
            return Successor::RightNode(self); // replece deleted node with right node
        }
        if self.right.as_ref().unwrap().as_ref().left.is_none() {
            return Successor::RightNode(self.right.as_ref().unwrap());
        }
        let mut buff = Buffer::new(1);
        buff.add(&self.right).ok();
        loop {
            let succ = buff.remove().unwrap().as_ref().unwrap().as_ref();
            let left: &Option<Box<Node>> = &succ.left;
            if left.as_ref().unwrap().left.is_none() {
                return Successor::LeftNode(succ);
            }
            buff.add(left).ok();
        }
    }

    pub fn find_node(&self, value: i32) -> Option<&Node> {
        let mut nodes: Queue<Option<&Node>> = queue![];
        nodes.add(Some(self)).ok()?;
        while nodes.size() > 0 {
            if let Some(&n) = nodes.remove().unwrap().as_ref() {
                if n.value == value {
                    return Some(n);
                }
                if let Some(n) = n.left.as_ref() {
                    nodes.add(Some(n)).ok()?;
                }
                if let Some(n) = n.right.as_ref() {
                    nodes.add(Some(n)).ok()?;
                }
            }
        }
        None
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

    #[test]
    fn find_value_in_single_element_tree() {
        let root = Box::new(Node::new(3));
        assert_eq!(root.find_node(3).unwrap(), root.as_ref());
    }

    #[test]
    fn find_value_in_multi_elements_tree() {
        let mut root = Box::new(Node::new(5));
        for i in 0..9 {
            root.add_node(i);
        }
        assert_eq!(root.get_depth(), 6);
        assert_eq!(
            root.find_node(6).unwrap(),
            root.right.as_ref().unwrap().as_ref()
        );
    }

    #[test]
    fn return_none_if_value_not_present() {
        let mut root = Box::new(Node::new(5));
        for i in 0..9 {
            root.add_node(i);
        }
        assert_eq!(root.find_node(12), None);
    }

    #[test]
    fn find_none_in_single_node_tree() {
        let root = Box::new(Node::new(5));
        assert_eq!(root.find_successor_parent(), Successor::None);
    }

    #[test]
    fn find_successor_parent_two_subtrees() {
        let root = prepare_tree();
        if let Successor::LeftNode(succ_parent) = root.find_successor_parent() {
            assert_eq!(succ_parent.value, 7);
        } else {
            panic!("Wrong node chosen as successor!");
        }
    }

    #[test]
    fn find_successor_parent_with_only_left_subtree() {
        let mut root = Box::new(Node::new(5));
        root.add_node(3);
        if let Successor::LeftNode(succ_parent) = root.find_successor_parent() {
            assert_eq!(succ_parent.value, 5);
        } else {
            panic!("Wrong node chosen as successor!");
        }
    }

    #[test]
    fn right_node_has_no_left_child() {
        let mut root = Box::new(Node::new(5));
        root.add_node(6);
        if let Successor::RightNode(succ_parent) = root.find_successor_parent() {
            assert_eq!(succ_parent.value, 5);
        } else {
            panic!("Wrong node chosen as successor!");
        }
    }

    fn prepare_tree() -> Box<Node> {
        /*
                5
             /     \
           2        7
         /  \      /  \
        0    4    6    8
        \   /           \
         1 3             9
        */
        let mut root = Box::new(Node::new(5));
        // left
        root.add_node(2);
        root.add_node(0);
        root.add_node(4);
        root.add_node(1);
        root.add_node(3);
        // right
        root.add_node(7);
        root.add_node(6);
        root.add_node(8);
        root.add_node(9);
        root
    }
}
