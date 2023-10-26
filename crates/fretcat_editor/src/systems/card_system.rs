use nih_plug::vizia::prelude::*;

use super::Card;

#[derive(Lens, Clone, PartialEq, Data)]
pub struct CardSystem {
    pub(crate) dragging: Option<Card>,
    pub(crate) is_dragging: bool,
    pub(crate) cursor: (f32, f32),
}

impl CardSystem {
    pub fn init(cx: &mut Context) {
        Self {
            dragging: None,
            is_dragging: false,
            cursor: (0.0, 0.0),
        }
        .build(cx);
    }

    pub fn view(cx: &mut Context) {
        Binding::new(cx, CardSystem::is_dragging, |cx, bind| {
            let is_dragging = bind.get(cx);
            if is_dragging {
                let card = CardSystem::dragging.get(cx);
                VStack::new(cx, |cx| {
                    if let Some(card) = card {
                        card.content(cx);
                    }
                })
                .background_color(Color::blue())
                .class("card-base")
                .width(Pixels(300.0))
                .position_type(PositionType::SelfDirected)
                .left(CardSystem::cursor.map(|cursor| Pixels(cursor.0)))
                .top(CardSystem::cursor.map(|cursor| Pixels(cursor.1)));
            }
        });
    }
}

pub enum CardEvent {
    DragChange(Option<Card>),
}

impl Model for CardSystem {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            WindowEvent::MouseMove(x, y) => {
                if !x.is_nan() && !y.is_nan() {
                    self.cursor = (*x / cx.scale_factor(), *y / cx.scale_factor());
                }
            }
            WindowEvent::MouseUp(btn) => match btn {
                MouseButton::Left => {
                    self.is_dragging = false;
                }
                _ => {}
            },
            _ => {}
        });

        event.map(|e, _| match e {
            CardEvent::DragChange(card) => {
                self.is_dragging = card.is_some();
                self.dragging = card.clone();
            }
        });
    }
}

