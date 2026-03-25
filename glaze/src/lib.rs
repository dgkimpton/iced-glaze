use iced::*;

pub mod button;
pub mod visual;
pub mod standard;

pub use button::FancyButton;

pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> FancyButton<'a, Message, Theme, Renderer>
where
    Theme: visual::Catalog<standard::Style> + 'a,
    Renderer: advanced::Renderer,
{
    FancyButton::new(content, standard::ButtonVisual::default())
}
