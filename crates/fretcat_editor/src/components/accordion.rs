use nih_plug::vizia::prelude::*;

#[derive(Debug, Clone, Lens)]
pub struct Accordion {
    pub is_open: bool,
}

enum AccordionMessage {
    Toggle
}

impl Accordion {
    pub fn new<L, F, V>(cx: &mut Context, label: L, content: F) -> Handle<Self>
    where
        L: 'static + Fn(&mut Context) -> Handle<V>,
        F: 'static + Fn(&mut Context),
        V: 'static + View,
    {
        Self { is_open: false }.build(cx, |cx| {
            Button::new(cx, |cx| cx.emit(AccordionMessage::Toggle), |cx| {
                Binding::new(cx, Self::is_open, |cx, bind| {
                    let is_open = bind.get(cx);
                    let mut icon = "";

                    if is_open {
                        icon = "";
                    }

                    Label::new(cx, icon)
                        .font_family(vec![FamilyOwned::Name("Symbols Nerd Font Mono".to_owned())]);
                });
                (label)(cx)
            });
            VStack::new(cx, |cx| {
                (content)(cx);
            })
            .bind(Self::is_open, |mut view, bind| {
                let is_open = bind.get(view.context());
                let view = view.visibility(is_open);
                if !is_open {
                    view.height(Pixels(0.0));
                } else {
                    view.height(Auto);
                }
            });
        })
    }
}

impl View for Accordion {
    fn element(&self) -> Option<&'static str> {
        Some("accordion")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            AccordionMessage::Toggle => {
                self.is_open = !self.is_open;
            }
        });
    }
}
