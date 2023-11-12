use crate::{effects::AudioEffect, frame::Frame};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MonoState {
    Left,
    #[default]
    Off,
    Right,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Mono {
    mono_state: MonoState,
}

impl Mono {
    pub fn set_state(&mut self, state: MonoState) {
        self.mono_state = state;
    }
}

impl AudioEffect for Mono {
    fn process(&mut self, input_buffer: &mut Frame, transport: &nih_plug::prelude::Transport) {
        match self.mono_state {
            MonoState::Left => {
                input_buffer.process_individual(|left, right| *right = *left);
            }
            MonoState::Off => {}
            MonoState::Right => {
                input_buffer.process_individual(|left, right| *left = *right);
            }
        }
    }
}
