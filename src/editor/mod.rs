use nih_plug::{
    nih_log,
    prelude::{Editor, GuiContext},
};
use nih_plug_iced::*;
use std::sync::Arc;

use crate::effects::EffectState;
use crate::effects::{chain::EffectChain, *};

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
    chain: EffectChain,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    GenericEffectMessage(u32, EffectMessages),
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
            chain: EffectChain::default(),
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
        Element::new(self.chain.iter_mut().fold(Column::new(), |column, effect| {
            let element = match effect {
                EffectState::Overdrive(o) => o.view().map(|msg| Message::GenericEffectMessage(o.id(), msg)),
            };

            column.push(element)
        }))
        .into()
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

impl FretCatEditor {
    pub fn get_chain_from_context(&self) -> EffectChain {
        serde_json::from_str(
            &self
                .context
                .get_state()
                .fields
                .get("chain-state")
                .expect("chain-state not found"),
        )
        .unwrap()
    }

    pub fn set_chain_to_context(&mut self, chain: &EffectChain) {
        let mut state = self.context.get_state();
        let chain_field = state
            .fields
            .get_mut("chain-state")
            .expect("chain-state not found");
        *chain_field = serde_json::to_string(chain).unwrap();
        self.context.set_state(state);
    }

    pub fn sync_chain_to_context(&mut self) {
        let mut state = self.context.get_state();
        let chain_field = state
            .fields
            .get_mut("chain-state")
            .expect("chain-state not found");
        *chain_field = serde_json::to_string(&self.chain).unwrap();
        self.context.set_state(state);
    }
}
