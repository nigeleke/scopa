use derive_more::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamId(Uuid);

impl TeamId {
    pub fn new() -> Self {
        TeamId(Uuid::new_v4())
    }
}
