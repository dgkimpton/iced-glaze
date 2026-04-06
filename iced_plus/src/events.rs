use iced::advanced::Shell;

/// Optionally wraps an event handler which can either be a simple message or a callback to generate a message
pub enum Event<'a, Message> {
    None,
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

/// Optionally wraps an event handler which can either be a callback to generate a message from a boolean
/// or two separate handlers each of which can be simple messages for yes/no or callbacks for big messages
pub enum EventWithTwoStates<'a, Message> {
    None,
    BothAsOne(EventWithArg<'a, Message, bool>),
    Separate(Event<'a, Message>, Event<'a, Message>),
}

/// Optionally wraps an event handler which uses a callback with an argument to form a message
pub enum EventWithArg<'a, Message, Arg> {
    None,
    Closure(Box<dyn Fn(Arg) -> Message + 'a>),
}

/// Helper methods to build callbacks
pub struct EventBuilder<'a, EventOwner, Message> {
    updater: fn(&mut EventOwner, Event<'a, Message>),
    owner: EventOwner,
}

/// Helper methods to build callbacks
pub struct EventWithTwoStatesBuilder<'a, EventOwner, Message> {
    updater: fn(&mut EventOwner, EventWithTwoStates<'a, Message>),
    owner: EventOwner,
}

/// Helper methods to build callbacks
pub struct EventWithArgBuilder<'a, EventOwner, Message, Arg> {
    updater: fn(&mut EventOwner, EventWithArg<'a, Message, Arg>),
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

impl<'a, EventOwner, Message: Clone> EventWithTwoStatesBuilder<'a, EventOwner, Message> {
    /// Construct a new [`EventWithTwoStatesBuilder`]
    pub fn new(
        owner: EventOwner,
        updater: fn(&mut EventOwner, EventWithTwoStates<'a, Message>),
    ) -> Self {
        Self { owner, updater }
    }

    /// When the event triggers this closure will be called to build the message.
    ///
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn send_with(mut self, msg_fn: impl Fn(bool) -> Message + 'a) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            EventWithTwoStates::BothAsOne(EventWithArg::new_closure(msg_fn)),
        );
        self.owner
    }

    /// When the event triggers and the paramter is Some this closure will be called to build the message.
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow and only sometimes applicable.
    ///
    /// If the parameter is None the event will be disabled
    pub fn maybe_send_with(mut self, msg_fn: Option<impl Fn(bool) -> Message + 'a>) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            EventWithTwoStates::BothAsOne(EventWithArg::new_closure_maybe(msg_fn)),
        );
        self.owner
    }
    /// When the event triggers the correct Message will be sent
    /// 
    /// The lhs message is sent when the value becomes true, the rhs is send when the value becomes false
    ///
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn send_apart(
        mut self,
        yes_fn: Message,
        no_fn: Message,
    ) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            EventWithTwoStates::Separate(
                Event::new_direct(yes_fn),
                Event::new_direct(no_fn),
            ),
        );
        self.owner
    }

    /// When the event triggers the correct Message will be sent
    /// 
    /// The lhs message is sent when the value becomes true, the rhs is send when the value becomes false
    /// 
    /// If the value is None it is not sent
    ///
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn maybe_send_apart(
        mut self,
        yes_fn: Option<Message>,
        no_fn: Option<Message>,
    ) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            EventWithTwoStates::Separate(
                Event::new_direct_maybe(yes_fn),
                Event::new_direct_maybe(no_fn),
            ),
        );
        self.owner
    }

    /// When the event triggers the appropriate closure will be called to build the message provided it is not None
    ///
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn send_apart_with(
        mut self,
        yes_fn: Option<impl Fn() -> Message + 'a>,
        no_fn: Option<impl Fn() -> Message + 'a>,
    ) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            EventWithTwoStates::Separate(
                Event::new_closure_maybe(yes_fn),
                Event::new_closure_maybe(no_fn),
            ),
        );
        self.owner
    }
}

impl<'a, Message: Clone> Event<'a, Message> {
    fn new_direct(message: Message) -> Self {
        Event::Direct(message)
    }

    fn new_direct_maybe(message: Option<Message>) -> Self {
        match message {
            Some(message) => Event::Direct(message),
            None => Event::None,
        }
    }

    fn new_closure(msg_fn: impl Fn() -> Message + 'a) -> Self {
        Event::Closure(Box::new(msg_fn))
    }

    fn new_closure_maybe(message: Option<impl Fn() -> Message + 'a>) -> Self {
        match message {
            Some(message) => Event::Closure(Box::new(message)),
            None => Event::None,
        }
    }

    /// True if a Message has been assigned to be sent when this event is raised
    pub fn is_active(&self) -> bool {
        match self {
            Event::None => false,
            _ => true,
        }
    }

    /// True when NO Message has been assigned to this event
    pub fn is_disabled(&self) -> bool {
        !self.is_active()
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

impl<'a, Message: Clone, Arg> EventWithArg<'a, Message, Arg> {
    fn new_closure(msg_fn: impl Fn(Arg) -> Message + 'a) -> Self {
        EventWithArg::Closure(Box::new(msg_fn))
    }

    fn new_closure_maybe(message: Option<impl Fn(Arg) -> Message + 'a>) -> Self {
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

impl<'a, Message: Clone> EventWithTwoStates<'a, Message> {
    /// True if a Message has been assigned to be sent when this event is raised
    pub fn is_active(&self) -> bool {
        match self {
            EventWithTwoStates::None => false,
            EventWithTwoStates::Separate(f_yes, f_no) => f_yes.is_active() || f_no.is_active(),
            EventWithTwoStates::BothAsOne(f) => f.is_active(),
        }
    }

    /// True when NO Message has been assigned to this event
    pub fn is_disabled(&self) -> bool {
        !self.is_active()
    }

    /// Raise this event on the provided [`Shell`] if a message has been assigned
    pub fn publish_to(&self, shell: &mut Shell<'_, Message>, value: bool) {
        match self {
            EventWithTwoStates::None => {}
            EventWithTwoStates::BothAsOne(f) => f.publish_to(shell, value),
            EventWithTwoStates::Separate(f_yes, f_no) => match value {
                true => f_yes.publish_to(shell),
                false => f_no.publish_to(shell),
            },
        }
    }
}
