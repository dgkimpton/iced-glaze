use iced::advanced::Shell;

pub enum Event<'a, Message> {
    None,
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

pub enum EventWithArg<'a, Message, Arg> {
    None,
    Closure(Box<dyn Fn(Arg) -> Message + 'a>),
}

pub struct EventBuilder<'a, EventOwner, Message> {
    updater: fn(&mut EventOwner, Event<'a, Message>),
    owner: EventOwner,
}
pub struct EventWithArgBuilder<'a, EventOwner, Message, Arg> {
    updater: fn(&mut EventOwner, EventWithArg<'a, Message, Arg>),
    owner: EventOwner,
}

impl<'a, EventOwner, Message: Clone> EventBuilder<'a, EventOwner, Message> {
    pub fn new(owner: EventOwner, updater: fn(&mut EventOwner, Event<'a, Message>)) -> Self {
        Self { owner, updater }
    }

    /// When the event triggers a clone of this message will be sent
    pub fn send(mut self, msg: Message) -> EventOwner {
        (self.updater)(&mut self.owner, Event::Direct(msg));
        self.owner
    }

    /// When the event triggers this closure will be called to build the message.
    ///
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn send_with(mut self, msg_fn: impl Fn() -> Message + 'a) -> EventOwner {
        (self.updater)(&mut self.owner, Event::Closure(Box::new(msg_fn)));
        self.owner
    }

    /// When the event triggers and the parameter is Some a clone of this message will be sent
    ///
    /// If the parameter is None the event will be disabled
    pub fn maybe_send(mut self, msg: Option<Message>) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            match msg {
                Some(msg) => Event::Direct(msg),
                None => Event::None,
            },
        );
        self.owner
    }

    /// When the event triggers and the paramter is Some this closure will be called to build the message.
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow and only sometimes applicable.
    ///
    /// If the parameter is None the event will be disabled
    pub fn maybe_send_with(mut self, msg_fn: Option<impl Fn() -> Message + 'a>) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            match msg_fn {
                Some(on) => Event::Closure(Box::new(on)),
                None => Event::None,
            },
        );
        self.owner
    }
}

impl<'a, Message: Clone> Event<'a, Message> {
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


impl<'a, EventOwner, Message: Clone, Arg> EventWithArgBuilder<'a, EventOwner, Message, Arg> {
    pub fn new(owner: EventOwner, updater: fn(&mut EventOwner, EventWithArg<'a, Message, Arg>)) -> Self {
        Self { owner, updater }
    }

    /// When the event triggers this closure will be called to build the message.
    pub fn send_with(mut self, msg_fn: impl Fn(Arg) -> Message + 'a) -> EventOwner {
        (self.updater)(&mut self.owner, EventWithArg::Closure(Box::new(msg_fn)));
        self.owner
    }

    /// When the event triggers and the paramter is Some this closure will be called to build the message.
    ///
    /// If the parameter is None the event will be disabled
    pub fn maybe_send_with(mut self, msg_fn: Option<impl Fn(Arg) -> Message + 'a>) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            match msg_fn {
                Some(on) => EventWithArg::Closure(Box::new(on)),
                None => EventWithArg::None,
            },
        );
        self.owner
    }
}

impl<'a, Message: Clone, Arg> EventWithArg<'a, Message, Arg> {
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
    pub fn publish_to(&self, shell: &mut Shell<'_, Message>, arg:Arg) {
        match self {
            EventWithArg::None => {}
            EventWithArg::Closure(f) => shell.publish(f(arg)),
        }
    }
}
