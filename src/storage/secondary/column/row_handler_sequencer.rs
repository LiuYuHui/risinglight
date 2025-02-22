// Copyright 2022 RisingLight Project Authors. Licensed under Apache-2.0.

use crate::array::{ArrayBuilder, I64Array, I64ArrayBuilder};
use crate::storage::secondary::SecondaryRowHandler;

/// Generates a sequence of row-ids
pub struct RowHandlerSequencer {}

impl RowHandlerSequencer {
    pub fn sequence(rowset_id: u32, begin_row_id: u32, length: u32) -> I64Array {
        let mut builder = I64ArrayBuilder::with_capacity(length as usize);
        for row_id in begin_row_id..(begin_row_id + length) {
            let item = SecondaryRowHandler(rowset_id, row_id).as_i64();
            builder.push(Some(&item));
        }
        builder.finish()
    }
}
