//! FancyButtons allow your users to perform actions by pressing them.
//!
//! # Example
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! use iced::widget::FancyButton;
//!
//! #[derive(Clone)]
//! enum Message {
//!     FancyButtonPressed,
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     FancyButton("Press me!").on_press(Message::FancyButtonPressed).into()
//! }
//! ```
use iced::advanced::{
    self, Clipboard, Layout, Shell, Widget, layout, mouse, overlay, renderer,
    widget::{
        Operation,
        tree::{self, Tree},
    },
};
use iced::touch;
use iced::window;
use iced::{Element, Event, Length, Padding, Rectangle, Size, Vector};
use iced_plus::PaddingExtensions;

use super::visual::{Catalog, StyleFn, Visual, VisualStyle};

/// A generic widget that produces a message when pressed.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::FancyButton;
///
/// #[derive(Clone)]
/// enum Message {
///     FancyButtonPressed,
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     FancyButton("Press me!").on_press(Message::FancyButtonPressed).into()
/// }
/// ```
///
/// If a [`FancyButton::on_press`] handler is not set, the resulting [`FancyButton`] will
/// be disabled:
///
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::FancyButton;
///
/// #[derive(Clone)]
/// enum Message {
///     FancyButtonPressed,
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     FancyButton("I am disabled!").into()
/// }
/// ```
pub struct FancyButton<
    'a,
    Message,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
    ActiveVisual = super::standard::ButtonVisual,
> where
    ActiveVisual: Visual<Renderer>,
    ActiveVisual::Style: VisualStyle,
    Renderer: advanced::renderer::Renderer,
    Theme: Catalog<ActiveVisual::Style>,
{
    visual: ActiveVisual,
    content: Element<'a, Message, Theme, Renderer>,
    on_press: Option<OnPress<'a, Message>>,
    width: Length,
    height: Length,
    padding: Padding,
    clip: bool,
    class: Theme::Class<'a>,
    status: Option<Status>,
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<Message: Clone> OnPress<'_, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

impl<'a, Message, Theme, Renderer, ActiveVisual>
    FancyButton<'a, Message, Theme, Renderer, ActiveVisual>
where
    Renderer: advanced::renderer::Renderer,
    ActiveVisual: Visual<Renderer>,
    Theme: Catalog<ActiveVisual::Style>,
    ActiveVisual::Style: VisualStyle,
{
    /// Creates a new [`Button`] with the given content.
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        visual: ActiveVisual,
    ) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        FancyButton {
            visual,
            content,
            on_press: None,
            width: size.width.fluid(),
            height: size.height.fluid(),
            padding: DEFAULT_CONTENT_PADDING,
            clip: false,
            class: Theme::default(),
            status: None,
        }
    }

    /// Sets the width of the [`FancyButton`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`FancyButton`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Padding`] of the [`FancyButton`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the message that will be produced when the [`FancyButton`] is pressed.
    ///
    /// Unless `on_press` is called, the [`FancyButton`] will be disabled.
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    /// Sets the message that will be produced when the [`FancyButton`] is pressed.
    ///
    /// This is analogous to [`FancyButton::on_press`], but using a closure to produce
    /// the message.
    ///
    /// This closure will only be called when the [`FancyButton`] is actually pressed and,
    /// therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }

    /// Sets the message that will be produced when the [`FancyButton`] is pressed,
    /// if `Some`.
    ///
    /// If `None`, the [`FancyButton`] will be disabled.
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press.map(OnPress::Direct);
        self
    }

    /// Sets whether the contents of the [`FancyButton`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Sets the style of the [`Button`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> ActiveVisual::Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme, ActiveVisual::Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, ActiveVisual::Style>).into();
        self
    }

    /// Sets the style class of the [`Button`].
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
}

impl<'a, Message, Theme, Renderer, ActiveVisual> Widget<Message, Theme, Renderer>
    for FancyButton<'a, Message, Theme, Renderer, ActiveVisual>
where
    Message: 'a + Clone,
    Renderer: 'a + advanced::renderer::Renderer,
    ActiveVisual: Visual<Renderer>,
    ActiveVisual::Style: VisualStyle,
    Theme: Catalog<ActiveVisual::Style>,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::padded(
            limits,
            self.width,
            self.height,
            self.padding.expand(ActiveVisual::VISUAL_SIZE),
            |limits| {
                self.content
                    .as_widget_mut()
                    .layout(&mut tree.children[0], renderer, limits)
            },
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if shell.is_event_captured() {
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() && self.visual.hit_test(layout, &self.padding, cursor) {
                    let state = tree.state.downcast_mut::<State>();

                    state.is_pressed = true;

                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = &self.on_press {
                    let state = tree.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        if self.visual.hit_test(layout, &self.padding, cursor) {
                            shell.publish(on_press.get());
                        }

                        shell.capture_event();
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            _ => {}
        }

        let current_status = if self.on_press.is_none() {
            Status::Disabled
        } else if self.visual.hit_test(layout, &self.padding, cursor) {
            let state = tree.state.downcast_ref::<State>();

            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        render_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let viewport = if self.clip {
            layout.bounds().intersection(viewport).unwrap_or(*viewport)
        } else {
            *viewport
        };

        let status = self.status.unwrap_or(Status::Disabled);
        let style = theme.style(&self.class, status);

        self.visual
            .draw_lowlight(renderer, &layout, &viewport, &self.padding, &style);

        let child_render_style = match VisualStyle::text_color(&style) {
            Some(text_color) => renderer::Style { text_color },
            None => *render_style,
        };

        let content_layout = layout.children().next().unwrap();
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &child_render_style,
            content_layout,
            cursor,
            &viewport,
        );

        self.visual
            .draw_highlight(renderer, &layout, &viewport, &self.padding, &style);
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.on_press.is_some() && self.visual.hit_test(layout, &self.padding, cursor) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer, ActiveVisual>
    From<FancyButton<'a, Message, Theme, Renderer, ActiveVisual>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    ActiveVisual: Visual<Renderer> + 'a,
    ActiveVisual::Style: VisualStyle + 'a,
    Theme: Catalog<ActiveVisual::Style> + 'a,
    Renderer: advanced::renderer::Renderer + 'a,
{
    fn from(button: FancyButton<'a, Message, Theme, Renderer, ActiveVisual>) -> Self {
        Self::new(button)
    }
}

/// The default [`Padding`] of a [`Button`].
pub const DEFAULT_CONTENT_PADDING: Padding = Padding {
    top: 3.0,
    bottom: 3.0,
    right: 8.0,
    left: 8.0,
};

/// The possible status of a [`FancyButton`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`FancyButton`] can be pressed.
    Active,
    /// The [`FancyButton`] can be pressed and it is being hovered.
    Hovered,
    /// The [`FancyButton`] is being pressed.
    Pressed,
    /// The [`FancyButton`] cannot be pressed.
    Disabled,
}
