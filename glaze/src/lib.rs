use iced::*;

pub mod button;

pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> button::GlazedButton<'a, Message, Theme, Renderer>
where
    Theme: button::visual::Catalog<button::standard::Style> + 'a,
    Renderer: advanced::Renderer,
{
    button::GlazedButton::new(content, button::standard::Visual::default())
}

pub fn lozenge_button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> button::GlazedButton<'a, Message, Theme, Renderer, button::lozenge::Visual>
where
    Theme: button::visual::Catalog<button::lozenge::Style> + 'a,
    Renderer: advanced::Renderer,
{
    button::GlazedButton::new(content, button::lozenge::Visual::default())
}
