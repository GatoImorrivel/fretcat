use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::*;
use std::sync::Arc;

use crate::effects::overdrive::{self, Overdrive};
use crate::effects::EffectUI;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(1024, 848)
}

pub(crate) fn create(editor_state: Arc<IcedState>) -> Option<Box<dyn Editor>> {
    create_iced_editor::<FretCatEditor>(editor_state, ())
}

struct FretCatEditor {
    context: Arc<dyn GuiContext>,
    overdrive: Overdrive,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Overdrive(overdrive::Message),
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

            overdrive: Overdrive::new(),
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
        Column::new()
            .align_items(Alignment::Center)
            .push(
                self.overdrive
                    .view()
                    .map(move |message| Message::Overdrive(message)),
            )
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.0,
        }
    }
}
