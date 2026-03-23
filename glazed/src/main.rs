use iced::*;

fn main() -> Result {
    println!("");
    iced::run(Glazed::update, Glazed::view)
}

#[derive(Default)]
struct Glazed {}

#[derive(Clone, Copy)]
enum Message {}

impl Glazed {
    fn view(&self) -> Element<'_, Message> {
        widget::text("Widget explorer application for the iced-glaze widgets").into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {}
    }
}
