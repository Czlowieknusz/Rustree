use iced::{Application, Settings};

mod iced_tree;

fn main() -> Result<(), iced::Error> {
    iced_tree::Counter::run(Settings::default())
    // iced_tree::node::Node::run(Settings::default())
}
