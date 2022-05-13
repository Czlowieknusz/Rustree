use iced::{button, Alignment, Button, Column, Element, Renderer, Row, Sandbox, Text};
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

        let curr_depth = format!("Current depth is {}", self.root.get_depth());
        view.push(
            Row::new()
                .align_items(Alignment::Center)
                .push(Text::new(curr_depth.to_string()).size(50)),
        )
        .push(print_tree(&self.root))
        .into()
    }
}

fn print_tree(node: &node::Node) -> Column<Message> {
    let mut nodes = vec![Some(node)];
    let mut ret: Column<Message> = Column::new().push(Row::new().padding(15));
    let mut depth = node.get_depth();

    loop {
        for node in nodes.iter() {
            // let mut tree_layer: Row<Message> = Row::new().padding(20);
            match node {
                Some(node) => {
                    ret = ret.push(
                        Column::new()
                            .push(Text::new(node.value.to_string()).size(25))
                            .padding(calc_padding(&depth)),
                    )
                }
                None => {
                    ret = ret.push(
                        Column::new()
                            .push(Text::new("*".to_string()).size(25))
                            .padding(calc_padding(&depth)),
                    )
                }
            }
        }
        depth -= 1;
        if !is_some_in_vec(&nodes) {
            break;
        }
        nodes = get_next_iter_nodes(nodes);
    }
    ret
}

fn calc_padding(depth: &u32) -> u16 {
    let mut paddings = vec![1];
    for i in 0..*depth {
        paddings.push(paddings[i as usize] * 2 + 1);
    }
    paddings.last().copied().unwrap()
}

fn get_next_iter_nodes<'a>(nodes: Vec<Option<&'a node::Node>>) -> Vec<Option<&'a node::Node>> {
    let mut tmp_nodes: Vec<Option<&node::Node>> = vec![];
    for n in nodes.iter() {
        match n {
            Some(n) => {
                match n.left.as_ref() {
                    Some(n) => tmp_nodes.push(Some(&n)),
                    None => tmp_nodes.push(None),
                };
                match n.right.as_ref() {
                    Some(n) => tmp_nodes.push(Some(&n)),
                    None => tmp_nodes.push(None),
                };
            }
            None => {
                tmp_nodes.push(None);
                tmp_nodes.push(None);
            }
        }
    }
    tmp_nodes
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
        let depth = 1;
        assert_eq!(calc_padding(&depth), 3);
    }

    #[test]
    fn padding_for_three() {
        let depth = 3;
        assert_eq!(calc_padding(&depth), 15);
    }

    #[test]
    fn padding_for_five() {
        let depth = 5;
        assert_eq!(calc_padding(&depth), 63);
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
        let mut node = node::Node::new(2);
        node.add_node(1);

        let mut nodes = vec![Option::from(&node)];

        nodes = get_next_iter_nodes(nodes);

        let exp_node = node::Node::new(1);
        let expected_nodes = vec![Option::from(&exp_node), None];
        assert_eq!(nodes, expected_nodes);
    }

    #[test]
    fn should_add_none() {
        let node = None;
        let mut nodes = vec![node];

        nodes = get_next_iter_nodes(nodes);

        let expected_nodes = vec![None, None];
        assert_eq!(nodes, expected_nodes);
    }
}
