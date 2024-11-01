use derive_more::*;

#[derive(Clone, Constructor, Copy, Debug, Display, From)]
pub struct TeamCount(usize);

impl PartialEq for TeamCount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
