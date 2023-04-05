pub mod components;

#[macro_use]
pub(self) mod macros;

use crossbeam::atomic::AtomicCell;
use nih_plug::{
    nih_log,
    prelude::{Editor, GuiContext},
};
use nih_plug_iced::*;
use std::sync::Arc;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 848;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

pub(crate) fn create(editor_state: Arc<IcedState>, ui_message: Arc<AtomicCell<Option<(usize, f32)>>>) -> Option<Box<dyn Editor>> {
    nih_log!("CREATING EDITOR");
    create_iced_editor::<FretCatEditor>(editor_state, ui_message)
}

struct FretCatEditor {
    context: Arc<dyn GuiContext>,
    btn_state: button::State,
    ui_message: Arc<AtomicCell<Option<(usize, f32)>>>
}

#[derive(Debug, Clone, Copy)]
enum Message {
    BtnPressed,
}

impl IcedEditor for FretCatEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<AtomicCell<Option<(usize, f32)>>>;

    fn new(
        _params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = FretCatEditor {
            context,
            btn_state: button::State::new(),
            ui_message: _params.clone()
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
            Message::BtnPressed => { self.ui_message.swap(Some((0, 1f32))); },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .push(
                Button::new(&mut self.btn_state, Text::new("Press me"))
                    .on_press(Message::BtnPressed)
            )
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
