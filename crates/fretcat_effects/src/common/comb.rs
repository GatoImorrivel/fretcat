use super::delayline::DelayLine;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Comb {
    delay_line: DelayLine,
    feedback: f32,
    filter_state: f32,
    dampening: f32,
    dampening_inverse: f32,
}

impl Comb {
    pub fn new(delay_length: usize) -> Self {
        Self {
            delay_line: DelayLine::new(delay_length),
            feedback: 0.5,
            filter_state: 0.0,
            dampening: 0.5,
            dampening_inverse: 0.5,
        }
    }

    pub fn set_dampening(&mut self, value: f32) {
        self.dampening = value;
        self.dampening_inverse = 1.0 - value;
    }

    pub fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let output = self.delay_line.read();

        self.filter_state = output * self.dampening_inverse + self.filter_state * self.dampening;

        self.delay_line
            .write_and_advance(input + self.filter_state * self.feedback);

        output
    }
}
