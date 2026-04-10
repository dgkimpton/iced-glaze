use iced::advanced::Shell;

/// Optionally wraps an event handler which can either be a callback to generate a message from a boolean
/// or two separate handlers each of which can be simple messages for yes/no or callbacks for big messages
pub enum EventWithTwoStates<'a, Message, ArgType: PartialEq> {
    None,
    BothAsOne(super::EventWithArg<'a, Message, ArgType>),
    Separate(ArgType, super::Event<'a, Message>, super::Event<'a, Message>),
}

/// Helper methods to build callbacks
pub struct EventWithTwoStatesBuilder<'a, EventOwner, Message, ArgType: PartialEq> {
    updater: fn(&mut EventOwner, EventWithTwoStates<'a, Message, ArgType>),
    owner: EventOwner,
    truthy: ArgType,
}

impl<'a, EventOwner, Message: Clone, ArgType: PartialEq>
    EventWithTwoStatesBuilder<'a, EventOwner, Message, ArgType>
{
    /// Construct a new [`EventWithTwoStatesBuilder`]
    pub fn new(
        owner: EventOwner,
        truthy: ArgType,
        updater: fn(&mut EventOwner, EventWithTwoStates<'a, Message, ArgType>),
    ) -> Self {
        Self {
            owner,
            updater,
            truthy,
        }
    }

    /// When the event triggers this closure will be called to build the message.
    ///
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn send_with(mut self, msg_fn: impl Fn(ArgType) -> Message + 'a) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            EventWithTwoStates::BothAsOne(super::EventWithArg::new_closure(msg_fn)),
        );
        self.owner
    }

    /// When the event triggers and the paramter is Some this closure will be called to build the message.
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow and only sometimes applicable.
    ///
    /// If the parameter is None the event will be disabled
    pub fn maybe_send_with(
        mut self,
        msg_fn: Option<impl Fn(ArgType) -> Message + 'a>,
    ) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            EventWithTwoStates::BothAsOne(super::EventWithArg::new_closure_maybe(msg_fn)),
        );
        self.owner
    }
    /// When the event triggers the correct Message will be sent
    ///
    /// The lhs message is sent when the value becomes true, the rhs is send when the value becomes false
    ///
    /// Therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn send_apart(mut self, yes_fn: Message, no_fn: Message) -> EventOwner {
        (self.updater)(
            &mut self.owner,
            EventWithTwoStates::Separate(
                self.truthy,
                super::Event::new_direct(yes_fn),
                super::Event::new_direct(no_fn),
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
                self.truthy,
                super::Event::new_direct_maybe(yes_fn),
                super::Event::new_direct_maybe(no_fn),
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
                self.truthy,
                super::Event::new_closure_maybe(yes_fn),
                super::Event::new_closure_maybe(no_fn),
            ),
        );
        self.owner
    }
}

impl<'a, Message: Clone, ArgType: PartialEq> EventWithTwoStates<'a, Message, ArgType> {
    /// True if a Message has been assigned to be sent when this event is raised
    pub fn is_active(&self) -> bool {
        match self {
            EventWithTwoStates::None => false,
            EventWithTwoStates::Separate(_truthy, f_yes, f_no) => f_yes.is_enabled() || f_no.is_enabled(),
            EventWithTwoStates::BothAsOne(f) => f.is_active(),
        }
    }

    /// True when NO Message has been assigned to this event
    pub fn is_disabled(&self) -> bool {
        !self.is_active()
    }

    /// Raise this event on the provided [`Shell`] if a message has been assigned
    pub fn publish_to(&self, shell: &mut Shell<'_, Message>, value: ArgType) {
        match self {
            EventWithTwoStates::None => {}
            EventWithTwoStates::BothAsOne(f) => f.publish_to(shell, value),
            EventWithTwoStates::Separate(truthy, f_yes, f_no) => {
                if value == *truthy {
                    f_yes.publish_to(shell)
                } else {
                    f_no.publish_to(shell)
                }
            }
        }
    }
}
