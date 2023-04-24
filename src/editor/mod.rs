use nih_plug::{
    nih_log,
    prelude::{Editor, GuiContext},
};
use nih_plug_iced::*;
use std::sync::Arc;

use crate::{effects::{chain::{Chain, ChainPtr}, ui::EffectUI, EffectUpdate}, params};

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 848;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

pub(crate) fn create(editor_state: Arc<IcedState>, chain_ptr: ChainPtr) -> Option<Box<dyn Editor>> {
    nih_log!("CREATING EDITOR");
    create_iced_editor::<FretCatEditor>(editor_state, chain_ptr)
}

struct FretCatEditor {
    context: Arc<dyn GuiContext>,
    ui_effects: Vec<Box<dyn EffectUI + Send + Sync>>,
    chain_ptr: ChainPtr
}

#[derive(Debug, Clone, Copy)]
enum Message {
    EffectUpdate(EffectUpdate)
}

impl IcedEditor for FretCatEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = ChainPtr;

    fn new(
        _params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let ui_effects = unsafe {
            let chain = &*_params.0;
            chain.build_ui()
        };

        let editor = FretCatEditor {
            context,
            ui_effects,
            chain_ptr: _params
        };

        nih_log!("INSIDE EDITOR NEW");

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
        match _message {
            Message::EffectUpdate(update) => {
                unsafe {
                    let chain = &mut *self.chain_ptr.0;
                    chain.update(update);
                }

                let (id, message) = update.take();
                self.ui_effects[id].update(message);
                nih_log!("{:#?}", update);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut column = Column::new();

        for effect in &mut self.ui_effects {
           column = column.push(effect.view().map(|msg| Self::Message::EffectUpdate(msg))); 
        }

        column.into()
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
