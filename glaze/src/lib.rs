use iced::*;

pub mod button;
pub mod visual;
pub mod standard;

pub use button::GlazedButton;

pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> GlazedButton<'a, Message, Theme, Renderer>
where
    Theme: visual::Catalog<standard::Style> + 'a,
    Renderer: advanced::Renderer,
{
    GlazedButton::new(content, standard::ButtonVisual::default())
}
