use iced::*;

pub mod button;
pub use button::GlazedButton;

pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> GlazedButton<'a, Message, Theme, Renderer>
where
    Theme: button::visual::Catalog<button::standard::Style> + 'a,
    Renderer: advanced::Renderer,
{
    GlazedButton::new(content, button::standard::ButtonVisual::default())
}
