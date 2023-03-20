use std::sync::{Mutex};

use nih_plug::params::persist::PersistentField;
use nih_plug_iced::futures::stream::select_all::Iter;

use super::Effects;

pub struct VecField<'a, T> {
    data: Mutex<Vec<T>>,
    marker: std::marker::PhantomData<&'a ()>,
}

impl <'a, T> VecField<'a, T> {
    pub fn data(&self) -> &Mutex<Vec<T>> {
        &self.data
    }
}

impl<'a, T> VecField<'a, T> {
    pub fn new(initial_value: Vec<T>) -> Self {
        Self {
            data: Mutex::new(initial_value),
            marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> PersistentField<'a, Vec<T>> for VecField<'a, T>
where
    T: serde::Serialize + serde::Deserialize<'a> + 'static + Send + Sync,
{
    fn set(&self, new_value: Vec<T>) {
        *self.data.lock().unwrap() = new_value;
    }

    fn map<F, R>(&self, f: F) -> R
    where
        F: Fn(&Vec<T>) -> R,
    {
        let data = self.data.lock().unwrap();
        let result = f(&*data);
        std::mem::drop(data); // Release the lock before returning the result
        result
    }
}