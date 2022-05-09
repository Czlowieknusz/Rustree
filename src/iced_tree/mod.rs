use iced::{button, Alignment, Button, Column, Element, Renderer, Row, Sandbox, Text};
use rand::Rng;
pub mod node;

#[derive(Default)]
pub struct Tree {
    root: node::Node,
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

fn print_tree(node: &node::Node) {
    println!("Node {} with depth {}.", node.value, node.get_depth());
}
