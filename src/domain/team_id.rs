use serde::{Deserialize, Serialize};

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamId(i64);

static NEXT_ID: LazyLock<AtomicI64> = LazyLock::new(|| AtomicI64::new(0));

impl TeamId {
    pub fn new() -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        TeamId(id)
    }
}

impl std::fmt::Display for TeamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
