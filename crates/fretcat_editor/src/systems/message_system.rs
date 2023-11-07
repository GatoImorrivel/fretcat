use std::{fmt::Debug, sync::Arc};

use nih_plug::vizia::prelude::*;

#[derive(Debug, Clone, Copy, Data, PartialEq, Eq)]
pub enum MessageKind {
    Error,
    Warning,
    Info,
}

#[derive(Clone, Data)]
pub struct Message {
    message: String,
    kind: MessageKind,
    color: Color,

    #[data(ignore)]
    custom_content: Option<Arc<dyn Fn(&mut Context, usize) + Send + Sync>>,
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("message", &self.message)
            .field("kind", &self.kind)
            .field("color", &self.color)
            .finish()
    }
}

const MESSAGE_OPACITY: u8 = 200;
const MESSAGE_HEIGHT: f32 = 30.0;

impl Message {
    pub fn new<S: AsRef<str>>(message: S, kind: MessageKind, color: Color) -> Self {
        Self {
            message: message.as_ref().to_owned(),
            kind,
            color,
            custom_content: None,
        }
    }

    pub fn make_info<S: AsRef<str>>(message: S) -> Self {
        Self::new(
            message,
            MessageKind::Info,
            Color::rgba(80, 198, 204, MESSAGE_OPACITY),
        )
    }

    pub fn make_error<S: AsRef<str>>(message: S) -> Self {
        Self::new(
            message,
            MessageKind::Error,
            Color::rgba(235, 30, 95, MESSAGE_OPACITY),
        )
    }

    pub fn make_warning<S: AsRef<str>>(message: S) -> Self {
        Self::new(
            message,
            MessageKind::Warning,
            Color::rgba(235, 154, 33, MESSAGE_OPACITY),
        )
    }

    pub fn with_custom_content(
        mut self,
        custom_content: impl Fn(&mut Context, usize) + Send + Sync + 'static,
    ) -> Self {
        self.custom_content = Some(Arc::new(custom_content));
        self
    }
}

#[derive(Debug, Clone, Lens)]
pub struct MessageSystem {
    pub messages: Vec<Message>,
}

impl MessageSystem {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self { messages: vec![] }.build(cx, |cx| {
            cx.add_listener(|view: &mut Self, ex, event| {
                event.map(|event, _| match event {
                    MessageEvent::Info(str) => {
                        view.messages.push(Message::make_info(str));
                        ex.schedule_emit(MessageEvent::Close(view.messages.len() - 1), Instant::now() + Duration::from_secs(2));
                    }
                    MessageEvent::Error(str) => {
                        view.messages.push(Message::make_error(str));
                        ex.schedule_emit(MessageEvent::Close(view.messages.len() - 1), Instant::now() + Duration::from_secs(2));
                    }
                    MessageEvent::Warning(str) => {
                        view.messages.push(Message::make_warning(str));
                    }
                    MessageEvent::Close(index) => {
                        view.messages.remove(*index);
                    }
                    MessageEvent::Custom(msg) => {
                        view.messages.push(msg.clone());
                    }
                    MessageEvent::ClearAll => {
                        view.messages.clear();
                    }
                });
            });

            Binding::new(cx, Self::messages, |cx, bind| {
                let messages = bind.get(cx);
                let height = messages.len() as f32 * MESSAGE_HEIGHT + 10.0;
                VStack::new(cx, |cx| {
                    for (index, message) in messages.into_iter().enumerate() {
                        HStack::new(cx, |cx| {
                            Label::new(cx, &message.message)
                                .class("message-text")
                                .color(Color::whitesmoke());
                            if let Some(content) = message.custom_content {
                                (content)(cx, index);
                            }
                            Button::new(
                                cx,
                                move |cx| {
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
                .height(Pixels(height))
                .row_between(Pixels(5.0))
                .top(Stretch(1.0));
            });
        })
        .position_type(PositionType::SelfDirected)
        .height(Self::messages.map(|messages| Pixels(messages.len() as f32 * MESSAGE_HEIGHT + 10.0)))
        .width(Stretch(1.0))
    }
}

#[derive(Debug, Clone)]
pub enum MessageEvent {
    Info(String),
    Error(String),
    Warning(String),
    Close(usize),
    Custom(Message),
    ClearAll,
}

impl View for MessageSystem {
    fn element(&self) -> Option<&'static str> {
        Some("message-system")
    }
}
