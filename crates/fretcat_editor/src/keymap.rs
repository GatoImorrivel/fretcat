use fretcat_effects::{ChainCommand, ChainData, effects::Overdrive};
use nih_plug::nih_log;
use nih_plug_vizia::vizia::prelude::{Code, KeyChord, Keymap, KeymapEntry, LensExt, Modifiers};

pub fn make_keymap() -> Keymap<Action> {
    Keymap::from(vec![
        (
            KeyChord::new(Modifiers::CTRL | Modifiers::SHIFT, Code::KeyP),
            KeymapEntry::new(Action::PrintChain, |ex| {
                nih_log!("{:#?}", ChainData::chain.get(ex).borrow());
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL | Modifiers::SHIFT, Code::KeyI),
            KeymapEntry::new(Action::InsertChain, |ex| {
                ChainData::chain
                    .get(ex)
                    .borrow()
                    .add_to_queue(ChainCommand::Insert(Box::new(Overdrive::default())));
            }),
        ),
    ])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    PrintChain,
    InsertChain,
}
