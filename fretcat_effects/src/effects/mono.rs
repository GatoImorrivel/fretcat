use super::AudioEffect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MonoState {
    Left,
    #[default]
    Off,
    Right
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Mono {
    mono_state: MonoState
}

impl Mono {
    pub fn set_state(&mut self, state: MonoState) {
        self.mono_state = state;
    }
}

impl AudioEffect for Mono {
    fn process(&mut self, input_buffer: (&mut [f32], &mut [f32]), transport: &nih_plug::prelude::Transport) {
        match self.mono_state {
            MonoState::Left => {
                input_buffer.0.iter_mut().zip(input_buffer.1.iter_mut()).for_each(|(left, right)| {
                    *right = *left;
                });
            }
            MonoState::Off => {}
            MonoState::Right => {
                input_buffer.0.iter_mut().zip(input_buffer.1.iter_mut()).for_each(|(left, right)| {
                    *left = *right;
                });
            }
        }
    }
}