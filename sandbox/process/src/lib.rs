use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator};

#[no_mangle]
pub fn process_sample(buffer: &mut [f32]) {
    buffer.par_iter_mut().enumerate().for_each(|(i, sample)| {
        *sample *= 10.0;
    });
}
