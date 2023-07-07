use fretcat_effects::EffectKind;
use nih_plug_vizia::vizia::prelude::*;

use crate::EFFECT_CARDS;

use super::{Sidebar, SidebarEvent};

pub fn effect_tab(cx: &mut Context) {
    kind_picker(cx);
    Binding::new(cx, Sidebar::selected_kind, |cx, bind| {
        let index = bind.get(cx);
        let kinds = Sidebar::effect_kinds.get(cx);
        let kind = kinds.get(index).unwrap();

        let cards = EFFECT_CARDS.get(kind).unwrap();

        VStack::new(cx, |cx| {
            cards.iter().for_each(|card| {
                card.render(cx);
            });
        }).class("cards-wrapper");
    });
}

fn kind_picker(cx: &mut Context) {
    let num_columns = 2;
    let kinds = Sidebar::effect_kinds.get(cx);
    let kind_rows: Vec<Vec<EffectKind>> = kinds
        .chunks(num_columns)
        .map(|chunk| {
            let mut v = vec![];
            for kind in chunk {
                v.push(kind.clone());
            }
            v
        })
        .collect();

    VStack::new(cx, |cx| {
        let mut cont = 0;
        for (i, row) in kind_rows.iter().enumerate() {
            HStack::new(cx, |cx| {
                for kind in row {
                    let cont2 = cont.clone();
                    let kind2 = kind.clone();
                    Binding::new(cx, Sidebar::selected_kind, move |cx, bind| {
                        let selection = bind.get(cx);
                        let class = if selection == cont2 {
                            "selected-kind"
                        } else {
                            "kind-btn"
                        };
                        Button::new(
                            cx,
                            move |e| e.emit(SidebarEvent::KindChange(cont2)),
                            |cx| Label::new(cx, &kind2.to_string()),
                        )
                        .class(class);
                    });
                    cont += 1;
                }
            })
            .class("picker-row");
        }
    })
    .class("picker-wrapper");
}