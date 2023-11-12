pub struct Frame<'a> {
    left: &'a mut [f32],
    right: &'a mut [f32],
}

impl<'a> Frame<'a> {
    #[inline]
    pub fn process_both(&mut self, mut f: impl FnMut(&mut f32)) {
        self.left.iter_mut().zip(self.right.iter_mut()).for_each(|(left, right)| {
            (f)(left);
            (f)(right);
        });
    }

    #[inline]
    pub fn process_individual(&mut self, mut f: impl FnMut(&mut f32, &mut f32)) {
        self.left.iter_mut().zip(self.right.iter_mut()).for_each(|(left, right)| {
            (f)(left, right);
        });
    }

    #[inline]
    pub fn get_mut_left(&mut self) -> &mut [f32] {
        self.left
    }

    #[inline]
    pub fn get_mut_right(&mut self) -> &mut [f32] {
        self.right
    }

    #[inline]
    pub fn get_left(&self) -> &[f32] {
        self.left
    }

    #[inline]
    pub fn get_right(&self) -> &[f32] {
        self.right
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.left.len()
    }
}

impl<'a> From<&mut [&'a mut [f32]]> for Frame<'a> {
    fn from(value: &mut [&'a mut [f32]]) -> Self {
        unsafe {
            let left: *mut &mut [f32] = std::mem::transmute(&mut value[0]);
            let right: *mut &mut [f32] = std::mem::transmute(&mut value[1]);

            assert_eq!(left.as_ref().unwrap().len(), right.as_ref().unwrap().len());

            Self {
                left: &mut *left,
                right: &mut *right
            }
        }
    }
}