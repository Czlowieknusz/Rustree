use iced::{button, Alignment, Button, Column, Element, Sandbox, Text};
use tree::Node;

#[derive(Default)]
pub struct Tree {
    root: Node,
}

#[derive(Default)]
pub struct Counter {
    value: u32,
    increment_button: button::State,
    decrement_button: button::State,
    values_history: Vec<u32>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                self.values_history.push(self.value);
            }
            Message::DecrementPressed => {
                self.value -= 1;
                self.values_history.pop();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut view = Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            );

        for v in &self.values_history {
            view = view.push(Text::new(v.to_string()).size(50));
        }
        view.into()
    }
}
