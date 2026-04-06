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
