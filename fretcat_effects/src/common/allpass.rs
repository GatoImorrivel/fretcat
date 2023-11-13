use super::delayline::DelayLine;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct AllPass {
    delay_line: DelayLine,
}

impl AllPass {
    pub fn new(delay_length: usize) -> Self {
        Self {
            delay_line: DelayLine::new(delay_length),
        }
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let delayed = self.delay_line.read();
        let output = -input + delayed;

        // in the original version of freeverb this is a member which is never modified
        let feedback = 0.5;

        self.delay_line
            .write_and_advance(input + delayed * feedback);

        output
    }
}