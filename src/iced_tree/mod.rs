use iced::{button, Alignment, Button, Column, Element, Row, Sandbox, Text};
use rand::Rng;

pub mod node;

#[derive(Default)]
pub struct Tree {
    root: Box<node::Tree>,
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
                let val = rng.gen::<i32>() % 10;
                println!("Trying to add {}.", val);
                self.root.add_node(val);
            }
            Message::DelChild => {
                let mut rng = rand::thread_rng();
                let val = rng.gen::<i32>() % 10;
                println!("Trying to delete {}.", val);
                // self.root.del_node(val);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let curr_depth = format!("Current depth is {}", self.root.get_depth());
        Column::new()
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
                    .push(Text::new(curr_depth).size(50)),
            )
            .push(print_tree(self.root.as_ref()))
            .into()
    }
}

fn print_tree(node: &node::Tree) -> Column<Message> {
    let mut nodes = vec![Some(node)];
    let mut ret: Column<Message> = Column::new().push(Row::new().padding(15));
    let mut depth = node.get_depth();
    let dst_coeficient = 3;

    loop {
        let mut tree_layer: Row<Message> = Row::new();
        for node in nodes.iter() {
            let padding = calc_padding(&depth) * dst_coeficient;
            match node {
                Some(node) => {
                    tree_layer = tree_layer.push(
                        Column::new()
                            .push(Text::new(node.0.as_ref().unwrap().value.to_string()).size(25))
                            .padding(padding)
                            .align_items(Alignment::Center),
                    )
                }
                None => {
                    tree_layer = tree_layer.push(
                        Column::new()
                            .push(Text::new("*".to_string()).size(25))
                            .padding(padding)
                            .align_items(Alignment::Center),
                    )
                }
            }
        }
        ret = ret.push(tree_layer);
        if !is_some_in_vec(&nodes) {
            break;
        }
        depth -= 1;
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

fn get_next_iter_nodes(nodes: Vec<Option<&node::Tree>>) -> Vec<Option<&node::Tree>> {
    let mut tmp_nodes: Vec<Option<&node::Tree>> = vec![];
    for n in nodes.iter() {
        match n {
            Some(n) => {
                match n.0.as_ref().unwrap().left.0 {
                    Some(_) => tmp_nodes.push(Some(&n.0.as_ref().unwrap().left)),
                    None => tmp_nodes.push(None),
                };
                match n.0.as_ref().unwrap().right.0 {
                    Some(_) => tmp_nodes.push(Some(&n.0.as_ref().unwrap().right)),
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

fn is_some_in_vec(v: &[Option<&node::Tree>]) -> bool {
    v.iter().any(|&n| n.is_some())
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
    fn one_some() {
        let n = node::Tree::new(3);
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
        let n1 = node::Tree::new(2);
        let n2 = node::Tree::new(3);
        let v = vec![Some(&n1), Some(&n2)];

        assert_eq!(is_some_in_vec(&v), true);
    }

    #[test]
    fn should_add_some() {
        let mut node = node::Tree::new(2);
        node.add_node(1);

        let mut nodes = vec![Option::from(&node)];

        nodes = get_next_iter_nodes(nodes);

        let exp_node = node::Tree::new(1);
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
