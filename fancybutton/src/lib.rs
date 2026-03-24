use iced::*;

mod button;
pub use button::*;

pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> FancyButton<'a, Message, Theme, Renderer>
where
    Theme: button::Catalog + 'a,
    Renderer: advanced::Renderer,
{
    FancyButton::new(content)
}
