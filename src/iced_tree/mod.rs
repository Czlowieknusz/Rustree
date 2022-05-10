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
                self.root.add_node(rng.gen());
            }
            Message::DelChild => {
                println!("Not implemented yet!");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut view = Column::new()
            .push(
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
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .push(Text::new(self.root.get_depth().to_string()).size(50)),
            );

        // Print the tree

        view = view.push(
            Row::new()
                .align_items(Alignment::Center)
                .push(Text::new(self.root.value.to_string()).size(50)),
        );
        print_tree(&self.root);
        // Finish printing the tree

        view.into()
    }
}

fn print_tree(node: &node::Node) -> Vec<Vec<Option<i32>>> {
    let mut values = vec![vec![Some(node.value)]];

    let mut nodes = vec![Some(node)];

    loop {
        let last_depth = values.len();
        values.push(vec![]);
        let mut tmp_nodes: Vec<Option<&node::Node>> = vec![];
        for node in nodes.iter() {
            match node {
                Some(node) => {
                    match node.left.as_ref() {
                        Some(n) => {
                            values[last_depth].push(Some(n.value));
                            tmp_nodes.push(Some(n.as_ref()));
                        }
                        None => {
                            values[last_depth].push(None);
                            tmp_nodes.push(None);
                        }
                    }
                    match node.right.as_ref() {
                        Some(n) => {
                            values[last_depth].push(Some(n.value));
                            tmp_nodes.push(Some(&n));
                        }
                        None => {
                            values[last_depth].push(None);
                            tmp_nodes.push(None);
                        }
                    }
                }
                None => {
                    values[last_depth].push(None);
                    values[last_depth].push(None);
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

// fn fill

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
    fn single_node_tree() {
        let root = Box::new(node::Node::new(1));

        let left = print_tree(&root);
        let right = vec![vec![Some(1)]];
        assert_eq!(left, right);
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
        let n1 = node::Node::new(3);
        let n2 = node::Node::new(3);
        let v = vec![Some(&n1), Some(&n2)];

        assert_eq!(is_some_in_vec(&v), true);
    }
}
