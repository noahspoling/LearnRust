use iced::widget::{button, column, text, Column};
use iced::{Element, Settings, Font};

const CUSTOM_FONT_BYTES: &[u8] = include_bytes!("../resources/fonts/debrosee-font/Debrosee-ALPnL.ttf");

const CUSTOM_FONT: Font = Font {
    family: iced::font::Family::Name("Debrosee"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

fn main() -> iced::Result {
    iced::run("Test", update, view)
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

#[derive(Default)]
struct Counter {
    value: u64,
}

fn update(value: &mut u64 ,message: Message) {
    match message {
        Message::IncrementPressed => {
            if *value < u64::MAX {
                *value += 1;
            }
        }
        Message::DecrementPressed => {
            if *value > 0  {
                *value -= 1;
            }
        }
    }
}

fn view(value: &u64) -> Column<Message> {
    column![
        button("+")
            .on_press(Message::IncrementPressed),
            // .font(CUSTOM_FONT), 
        text(value.to_string())
            .font(CUSTOM_FONT),
        button("-")
            .on_press(Message::DecrementPressed),
            // .font(CUSTOM_FONT),
    ]
}

