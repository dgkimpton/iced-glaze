mod button;
mod check;

pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<iced::Element<'a, Message, Theme, Renderer>>,
) -> behaviours::button::GlazedButton<'a, Message, Theme, Renderer, button::Visual>
where
    Message: Clone,
    Theme: behaviours::visuals::Catalog<behaviours::button::Status, button::Style> + 'a,
    Renderer: iced::advanced::Renderer,
{
    behaviours::button::GlazedButton::new(content)
}
