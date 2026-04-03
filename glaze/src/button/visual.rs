//! Visual styling framework for the glaze
//!
use iced::advanced::{Layout, mouse, renderer};
use iced::{Color, Padding, Rectangle};

use crate::button::Status;

/// A styling function for a [`Button`].
pub type StyleFn<'a, Theme, ActiveVisualStyle> =
    Box<dyn Fn(&Theme, Status, &renderer::Style) -> ActiveVisualStyle + 'a>;

/// The theme catalog of a [`Button`].
///
/// All themes that can be used with [`Button`]
/// must implement this trait.
///
/// # Example
/// ```no_run
/// # use iced::{Color, Background};
/// # use glaze::button::{Status};
/// # use glaze::button::visual::{Catalog};
/// # use glaze::button::standard::{Style};
/// # struct MyTheme;
/// #[derive(Debug, Default)]
/// pub enum ButtonClass {
///     #[default]
///     Primary,
///     Secondary,
///     Danger
/// }
///
/// impl Catalog<Style> for MyTheme {
///     type Class<'a> = ButtonClass;
///     
///     fn default<'a>() -> Self::Class<'a> {
///         ButtonClass::default()
///     }
///     
///
///     fn style(&self, class: &Self::Class<'_>, status: Status, _inherited: &renderer::Style) -> Style {
///         let mut style = Style::default();
///
///         match class {
///             ButtonClass::Primary => {
///                 style.background = Some(Background::Color(Color::from_rgb(0.529, 0.808, 0.921)));
///             },
///             ButtonClass::Secondary => {
///                 style.background = Some(Background::Color(Color::WHITE));
///             },
///             ButtonClass::Danger => {
///                 style.background = Some(Background::Color(Color::from_rgb(0.941, 0.502, 0.502)));
///             },
///         }
///
///         style
///     }
/// }
/// ```
///
/// Although, in order to use [`Button::style`]
/// with `MyTheme`, [`Catalog::Class`] must implement
/// `From<StyleFn<'_, MyTheme>>`.
pub trait Catalog<ActiveVisualStyle>
where
    ActiveVisualStyle: VisualStyle,
{
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(
        &self,
        class: &Self::Class<'_>,
        status: Status,
        inherited: &renderer::Style,
    ) -> ActiveVisualStyle;
}

/// Base trait for the [`VisualStyle`] of [`glaze::Button`]'s.
pub trait VisualStyle {
    /// The text color that will be inherited by the [`content`] of the [`glaze::Button`]
    fn text_color(&self) -> Color;
}

/// Trait that should be implemented to define the actual rendering of [`VisualStyle`]'s.
/// Lowlights are drawn below the `content` and highlights are drawn above the `content`.
/// `visual_size` is added to the `padding`` of [`glaze::Button`] to determine how big the button
/// actually is. The visual_size returned by the [`Visual`] should be sufficient to contain
/// all the rendering done by the [`Visual`].
pub trait ButtonVisual<Renderer> {
    /// The style type that relates to this Visual
    type Style;

    /// Extra space that provides room to render the Visual, e.g. space for borders,
    /// drop shadows, etc.
    fn visual_size(&self) -> Padding;

    /// Test if the cursor location is considerd clickable for this Visual
    fn hit_test(&self, layout: Layout<'_>, padding: &Padding, cursor: mouse::Cursor) -> bool;

    /// Render the visual behind the content (e.g. button backgrounds)
    fn draw_lowlight(
        &self,
        renderer: &mut Renderer,
        layout: &Layout<'_>,
        viewport: &Rectangle,
        padding: &Padding,
        style: &Self::Style,
    );

    /// Render any highlights above the content (e.g. glimmer effects)
    fn draw_highlight(
        &self,
        _renderer: &mut Renderer,
        _layout: &Layout<'_>,
        _viewport: &Rectangle,
        _padding: &Padding,
        _style: &Self::Style,
    ) {
    }
}
