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
    updater: fn(&mut EventOwner, Event<'a, Message>) ,
    owner: EventOwner,
}

impl<'a, EventOwner, Message: Clone> EventBuilder<'a, EventOwner, Message> {
    pub fn new(owner: EventOwner, updater: fn(&mut EventOwner, Event<'a, Message>)) -> Self {
        Self { owner, updater }
    }

    /// When the event triggers a clone of this message will be sent
    pub fn send(mut self, msg: Message) -> EventOwner {
        (self.updater)(&mut self.owner, Event::direct(msg));
        self.owner
    }

    /// When the event triggers this closure will be called to build the message.
    ///
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn send_with(mut self, msg_fn: impl Fn() -> Message + 'a) -> EventOwner {
        (self.updater)(&mut self.owner, Event::closure(msg_fn));
        self.owner
    }
    
    /// When the event triggers and the parameter is Some a clone of this message will be sent
    ///
    /// If the parameter is None the event will be disabled
    pub fn maybe_send(mut self, msg: Option<Message>) -> EventOwner {
        (self.updater)(&mut self.owner, Event::maybe_direct(msg));
        self.owner
    }

    /// When the event triggers and the paramter is Some this closure will be called to build the message.
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow and only sometimes applicable.
    ///
    /// If the parameter is None the event will be disabled
    pub fn maybe_send_with(mut self, msg_fn: Option<impl Fn() -> Message + 'a>) -> EventOwner {
        (self.updater)(&mut self.owner, Event::maybe_closure(msg_fn));
        self.owner
    }
}

impl<'a, Message: Clone> Event<'a, Message> {
    pub fn direct(msg: Message) -> Self {
        Self::Direct(msg)
    }

    pub fn maybe_direct(msg: Option<Message>) -> Self {
        match msg {
            Some(msg) => Self::direct(msg),
            None => Self::None,
        }
    }

    pub fn closure(on: impl Fn() -> Message + 'a) -> Self {
        Self::Closure(Box::new(on))
    }

    pub fn maybe_closure(on: Option<impl Fn() -> Message + 'a>) -> Self {
        match on {
            Some(on) => Self::closure(on),
            None => Self::None,
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            Event::None => false,
            _ => true,
        }
    }

    pub fn is_disabled(&self) -> bool {
        !self.is_active()
    }

    pub fn publish_to(&self, shell: &mut Shell<'_, Message>) {
        match self {
            Event::None => {}
            Event::Direct(message) => shell.publish(message.clone()),
            Event::Closure(f) => shell.publish(f()),
        }
    }
}

impl<'a, Message: Clone, Arg> EventWithArg<'a, Message, Arg> {
    pub fn closure(on: impl Fn(Arg) -> Message + 'a) -> Self {
        Self::Closure(Box::new(on))
    }
    pub fn maybe_closure(on: Option<impl Fn(Arg) -> Message + 'a>) -> Self {
        match on {
            Some(on) => Self::closure(on),
            None => Self::None,
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            EventWithArg::None => false,
            _ => true,
        }
    }

    pub fn is_disabled(&self) -> bool {
        !self.is_active()
    }

    pub fn publish_to(&self, shell: &mut Shell<'_, Message>, arg: Arg) {
        match self {
            EventWithArg::None => {}
            EventWithArg::Closure(f) => shell.publish(f(arg)),
        }
    }
}
