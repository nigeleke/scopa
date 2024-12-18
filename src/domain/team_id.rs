use derive_more::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamId(i64);

impl TeamId {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        TeamId(rng.gen::<i64>())
    }
}
