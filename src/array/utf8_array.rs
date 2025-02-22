// Copyright 2022 RisingLight Project Authors. Licensed under Apache-2.0.

use std::iter::FromIterator;

use bitvec::vec::BitVec;
use serde::{Deserialize, Serialize};

use super::{Array, ArrayBuilder, ArrayEstimateExt, ArrayValidExt};

/// A collection of Rust UTF8 `String`s.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Utf8Array {
    offset: Vec<usize>,
    valid: BitVec,
    data: Vec<u8>,
}

impl Array for Utf8Array {
    type Item = str;
    type Builder = Utf8ArrayBuilder;

    fn get(&self, idx: usize) -> Option<&str> {
        if self.valid[idx] {
            let data_slice = &self.data[self.offset[idx]..self.offset[idx + 1]];
            Some(unsafe { std::str::from_utf8_unchecked(data_slice) })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.valid.len()
    }
}

impl ArrayValidExt for Utf8Array {
    fn get_valid_bitmap(&self) -> &BitVec {
        &self.valid
    }
}

impl ArrayEstimateExt for Utf8Array {
    fn get_estimated_size(&self) -> usize {
        self.data.len() + self.offset.len() + self.valid.len() / 8
    }
}

/// A builder that uses `&str` to build an [`Utf8Array`].
pub struct Utf8ArrayBuilder {
    offset: Vec<usize>,
    valid: BitVec,
    data: Vec<u8>,
}

impl ArrayBuilder for Utf8ArrayBuilder {
    type Array = Utf8Array;

    fn with_capacity(capacity: usize) -> Self {
        let mut offset = Vec::with_capacity(capacity + 1);
        offset.push(0);
        Self {
            offset,
            data: Vec::with_capacity(capacity),
            valid: BitVec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<&str>) {
        self.valid.push(value.is_some());
        if let Some(x) = value {
            self.data.extend_from_slice(x.as_bytes());
        }
        self.offset.push(self.data.len());
    }

    fn append(&mut self, other: &Utf8Array) {
        self.valid.extend_from_bitslice(&other.valid);
        self.data.extend_from_slice(&other.data);
        let start = *self.offset.last().unwrap();
        for other_offset in &other.offset[1..] {
            self.offset.push(*other_offset + start);
        }
    }

    fn finish(self) -> Utf8Array {
        Utf8Array {
            valid: self.valid,
            data: self.data,
            offset: self.offset,
        }
    }
}

// Enable `collect()` an array from iterator of `Option<&str>` or `Option<String>`.
impl<Str: AsRef<str>> FromIterator<Option<Str>> for Utf8Array {
    fn from_iter<I: IntoIterator<Item = Option<Str>>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut builder = <Self as Array>::Builder::with_capacity(iter.size_hint().0);
        for e in iter {
            if let Some(s) = e {
                builder.push(Some(s.as_ref()));
            } else {
                builder.push(None);
            }
        }
        builder.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_utf8_builder() {
        let mut builder = Utf8ArrayBuilder::with_capacity(100);
        for i in 0..100 {
            if i % 2 == 0 {
                builder.push(Some(&format!("{}", i)));
            } else {
                builder.push(None);
            }
        }
        builder.finish();
    }
}
