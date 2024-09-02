use iced::{
    theme::Theme,
    widget::{container, text, Button, Column, Container, Scrollable, Text},
    Application, Command, Element, Length, Settings,
};
use iced_aw::{helpers::card, style::CardStyles};

fn main() -> iced::Result {
    CardExample::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Debug)]
struct CardExample;

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for CardExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (CardExample, Command<Message>) {
        (CardExample, Command::none())
    }

    fn title(&self) -> String {
        String::from("Card error")
    }

    fn update(&mut self, _message: self::Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, self::Message> {
        let mut col = Column::new();

        for _i in 0..3 {
            col = col.push(
                Container::new(card(
                    text("Title"),
                    container(text("content\ncontent\ncontent\ncontent")),
                ))
                .height(100.0),
            );
        }
        col.into()
    }
}
