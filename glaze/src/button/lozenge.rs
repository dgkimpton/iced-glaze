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
            bottom: 4.0,
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
        padding: &Padding,
        style: &Self::Style,
    ) where
        Renderer: advanced::renderer::Renderer,
    {
        match style.color {
            Background::Color(color) => {
                if color.a == 0f32 {
                    return;
                }
            }
            _ => {}
        }

        let bounds = layout.bounds();

        let space = <Visual as ButtonVisual<Renderer>>::visual_size(self);

        let lozenge_bounds = if style.is_expanded {
            Rectangle {
                x: 0f32,
                y: 0f32,
                width: bounds.width,
                height: space.y(),
            }
        } else {
            Rectangle {
                x: space.left + padding.left,
                y: 0f32,
                width: bounds.width - space.x() - padding.x(),
                height: space.y(),
            }
        } + Vector {
            x: bounds.x,
            y: bounds.y + bounds.height - space.y(),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: lozenge_bounds,
                border: Border {
                    radius: Radius::new(space.left / 2f32),
                    ..Default::default()
                },
                shadow: Shadow::default(),
                snap: style.snap,
            },
            style.color,
        );
    }
}

/// The style of a lozenge button.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The Highlight Color of the button.
    pub color: Background,
    /// The text [`Color`] of the button.
    pub text_color: Color,
    /// If true the lozenge is the full width, otherwise it is the width of the content
    pub is_expanded: bool,
    /// Whether the button should be snapped to the pixel grid.
    pub snap: bool,
}

impl VisualStyle for Style {
    fn text_color(&self) -> Color {
        self.text_color
    }
}

impl Catalog<Style> for Theme {
    type Class<'a> = StyleFn<'a, Self, Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status, inherited: &renderer::Style) -> Style {
        class(self, status, inherited)
    }
}

pub fn primary(theme: &Theme, status: Status, inherited: &renderer::Style) -> Style {
    let palette = theme.extended_palette();

    match status {
        Status::Active => active(palette, inherited),
        Status::Pressed => pressed(palette, inherited),
        Status::Hovered => hovered(palette, inherited),
        Status::Disabled => disabled(palette, inherited),
    }
}

fn disabled(_palette: &palette::Extended, inherited: &renderer::Style) -> Style {
    Style {
        color: Background::Color(Color::TRANSPARENT),
        text_color: inherited.text_color.scale_alpha(0.5),
        is_expanded: false,
        snap: true,
    }
}

fn active(_palette: &palette::Extended, inherited: &renderer::Style) -> Style {
    Style {
        color: Background::Color(Color::TRANSPARENT),
        text_color: inherited.text_color,
        is_expanded: false,
        snap: true,
    }
}

fn hovered(palette: &palette::Extended, inherited: &renderer::Style) -> Style {
    Style {
        color: Background::Color(palette.primary.weak.color),
        text_color: inherited.text_color,
        is_expanded: true,
        snap: true,
    }
}

fn pressed(palette: &palette::Extended, inherited: &renderer::Style) -> Style {
    Style {
        color: Background::Color(palette.primary.strong.color),
        text_color: inherited.text_color,
        is_expanded: false,
        snap: true,
    }
}
