mod effects;

use std::any::TypeId;
use std::sync::Arc;

use effects::EffectHandle;
use fretcat_effects::{AtomicRefCell, Chain, Overdrive};
use nih_plug::prelude::Editor;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};

use nih_plug_vizia::vizia::prelude::*;

pub use nih_plug;
pub use nih_plug_vizia;

pub type EditorState = ViziaState;

const EDITOR_WIDTH: u32 = 1260;
const EDITOR_HEIGHT: u32 = 848;

pub fn default_state() -> Arc<EditorState> {
    EditorState::new(|| (EDITOR_WIDTH, EDITOR_HEIGHT))
}

#[allow(unused_parens)]
pub type InitFlags = (Arc<AtomicRefCell<Chain>>);

#[derive(Lens)]
struct EditorData {
    chain: Arc<AtomicRefCell<Chain>>,
}

impl Model for EditorData {}

pub fn create(
    #[allow(unused_parens)] (chain): InitFlags,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        EditorData {
            chain: chain.clone(),
        }
        .build(cx);

        cx.add_theme(include_str!("../css/editor.css"));

        VStack::new(cx, |cx| {
            let chain = EditorData::chain.get(cx);
            for effect in chain.borrow().effects.iter() {
                let borrow = chain.borrow();
                let (type_id, data) = borrow.query(effect).unwrap();

                if type_id == TypeId::of::<Overdrive>() {
                    let data = data.downcast_ref::<Overdrive>().unwrap();

                    EffectHandle::<Overdrive>::new(cx, chain.clone(), effect, data);
                }
            }
        });
    })
}
