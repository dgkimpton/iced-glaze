use iced::advanced::Shell;

/// Optionally wraps an event handler which can either be a simple message or a callback to generate a message
pub enum Event<'a, Message> {
    None,
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

/// Helper methods to build callbacks
pub struct EventBuilder<'a, EventOwner, Message> {
    updater: fn(&mut EventOwner, Event<'a, Message>),
    owner: EventOwner,
}

impl<'a, EventOwner, Message: Clone> EventBuilder<'a, EventOwner, Message> {
    /// Construct a new [`EventBuilder`]
    pub fn new(owner: EventOwner, updater: fn(&mut EventOwner, Event<'a, Message>)) -> Self {
        Self { owner, updater }
    }

    /// When the event triggers a clone of this message will be sent
    pub fn send(mut self, msg: Message) -> EventOwner {
        (self.updater)(&mut self.owner, Event::new_direct(msg));
        self.owner
    }

    /// When the event triggers this closure will be called to build the message.
    ///
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn send_with(mut self, msg_fn: impl Fn() -> Message + 'a) -> EventOwner {
        (self.updater)(&mut self.owner, Event::new_closure(msg_fn));
        self.owner
    }

    /// When the event triggers and the parameter is Some a clone of this message will be sent
    ///
    /// If the parameter is None the event will be disabled
    pub fn maybe_send(mut self, msg: Option<Message>) -> EventOwner {
        (self.updater)(&mut self.owner, Event::new_direct_maybe(msg));
        self.owner
    }

    /// When the event triggers and the paramter is Some this closure will be called to build the message.
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow and only sometimes applicable.
    ///
    /// If the parameter is None the event will be disabled
    pub fn maybe_send_with(mut self, msg_fn: Option<impl Fn() -> Message + 'a>) -> EventOwner {
        (self.updater)(&mut self.owner, Event::new_closure_maybe(msg_fn));
        self.owner
    }
}

impl<'a, Message: Clone> Event<'a, Message> {
    pub(crate) fn new_direct(message: Message) -> Self {
        Event::Direct(message)
    }

    pub(crate) fn new_direct_maybe(message: Option<Message>) -> Self {
        match message {
            Some(message) => Event::Direct(message),
            None => Event::None,
        }
    }

    pub(crate) fn new_closure(msg_fn: impl Fn() -> Message + 'a) -> Self {
        Event::Closure(Box::new(msg_fn))
    }

    pub(crate) fn new_closure_maybe(message: Option<impl Fn() -> Message + 'a>) -> Self {
        match message {
            Some(message) => Event::Closure(Box::new(message)),
            None => Event::None,
        }
    }

    /// True if a Message has been assigned to be sent when this event is raised
    pub fn is_enabled(&self) -> bool {
        match self {
            Event::None => false,
            _ => true,
        }
    }

    /// True when NO Message has been assigned to this event
    pub fn is_disabled(&self) -> bool {
        !self.is_enabled()
    }

    /// Raise this event on the provided [`Shell`] if a message has been assigned
    pub fn publish_to(&self, shell: &mut Shell<'_, Message>) {
        match self {
            Event::None => {}
            Event::Direct(message) => shell.publish(message.clone()),
            Event::Closure(f) => shell.publish(f()),
        }
    }
}
