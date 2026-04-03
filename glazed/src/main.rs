use iced::{widget::Container, *};

fn main() -> Result {
    application(Glazed::default, Glazed::update, Glazed::view)
        .theme(Theme::SolarizedLight)
        .run()
}

#[derive(Default)]
struct Glazed {}

#[derive(Clone, Copy)]
enum Message {
    Clicked,
}

impl Glazed {
    fn view(&self) -> Element<'_, Message> {
        widget::row![
            widget::column![
                widget::text("Widget explorer application for the iced-glaze widgets"),
                surround(glaze::button("A button").on_press(Message::Clicked).into()),
                surround(glaze::button("disabled button").into()),
                surround(
                    glaze::lozenge_button("lozenge button")
                        .on_press(Message::Clicked)
                        .into()
                ),
                surround(glaze::lozenge_button("disabled lozenge button").into()),
            ],
            widget::column![
                widget::text(""),
                iced::widget::button("test"),
                glaze::button("A button").on_press(Message::Clicked),
                glaze::button("disabled button"),
                glaze::lozenge_button("lozenge button").on_press(Message::Clicked),
                glaze::lozenge_button("disabled lozenge button"),
            ]
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
