//! Visual styling framework for the glaze
//!
//!
use iced::advanced::{Layout, mouse, renderer};
use iced::{Color, Padding, Rectangle};

/// Base trait for the [`VisualStyle`] of glazed widgets.
pub trait VisualStyle {
    /// The text color that will be inherited by the content of the widget
    fn text_color(&self) -> Color;
}

/// Sytling function that can be used with function based styles
pub type StyleFn<'a, Theme, WidgetStatus, ActiveVisualStyle> =
    Box<dyn Fn(&Theme, WidgetStatus, &renderer::Style) -> ActiveVisualStyle + 'a>;

/// The theme catalog of a widget used by widget implementors to provided theming functions.
///
/// All themes that can be used with a glazed widget must implement this trait.
///
/// # Example
/// ```no_run
/// # use iced::{Color, Background, advanced::renderer};
/// # use glaze::button::Status;
/// # use glaze::button::standard::Style;
/// # struct MyTheme;
/// #[derive(Debug, Default)]
/// pub enum ButtonClass {
///     #[default]
///     Primary,
///     Secondary,
///     Danger
/// }
///
/// // TODO: Styles are defined per visual, so this example needs fixing
///
/// impl iced_plus::visuals::Catalog<button::Status, Style> for MyTheme {
///     type Class<'a> = ButtonClass;
///     
///     fn default<'a>() -> Self::Class<'a> {
///         ButtonClass::default()
///     }
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
pub trait Catalog<Status, ActiveVisualStyle>
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

/// Trait that should be implemented to define the actual rendering of a [`WidgetVisual`].
/// Lowlights are drawn below the `content` and highlights are drawn above the `content`.
/// `visual_size` is added to the `padding`` of the widget to determine how big the widget
/// actually is. The visual_size returned by the [`WidgetVisual`] should be sufficient to contain
/// all the rendering done by the [`WidgetVisual`].
pub trait WidgetVisual<Renderer> {
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
        renderer: &mut Renderer,
        layout: &Layout<'_>,
        viewport: &Rectangle,
        padding: &Padding,
        style: &Self::Style,
    );
}

/// A visual for button that renders nothing and has no styling or interaction
#[derive(Default)]
pub struct NullVisual {}

/// A style that has no properties
#[derive(Default)]
pub struct NullStyle {}

impl<Renderer> WidgetVisual<Renderer> for NullVisual
where
    Renderer: renderer::Renderer,
{
    type Style = NullStyle;

    fn visual_size(&self) -> Padding {
        Padding::default()
    }

    fn hit_test(&self, _layout: Layout<'_>, _padding: &Padding, _cursor: mouse::Cursor) -> bool {
        false
    }

    fn draw_lowlight(
        &self,
        _renderer: &mut Renderer,
        _layout: &Layout<'_>,
        _viewport: &Rectangle,
        _padding: &Padding,
        _style: &Self::Style,
    ) {
    }
    
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

impl VisualStyle for NullStyle {
    fn text_color(&self) -> Color {
        Color::BLACK
    }
}
