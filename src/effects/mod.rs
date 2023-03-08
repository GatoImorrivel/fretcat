pub fn overdrive(sample: f32, gain: f32, threshold: f32, blend: f32) -> f32 {
    let amplified = sample * gain;

    let clipped = amplified.clamp(-threshold, threshold);

    blend * clipped + (1.0 - blend) * sample
}