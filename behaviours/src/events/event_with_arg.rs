use iced::advanced::Shell;

/// Optionally wraps an event handler which uses a callback with an argument to form a message
pub enum EventWithArg<'a, Message, Arg> {
    None,
    Closure(Box<dyn Fn(Arg) -> Message + 'a>),
}

/// Helper methods to build callbacks
pub struct EventWithArgBuilder<'a, EventOwner, Message, Arg> {
    updater: fn(&mut EventOwner, EventWithArg<'a, Message, Arg>),
    owner: EventOwner,
}

impl<'a, EventOwner, Message: Clone, Arg> EventWithArgBuilder<'a, EventOwner, Message, Arg> {
    /// Construct a new [`EventWithArgBuilder`]
    pub fn new(
        owner: EventOwner,
        updater: fn(&mut EventOwner, EventWithArg<'a, Message, Arg>),
    ) -> Self {
        Self { owner, updater }
    }

    /// When the event triggers this closure will be called to build the message.
    pub fn send_with(mut self, msg_fn: impl Fn(Arg) -> Message + 'a) -> EventOwner {
        (self.updater)(&mut self.owner, EventWithArg::new_closure(msg_fn));
        self.owner
    }

    /// When the event triggers and the paramter is Some this closure will be called to build the message.
    ///
    /// If the parameter is None the event will be disabled
    pub fn maybe_send_with(mut self, msg_fn: Option<impl Fn(Arg) -> Message + 'a>) -> EventOwner {
        (self.updater)(&mut self.owner, EventWithArg::new_closure_maybe(msg_fn));
        self.owner
    }
}

impl<'a, Message: Clone, Arg> EventWithArg<'a, Message, Arg> {
    pub(crate) fn new_closure(msg_fn: impl Fn(Arg) -> Message + 'a) -> Self {
        EventWithArg::Closure(Box::new(msg_fn))
    }

    pub(crate) fn new_closure_maybe(message: Option<impl Fn(Arg) -> Message + 'a>) -> Self {
        match message {
            Some(message) => EventWithArg::Closure(Box::new(message)),
            None => EventWithArg::None,
        }
    }

    /// True if a Message has been assigned to be sent when this event is raised
    pub fn is_active(&self) -> bool {
        match self {
            EventWithArg::None => false,
            _ => true,
        }
    }

    /// True when NO Message has been assigned to this event
    pub fn is_disabled(&self) -> bool {
        !self.is_active()
    }

    /// Raise this event on the provided [`Shell`] if a message has been assigned
    pub fn publish_to(&self, shell: &mut Shell<'_, Message>, arg: Arg) {
        match self {
            EventWithArg::None => {}
            EventWithArg::Closure(f) => shell.publish(f(arg)),
        }
    }
}
