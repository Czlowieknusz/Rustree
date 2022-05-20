use std::cmp;

#[derive(Debug, Default, PartialEq)]
pub struct Node {
    pub value: i32,
    pub left: Tree,
    pub right: Tree,
}

#[derive(Debug, Default, PartialEq)]
pub struct Tree(pub Option<Box<Node>>);

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
        if self.0.is_none() {
            self.0 = Option::from(Box::new(Node::new(value)))
        }
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
            self.0.as_ref().unwrap().left.0.as_ref(),
            self.0.as_ref().unwrap().right.0.as_ref(),
        ) {
            (None, None) => 1,
            (Some(_), None) => self.0.as_ref().unwrap().left.get_depth() + 1,
            (None, Some(_)) => self.0.as_ref().unwrap().right.get_depth() + 1,
            (Some(_), Some(_)) => {
                cmp::max(
                    self.0.as_ref().unwrap().left.get_depth(),
                    self.0.as_ref().unwrap().right.get_depth(),
                ) + 1
            }
        }
    }

    pub fn new(val: i32) -> Tree {
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
                } else if node.value > value {
                    current = &mut node.left;
                } else if node.value < value {
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
        let root = Tree::new(1);

        assert_eq!(root.get_depth(), 1);
    }

    #[test]
    fn left_depth_if_bigger() {
        let mut root = Tree::new(5);
        root.add_node(3);
        root.add_node(2);
        root.add_node(1);
        root.add_node(6);

        assert_eq!(root.get_depth(), 4);
    }

    #[test]
    fn right_depth_if_bigger() {
        let mut root = Tree::new(3);
        root.add_node(5);
        root.add_node(4);
        root.add_node(6);
        root.add_node(1);

        assert_eq!(root.get_depth(), 3);
    }

    #[test]
    fn depth_is_one_hundred() {
        let mut root = Tree::new(0);
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
        let mut root = Tree::new(0);
        assert_eq!(root.get_depth(), 1);

        root.del_node(0);
        assert_eq!(root.get_depth(), 0);
    }

    #[test]
    fn extract_node_from_complex_tree() {
        let mut root = prepare_tree();
        unsafe {
            assert_eq!(root.0.as_mut().unwrap().right.extract_min(), 6);
        }
    }

    #[test]
    fn extract_left_node() {
        let mut root = Tree::new(5);
        root.add_node(3);

        unsafe {
            assert_eq!(root.extract_min(), 3);
        }
    }

    fn prepare_tree() -> Tree {
        /*
                5
             /     \
           2        7
         /  \      /  \
        0    4    6    8
        \   /           \
         1 3             9
        */
        let mut root = Tree::new(5);
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
