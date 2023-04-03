use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::*;
use std::sync::{Arc, RwLock};

use crate::effects::*;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 848;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

pub(crate) fn create(editor_state: Arc<IcedState>) -> Option<Box<dyn Editor>> {
    create_iced_editor::<FretCatEditor>(editor_state, ())
}

struct FretCatEditor {
    context: Arc<dyn GuiContext>,
    chain: Option<Vec<Effects>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    GenericEffectMessage(),
}

impl IcedEditor for FretCatEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = ();

    fn new(
        _params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = FretCatEditor {
            context,
            chain: None,
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        _message: Self::Message,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        self.chain = serde_json::from_str(
            self.context
                .get_state()
                .fields
                .get("chain-state")
                .expect("No chain-state"),
        )
        .unwrap();

        let mut effect_elements = vec![];

        for effect in self.chain.as_mut().unwrap() {
            let element = match effect {
                Effects::Overdrive(o) => o.view().map(|msg| Message::GenericEffectMessage())
            };
            effect_elements.push(element);
        }

        let column = effect_elements.into_iter().fold(Column::new(), |column, element| {
            column.push(element)
        });

        let element: Element<_> = column.into();

        element
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 25. / 255.,
            g: 25. / 255.,
            b: 26. / 255.,
            a: 1.0,
        }
    }
}
