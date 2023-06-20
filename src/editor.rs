use nih_plug::{
    nih_log,
    prelude::{Editor, GuiContext},
};
use nih_plug_iced::*;
use std::sync::Arc;

use crate::{effects::{chain::{Chain, ChainPtr}}, params};

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 848;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(WINDOW_WIDTH, WINDOW_HEIGHT)
}

pub(crate) fn create(editor_state: Arc<IcedState>, chain_ptr: ChainPtr) -> Option<Box<dyn Editor>> {
    create_iced_editor::<FretCatEditor>(editor_state, chain_ptr)
}

struct FretCatEditor {
    context: Arc<dyn GuiContext>,
    chain_ptr: ChainPtr
}
impl IcedEditor for FretCatEditor {
    type Executor = executor::Default;
    type Message = ();
    type InitializationFlags = ChainPtr;

    fn new(
        _params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = FretCatEditor {
            context,
            chain_ptr: _params
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

    fn view(&mut self) -> Element<'_, ()> {
        let mut effect_column = Column::new().width(Length::FillPortion(4));
        let sidebar = Column::new().width(Length::Fill);

        for effect in &mut self.chain_ptr.deref_mut().effects {
           effect_column = effect_column.push(effect.render()); 
        }

        let top_row = Row::new().height(Length::Fill);
        let bottom_row = Row::new().height(Length::FillPortion(20));


        Column::new()
            .push(top_row)
            .push(bottom_row
                .push(sidebar)
                .push(effect_column)
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
