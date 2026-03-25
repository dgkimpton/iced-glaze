//! Standard visual rendering for default buttons
//!
use iced::Vector;
use iced::advanced::{Layout, mouse, renderer};
use iced::border::{Border, Radius};
use iced::theme::palette;
use iced::{Background, Color, Padding, Rectangle, Shadow, Theme, advanced};

use crate::button::{
    Status,
    visual::{ButtonVisual, Catalog, StyleFn, VisualStyle},
};

/// Type used to hook the Visual Rendering of the [`glaze::Button`] class
/// to provide background rendering for interactivity
#[derive(Default)]
pub struct Visual {}

impl<Renderer> ButtonVisual<Renderer> for Visual
where
    Renderer: advanced::renderer::Renderer,
{
    type Style = Style;

    fn visual_size(&self) -> Padding {
        Padding {
            top: 0.0,
            bottom: 8.0,
            right: 8.0,
            left: 8.0,
        }
    }

    fn hit_test(&self, layout: Layout<'_>, _padding: &Padding, cursor: mouse::Cursor) -> bool {
        cursor.is_over(layout.bounds())
    }

    fn draw_lowlight(
        &self,
        renderer: &mut Renderer,
        layout: &Layout<'_>,
        _viewport: &Rectangle,
        _padding: &Padding,
        style: &Self::Style,
    ) where
        Renderer: advanced::renderer::Renderer,
    {
        if style.color.is_some() {
            let bounds = layout.bounds();

            let space = <Visual as ButtonVisual<Renderer>>::visual_size(self);

            let lozenge_bounds = Rectangle {
                x: space.left,
                y: 0f32,
                width: bounds.width - space.x(),
                height: space.y(),
            } + Vector {
                x: bounds.x,
                y: bounds.y + bounds.height - space.y(),
            };

            println!("lozenge: {:?}", lozenge_bounds);

            renderer.fill_quad(
                renderer::Quad {
                    bounds: lozenge_bounds,
                    border: Border{radius:Radius::new(space.left / 2f32), ..Default::default()},
                    shadow: Shadow::default(),
                    snap: style.snap,
                },
                style.color.unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }
    }
}

/// The style of a lozenge button.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The Color of the button.
    pub color: Option<Background>,
    /// Whether the button should be snapped to the pixel grid.
    pub snap: bool,
}

impl Style {
    /// Updates the [`Style`] with the given color implemented as a [`Background`].
    pub fn with_color(self, color: impl Into<Background>) -> Self {
        Self {
            color: Some(color.into()),
            ..self
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            color: None,
            snap: true,
        }
    }
}

impl VisualStyle for Style {}

impl Catalog<Style> for Theme {
    type Class<'a> = StyleFn<'a, Self, Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    match status {
        Status::Active => active(palette),
        Status::Pressed => pressed(palette),
        Status::Hovered => hovered(palette),
        Status::Disabled => disabled(palette),
    }
}

fn disabled(palette: &palette::Extended) -> Style {
    Style {
        color: Some(Background::Color(
            palette.background.weakest.color.scale_alpha(0.5),
        )),
        snap: true,
    }
}

fn hovered(palette: &palette::Extended) -> Style {
    Style {
        color: Some(Background::Color(palette.primary.weak.color)),
        snap: true,
    }
}

fn pressed(palette: &palette::Extended) -> Style {
    Style {
        color: Some(Background::Color(palette.primary.strong.color)),
        snap: true,
    }
}

fn active(palette: &palette::Extended) -> Style {
    Style {
        color: None,
        snap: true,
    }
}
