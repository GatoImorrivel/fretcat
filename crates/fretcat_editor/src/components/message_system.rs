use nih_plug::vizia::prelude::*;

#[derive(Debug, Clone, Copy, Data, PartialEq, Eq)]
pub enum MessageKind {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Data)]
pub struct Message {
    message: String,
    kind: MessageKind,
    color: Color,
    #[data(ignore)]
    on_close: Option<Box<fn(&mut EventContext)>>,
}

const MESSAGE_OPACITY: u8 = 200;
const MESSAGE_WIDTH: f32 = 300.0;
const MESSAGE_HEIGHT: f32 = 30.0;

impl Message {
    pub fn new<S: AsRef<str>>(
        message: S,
        kind: MessageKind,
        color: Color,
        on_close: Option<Box<fn(&mut EventContext)>>,
    ) -> Self {
        Self {
            message: message.as_ref().to_owned(),
            kind,
            color,
            on_close,
        }
    }

    pub fn make_info<S: AsRef<str>>(message: S) -> Self {
        Self::new(
            message,
            MessageKind::Info,
            Color::rgba(80, 198, 204, MESSAGE_OPACITY),
            None,
        )
    }

    pub fn make_error<S: AsRef<str>>(message: S) -> Self {
        Self::new(
            message,
            MessageKind::Error,
            Color::rgba(235, 30, 95, MESSAGE_OPACITY),
            None,
        )
    }

    pub fn make_warning<S: AsRef<str>>(message: S) -> Self {
        Self::new(
            message,
            MessageKind::Warning,
            Color::rgba(235, 154, 33, MESSAGE_OPACITY),
            None,
        )
    }

    pub fn with_callback(mut self, on_close: fn(&mut EventContext)) -> Self {
        self.on_close = Some(Box::new(on_close));
        self
    }
}

#[derive(Debug, Clone, Lens)]
pub struct MessageSystem {
    pub messages: Vec<Message>,
}

impl MessageSystem {
    pub fn init(cx: &mut Context) {
        Self {
            messages: vec![Message::make_error("Bolas"), Message::make_info("ola")],
        }
        .build(cx);
    }

    pub fn view(cx: &mut Context) {
        Binding::new(cx, Self::messages, |cx, bind| {
            let messages = bind.get(cx);
            let height = messages.len() as f32 * MESSAGE_HEIGHT;
            VStack::new(cx, |cx| {
                for (index, message) in messages.into_iter().enumerate() {
                    HStack::new(cx, |cx| {
                        Label::new(cx, &message.message).class("message-text").color(Color::white());
                        Button::new(
                            cx,
                            move |cx| {
                                if let Some(on_close) = &message.on_close {
                                    (on_close)(cx);
                                }
                                cx.emit(MessageEvent::Close(index));
                            },
                            |cx| Label::new(cx, ""),
                        )
                        .class("message-close-btn");
                    })
                    .class("message-body")
                    .width(Stretch(1.0))
                    .height(Pixels(MESSAGE_HEIGHT))
                    .background_color(message.color);
                }
            })
            .position_type(PositionType::SelfDirected)
            .row_between(Pixels(5.0))
            .top(Stretch(1.0))
            .height(Pixels(height + 10.0))
            .width(Stretch(1.0));
        });
    }
}

#[derive(Debug, Clone)]
pub enum MessageEvent {
    Info(String),
    Error(String),
    Warning(String),
    InfoCallback(String, fn(&mut EventContext)),
    ErrorCallback(String, fn(&mut EventContext)),
    WarningCallback(String, fn(&mut EventContext)),
    Close(usize),
}

impl Model for MessageSystem {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            MessageEvent::Info(str) => {
                self.messages.push(Message::make_info(str));
            }
            MessageEvent::Error(str) => {
                self.messages.push(Message::make_error(str));
            }
            MessageEvent::Warning(str) => {
                self.messages.push(Message::make_warning(str));
            }
            MessageEvent::InfoCallback(str, on_close) => {
                self.messages
                    .push(Message::make_info(str).with_callback(*on_close));
            }
            MessageEvent::ErrorCallback(str, on_close) => {
                self.messages
                    .push(Message::make_error(str).with_callback(*on_close));
            }
            MessageEvent::WarningCallback(str, on_close) => {
                self.messages
                    .push(Message::make_warning(str).with_callback(*on_close));
            }
            MessageEvent::Close(index) => {
                self.messages.remove(*index);
            }
        });
    }
}
