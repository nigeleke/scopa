use derive_more::*;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash)]
pub struct TeamId(Uuid);

impl TeamId {
    pub fn new() -> Self {
        TeamId(Uuid::new_v4())
    }
}
