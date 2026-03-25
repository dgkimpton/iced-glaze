use iced::{widget::Container, *};

fn main() -> Result {
    iced::run(Glazed::update, Glazed::view)
}

#[derive(Default)]
struct Glazed {}

#[derive(Clone, Copy)]
enum Message {
    Clicked,
}

impl Glazed {
    fn view(&self) -> Element<'_, Message> {
        widget::column![
            widget::text("Widget explorer application for the iced-glaze widgets"),
            surround(
                glaze::button("A button")
                    .on_press(Message::Clicked)
                    .into()
            ),
            surround(glaze::button("disabled button").into()),
        ]
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Clicked => {
                println!("Clicked");
                Task::none()
            }
        }
    }
}

fn surround<'a, Message, Theme, Renderer>(
    content: Element<'a, Message, Theme, Renderer>,
) -> Element<'a, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer + 'a,
    Theme: widget::container::Catalog + 'a,
    Message: 'a,
    Theme::Class<'a>: From<Box<dyn Fn(&Theme) -> widget::container::Style + 'a>>,
{
    Container::new(content)
        .style(bordered_box)
        .padding(Padding::from(2f32))
        .into()
}

fn bordered_box<Theme>(_theme: &Theme) -> widget::container::Style {
    widget::container::Style {
        border: Border {
            width: 1.0,
            radius: 0.into(),
            color: Color::BLACK,
        },
        ..widget::container::Style::default()
    }
}
