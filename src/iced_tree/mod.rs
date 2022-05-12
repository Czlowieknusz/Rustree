use iced::{button, Alignment, Button, Column, Element, Row, Sandbox, Text};
use rand::Rng;

pub mod node;

#[derive(Default)]
pub struct Tree {
    root: Box<node::Node>,
    add_child_btn: button::State,
    del_child_btn: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    AddChild,
    DelChild,
}

impl Sandbox for Tree {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddChild => {
                let mut rng = rand::thread_rng();
                self.root.add_node(rng.gen::<i32>() % 10);
            }
            Message::DelChild => {
                println!("Not implemented yet!");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut view = Column::new().push(
            Row::new()
                .padding(20)
                .align_items(Alignment::Center)
                .push(
                    Button::new(&mut self.add_child_btn, Text::new("AddChild"))
                        .on_press(Message::AddChild),
                )
                .push(
                    Button::new(&mut self.del_child_btn, Text::new("DelChild"))
                        .on_press(Message::DelChild),
                ),
        );

        // Print the tree

        let depth = format!("Current depth is {}", self.root.get_depth());
        view = view.push(
            Row::new()
                .align_items(Alignment::Center)
                .push(Text::new(depth.to_string()).size(50)),
        );
        let printable_nodes = print_tree(&self.root);

        for nodes in printable_nodes {
            let mut tree_layer = Row::new().padding(20);
            for n in nodes {
                match n {
                    Some(n) => tree_layer = tree_layer.push(Text::new(n.to_string()).size(25)),
                    None => tree_layer = tree_layer.push(Text::new("leaf".to_string()).size(25)),
                }
            }
            view = view.push(Column::new().push(tree_layer));
        }
        // Finish printing the tree

        view.into()
    }
}

fn print_tree(node: &node::Node) -> Vec<Vec<Option<i32>>> {
    let mut values = vec![vec![Some(node.value)]];

    let mut nodes = vec![Some(node)];

    // let mut ret_val = Column::new().push(Row::new().padding(15));

    // loop {

    // }

    loop {
        let insert_row = values.len();
        values.push(vec![]);
        let mut tmp_nodes: Vec<Option<&node::Node>> = vec![];
        for node in nodes.iter() {
            match node {
                Some(node) => {
                    fill_values_and_nodes(&node.left, insert_row, &mut values, &mut tmp_nodes);
                    fill_values_and_nodes(&node.right, insert_row, &mut values, &mut tmp_nodes);
                }
                None => {
                    values[insert_row].push(None);
                    values[insert_row].push(None);
                    tmp_nodes.push(None);
                    tmp_nodes.push(None);
                }
            }
        }

        if !is_some_in_vec(&tmp_nodes) {
            break;
        }
        nodes = tmp_nodes;
    }

    values
}

fn calc_padding(n: &node::Node) -> i32 {
    let mut paddings = vec![1];
    let d = n.get_depth() as usize;
    for i in 0..d {
        paddings.push(paddings[i] * 2 + 1);
    }
    paddings.last().copied().unwrap()
}

fn fill_values_and_nodes<'a>(
    node: &'a Option<Box<node::Node>>,
    insert_row: usize,
    values: &mut Vec<Vec<Option<i32>>>,
    nodes: &mut Vec<Option<&'a node::Node>>,
) {
    match node.as_ref() {
        Some(n) => {
            values[insert_row].push(Some(n.value));
            nodes.push(Some(&n));
        }
        None => {
            values[insert_row].push(None);
            nodes.push(None);
        }
    };
}

fn is_some_in_vec(v: &Vec<Option<&node::Node>>) -> bool {
    v.iter().any(|&n| match n {
        Some(_) => true,
        None => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding_for_one() {
        let n = node::Node::new(1);
        assert_eq!(calc_padding(&n), 3);
    }

    #[test]
    fn padding_for_three() {
        let mut n = node::Node::new(1);
        n.add_node(2);
        n.add_node(3);
        assert_eq!(calc_padding(&n), 15);
    }

    #[test]
    fn padding_for_five() {
        let mut n = node::Node::new(1);
        n.add_node(2);
        n.add_node(3);
        n.add_node(4);
        n.add_node(5);
        assert_eq!(calc_padding(&n), 63);
    }

    #[test]
    fn single_node_tree() {
        let root = Box::new(node::Node::new(1));

        let right = vec![vec![Some(1)], vec![None, None]];
        assert_eq!(print_tree(&root), right);
    }

    #[test]
    fn multiple_nodes_tree() {
        let mut root = Box::new(node::Node::new(3));
        root.add_node(1);
        root.add_node(2);
        root.add_node(5);

        let right = vec![
            vec![Some(3)],
            vec![Some(1), Some(5)],
            vec![None, Some(2), None, None],
            vec![None, None, None, None, None, None, None, None],
        ];
        assert_eq!(print_tree(&root), right);
    }

    #[test]
    fn one_some() {
        let n = node::Node::new(3);
        let v = vec![None, None, Some(&n)];
        assert_eq!(is_some_in_vec(&v), true);
    }

    #[test]
    fn no_some() {
        let v = vec![None, None, None, None];

        assert_eq!(is_some_in_vec(&v), false);
    }

    #[test]
    fn all_some() {
        let n1 = node::Node::new(2);
        let n2 = node::Node::new(3);
        let v = vec![Some(&n1), Some(&n2)];

        assert_eq!(is_some_in_vec(&v), true);
    }

    #[test]
    fn should_add_some() {
        let node = Option::from(Box::new(node::Node::new(3)));
        let insert_row = 0;
        let mut values = vec![vec![]];
        let mut nodes = vec![];

        fill_values_and_nodes(&node, insert_row, &mut values, &mut nodes);

        let expected_values = vec![vec![Some(3)]];
        let tmp = node.as_ref().unwrap().as_ref();
        let expected_nodes = vec![Some(tmp)];
        assert_eq!(values, expected_values);
        assert_eq!(nodes, expected_nodes);
    }

    #[test]
    fn should_add_none() {
        let node = None;
        let insert_row = 0;
        let mut values = vec![vec![]];
        let mut nodes = vec![];

        fill_values_and_nodes(&node, insert_row, &mut values, &mut nodes);

        let expected_values = vec![vec![None]];
        let expected_nodes = vec![None];
        assert_eq!(values, expected_values);
        assert_eq!(nodes, expected_nodes);
    }
}
