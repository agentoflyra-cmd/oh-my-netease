mod api;

use iced::{
    widget::{button, column, text, Column},
    Font,
    Length::Fill,
};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            button("增加").on_press(Message::Increment),
            text(self.value),
            button("减少").on_press(Message::Decrement),
            text("Hello Iced!")
                .font(Font::MONOSPACE)
                .size(30)
                .line_height(1.5)
                .width(Fill)
                .height(Fill)
                .center()
        ]
    }
}

pub fn main() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view).run()
}

#[cfg(test)]
mod test {
    use crate::{Counter, Message};

    #[test]
    fn test_value() {
        let mut counter = Counter { value: 0 };
        counter.update(Message::Increment);
        counter.update(Message::Increment);
        counter.update(Message::Decrement);
        assert_eq!(counter.value, 1, "main:test_value:should be 1");
    }
}
