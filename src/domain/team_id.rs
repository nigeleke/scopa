use derive_more::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamId(i64);

lazy_static! {
    static ref NEXT_ID: AtomicI64 = AtomicI64::new(0);
}

impl TeamId {
    pub fn new() -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        TeamId(id)
    }
}
