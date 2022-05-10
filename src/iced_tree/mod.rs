use std::rc::Rc;

use iced::{button, Alignment, Button, Column, Element, Renderer, Row, Sandbox, Text};
use rand::Rng;

pub mod node;

#[derive(Default)]
pub struct Tree {
    root: Rc<node::Node>,
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
        print_tree(self.root.clone());
        // Finish printing the tree

        view.into()
    }
}

fn print_tree(node: Rc<node::Node>) /* -> Vec<Vec<Option<node::Node>>> */
{
    // let mut values = vec![vec![]];

    let mut nodes = vec![vec![Option::from(Rc::clone(&node))]];

    let mut last_depth = nodes.len();

    loop {
        nodes.push(Vec::new());
        let non_leaf_nodes = &nodes[nodes.len() - 2];
        for node in non_leaf_nodes {
            nodes
                .last()
                .unwrap()
                .push(match node.as_ref().unwrap().left {
                    None => None,
                    Some(n) => Option::from(Rc::clone(&n)),
                });
        }

        if last_depth == nodes.len() {
            break;
        }
        last_depth = nodes.len();
    }

    for vec in nodes {
        for n in vec {
            println!(
                "{}",
                match n {
                    Some(n) => n.value.to_string(),
                    None => "-".to_string(),
                }
            );
        }
    }

    // nodes.into_iter().map(|n| n.value).collect();

    println!("Node {} with depth {}.", node.value, node.get_depth());
    // nodes
}
