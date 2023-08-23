use std::f32::consts::PI;

use rayon::prelude::*;

#[no_mangle]
pub fn process_sample(buffer: &mut [f32]) {
    let blend = 0.0;
    let threshold = 1.0;
    let gain = 1.0;
    buffer.par_iter_mut().for_each(|sample| {
        let clean = *sample;
        let amplified = *sample * gain * threshold;
        let distorted = (2.0 / PI) * f32::atan(amplified);

        *sample = (distorted * blend) + (clean * (1.0 - blend));
    });
}
