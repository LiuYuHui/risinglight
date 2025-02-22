// Copyright 2022 RisingLight Project Authors. Licensed under Apache-2.0.

use std::iter::Iterator;
use std::marker::PhantomData;

use super::Array;

/// An iterator over the elements of an [`Array`].
#[derive(Clone)]
pub struct ArrayIter<'a, A: Array> {
    data: &'a A,
    pos: usize,
    _phantom: PhantomData<&'a usize>,
}

impl<'a, A: Array> ArrayIter<'a, A> {
    pub fn new(data: &'a A) -> Self {
        Self {
            data,
            pos: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, A: Array> Iterator for ArrayIter<'a, A> {
    type Item = Option<&'a A::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            None
        } else {
            let item = self.data.get(self.pos);
            self.pos += 1;
            Some(item)
        }
    }
}
