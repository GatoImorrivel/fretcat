use crossbeam::atomic::AtomicCell;
use nih_plug::{
    nih_log,
    prelude::{Editor, GuiContext},
};
use nih_plug_iced::*;
use std::sync::Arc;

use crate::effects::{chain::Chain, ui::EffectUI, EffectMessage, EffectUpdate};

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 848;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

pub(crate) fn create(editor_state: Arc<IcedState>, chain: Arc<Chain>) -> Option<Box<dyn Editor>> {
    nih_log!("CREATING EDITOR");
    create_iced_editor::<FretCatEditor>(editor_state, chain)
}

struct FretCatEditor {
    context: Arc<dyn GuiContext>,
    ui_effects: Vec<Box<dyn EffectUI + Send + Sync>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    EffectUpdate(EffectUpdate)
}

impl IcedEditor for FretCatEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<Chain>;

    fn new(
        _params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = FretCatEditor {
            context,
            ui_effects: _params.build_ui(),
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
