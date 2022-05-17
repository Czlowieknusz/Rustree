use std::cmp;

use queues::*;

#[derive(Debug, Default, PartialEq)]
pub struct Node {
    pub value: i32,
    pub left: Tree,
    pub right: Tree,
}

#[derive(Debug, Default, PartialEq)]
pub struct Tree(pub Option<Box<Node>>);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Successor<'a> {
    LeftNode(&'a Node),
    RightNode(&'a Node),
    None,
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            value,
            left: Tree(None),
            right: Tree(None),
        }
    }
}

impl Tree {
    pub fn add_node(&mut self, value: i32) {
        if self.0.as_ref().unwrap().value == value {
            println!("Value {} already present!", value);
            return;
        }
        let subtree_root_node = if value > self.0.as_ref().unwrap().value {
            &mut self.0.as_mut().unwrap().right
        } else {
            &mut self.0.as_mut().unwrap().left
        };
        match subtree_root_node.0 {
            Some(_) => subtree_root_node.add_node(value),
            None => subtree_root_node.0 = Option::from(Box::new(Node::new(value))),
        }
    }

    pub fn get_depth(&self) -> u32 {
        if self.0.is_none() {
            return 0;
        }

        match (
            self.0.unwrap().as_ref().left.0.as_ref(),
            self.0.unwrap().as_ref().right.0.as_ref(),
        ) {
            (None, None) => 1,
            (Some(_), None) => self.0.unwrap().as_ref().left.get_depth() + 1,
            (None, Some(_)) => self.0.unwrap().as_ref().right.get_depth() + 1,
            (Some(_), Some(_)) => {
                cmp::max(
                    self.0.unwrap().as_ref().left.get_depth() + 1,
                    self.0.unwrap().as_ref().right.get_depth() + 1,
                ) + 1
            }
        }
    }

    fn new(val: i32) -> Tree {
        Tree(Some(Box::new(Node::new(val))))
    }

    pub fn del_node(&mut self, value: i32) -> bool {
        let mut current: *mut Tree = self;

        unsafe {
            while let Some(ref mut node) = (*current).0 {
                if node.value == value {
                    match (node.left.0.as_mut(), node.right.0.as_mut()) {
                        (None, None) => (*current).0 = None,
                        (None, Some(_)) => (*current).0 = node.right.0.take(),
                        (Some(_), None) => (*current).0 = node.left.0.take(),
                        (Some(_), Some(_)) => {
                            (*current).0.as_mut().unwrap().value = node.right.extract_min();
                            return true;
                        }
                    }
                }
                if node.value > value {
                    current = &mut node.left;
                }
                if node.value < value {
                    current = &mut node.right;
                }
            }
        }
        false
    }

    unsafe fn extract_min(&mut self) -> i32 {
        let mut current: *mut Tree = self;

        if self.0.is_none() {
            panic!("Called extract_min on Tree without Node. This should never happen!");
        }

        while (*current).0.as_ref().unwrap().left.0.is_some() {
            current = &mut (*current).0.as_mut().unwrap().left;
        }
        let deleted_node = (*current).0.take().unwrap();
        (*current).0 = deleted_node.right.0;
        deleted_node.value
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
        let mut root = Tree::new(0);
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
            root.right.0.as_ref().unwrap().as_ref()
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
