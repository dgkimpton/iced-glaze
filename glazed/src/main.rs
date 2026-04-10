use iced::{widget::Container, *};

fn main() -> Result {
    application(Glazed::default, Glazed::update, Glazed::view)
        .theme(Theme::Light)
        .run()
}

#[derive(Default)]
struct Glazed {
    le_toggle: bool,
    status_string: Option<String>,
}

#[derive(Clone)]
enum Message {
    Clicked,
    Toggled(bool),
    ShowStatus(String),
    ClearStatus,
    TextEntered(String),
}

use glaze::lozenge as w;

impl Glazed {
    fn view(&self) -> Element<'_, Message> {
        widget::column![
            widget::row![
                widget::column![
                    widget::text("Widget explorer application for the iced-glaze widgets"),
                    surround(
                        glaze::standard::button("A button")
                            .on_press()
                            .send(Message::Clicked)
                    ),
                    surround(glaze::standard::button("disabled button")),
                    surround(
                        glaze::lozenge::button("lozenge button")
                            .on_press()
                            .send(Message::Clicked)
                    ),
                    surround(glaze::lozenge::button("disabled lozenge button")),
                ],
                widget::column![
                    iced::widget::button("test"),
                    w::button("A button")
                        .on_press()
                        .send(Message::Clicked)
                        .on_hover()
                        .send_apart(
                            Message::ShowStatus(String::from("A button")),
                            Message::ClearStatus
                        ),
                    w::button("disabled button"),
                    w::button("lozenge button")
                        .on_press()
                        .send(Message::Clicked)
                        .on_hover()
                        .send_with(|b| Message::ShowStatus(format!("Lozenge Button Hovered {b}"))),
                    w::button("disabled lozenge button"),
                    iced::widget::toggler(self.le_toggle).on_toggle(Message::Toggled),
                    iced::widget::toggler(self.le_toggle),
                    iced::widget::text_input("", "").on_input(Message::TextEntered)
                ]
            ],
            widget::text(format!(
                "Status: {}",
                self.status_string.as_ref().unwrap_or(&String::new())
            ))
        ]
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Clicked => {
                println!("Clicked");
                Task::none()
            }

            Message::Toggled(state) => {
                self.le_toggle = state;
                Task::none()
            }
            Message::ShowStatus(status) => {
                self.status_string = Some(status.into());
                Task::none()
            }
            Message::ClearStatus => {
                self.status_string = None;
                Task::none()
            }
            Message::TextEntered(txt) => {
                println!("Text: {txt}");
                Task::none()
            }
        }
    }
}

fn surround<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
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
