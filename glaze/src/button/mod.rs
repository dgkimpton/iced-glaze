//! GlazedButtons allow your users to perform actions by pressing them.
//!
//! # Example
//! ```no_run
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced::Element<'a, Message, iced::Theme, iced::Renderer>;
//! use crate::glaze::button;
//!
//! #[derive(Clone)]
//! enum Message {
//!     ButtonPressed,
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     button("Press me!").on_press().send(Message::ButtonPressed).into()
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
use iced_plus::{padding::PaddingExtensions, *};

pub mod lozenge;
pub mod standard;
pub mod visual;

use visual::{ButtonVisual, Catalog, StyleFn, VisualStyle};

/// A generic widget that produces a message when pressed.
///
/// # Example
/// ```no_run
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced::Element<'a, Message, iced::Theme, iced::Renderer>;
/// use glaze::button;
///
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     button("Press me!").on_press().send(Message::ButtonPressed).into()
/// }
/// ```
///
/// If a [`GlazedButton::on_press`] handler is not set, the resulting [`GlazedButton`] will
/// be disabled:
///
/// ```no_run
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced::Element<'a, Message, iced::Theme, iced::Renderer>;
/// use glaze::button;
///
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     button("I am disabled!").into()
/// }
/// ```
pub struct GlazedButton<
    'a,
    Message,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
    ActiveVisual = standard::Visual,
> where
    Message: Clone,
    ActiveVisual: ButtonVisual<Renderer>,
    ActiveVisual::Style: VisualStyle,
    Renderer: advanced::renderer::Renderer,
    Theme: Catalog<ActiveVisual::Style>,
{
    visual: ActiveVisual,
    content: Element<'a, Message, Theme, Renderer>,
    on_press: events::Event<'a, Message>,
    on_hover_enter: events::Event<'a, Message>,
    on_hover_leave: events::Event<'a, Message>,
    size: Size<Length>,
    padding: Padding,
    clip: bool,
    class: Theme::Class<'a>,
    status: Option<Status>,
}

impl<'a, Message, Theme, Renderer, ActiveVisual>
    GlazedButton<'a, Message, Theme, Renderer, ActiveVisual>
where
    Message: Clone,
    Renderer: advanced::renderer::Renderer,
    ActiveVisual: ButtonVisual<Renderer>,
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

        Self {
            visual,
            content,
            on_press: events::Event::None,
            on_hover_enter: events::Event::None,
            on_hover_leave: events::Event::None,
            size: Size::new(size.width.fluid(), size.height.fluid()),
            padding: DEFAULT_CONTENT_PADDING,
            clip: false,
            class: Theme::default(),
            status: None,
        }
    }

    /// Sets the width of the [`GlazedButton`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    /// Sets the height of the [`GlazedButton`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    /// Sets the [`Padding`] of the [`GlazedButton`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the message that will be produced when the [`GlazedButton`] is pressed.
    ///
    /// If no event handler is hooked up the button is disabled
    pub fn on_press(self) -> events::EventBuilder<'a, Self, Message> {
        events::EventBuilder::new(self, |s, e| s.on_press = e)
    }
    /// Sets the message that will be produced when the cursor enters the [`GlazedButton`].
    pub fn on_hover_enter(self) -> events::EventBuilder<'a, Self, Message> {
        events::EventBuilder::new(self, |s, e| s.on_hover_enter = e)
    }

    /// Sets the message that will be produced when the cursor leaves the [`GlazedButton`].
    pub fn on_hover_leave(self) -> events::EventBuilder<'a, Self, Message> {
        events::EventBuilder::new(self, |s, e| s.on_hover_leave = e)
    }

    /// Sets whether the contents of the [`GlazedButton`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Sets the style of the [`Button`].
    #[must_use]
    pub fn style(
        mut self,
        style: impl Fn(&Theme, Status, &renderer::Style) -> ActiveVisual::Style + 'a,
    ) -> Self
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
    is_hovered: bool,
}

impl<'a, Message, Theme, Renderer, ActiveVisual> Widget<Message, Theme, Renderer>
    for GlazedButton<'a, Message, Theme, Renderer, ActiveVisual>
where
    Message: 'a + Clone,
    Renderer: 'a + advanced::renderer::Renderer,
    ActiveVisual: ButtonVisual<Renderer>,
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
        self.size
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::padded(
            limits,
            self.size.width,
            self.size.height,
            self.padding.expand(self.visual.visual_size()),
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

        let state = tree.state.downcast_mut::<State>();
        let is_enabled = self.on_press.is_active();
        let cursor_is_over = is_enabled && self.visual.hit_test(layout, &self.padding, cursor);

        // Note: hover changes need to occur before press-events in order to preserve
        // ui consistency
        if state.is_hovered != cursor_is_over {
            state.is_hovered = cursor_is_over;

            match state.is_hovered {
                true => self.on_hover_enter.publish_to(shell),
                false => self.on_hover_leave.publish_to(shell),
            }
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor_is_over {
                    state.is_pressed = true;
                    shell.capture_event();
                }
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if is_enabled {
                    if state.is_pressed {
                        state.is_pressed = false;

                        if cursor_is_over {
                            self.on_press.publish_to(shell);
                        }

                        shell.capture_event();
                    }
                }
            }

            Event::Touch(touch::Event::FingerLost { .. }) => {
                state.is_pressed = false;
            }

            _ => {}
        }

        let current_status = match (is_enabled, cursor_is_over, state.is_pressed) {
            (true, true, true) => Status::Pressed,
            (true, true, false) => Status::Hovered,
            (true, _, _) => Status::Active,
            _ => Status::Disabled,
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
        let style = theme.style(&self.class, status, render_style);

        self.visual
            .draw_lowlight(renderer, &layout, &viewport, &self.padding, &style);

        let child_render_style = renderer::Style {
            text_color: style.text_color(),
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
        if self.on_press.is_active() && self.visual.hit_test(layout, &self.padding, cursor) {
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
    From<GlazedButton<'a, Message, Theme, Renderer, ActiveVisual>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    ActiveVisual: ButtonVisual<Renderer> + 'a,
    ActiveVisual::Style: VisualStyle + 'a,
    Theme: Catalog<ActiveVisual::Style> + 'a,
    Renderer: advanced::renderer::Renderer + 'a,
{
    fn from(button: GlazedButton<'a, Message, Theme, Renderer, ActiveVisual>) -> Self {
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

/// The possible status of a [`GlazedButton`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`GlazedButton`] can be pressed.
    Active,
    /// The [`GlazedButton`] can be pressed and it is being hovered.
    Hovered,
    /// The [`GlazedButton`] is being pressed.
    Pressed,
    /// The [`GlazedButton`] cannot be pressed.
    Disabled,
}
