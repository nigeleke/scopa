use std::sync::{
    atomic::{AtomicI64, Ordering},
    LazyLock,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id(i64);

static NEXT_ID: LazyLock<AtomicI64> = LazyLock::new(|| AtomicI64::new(0));

impl Default for Id {
    fn default() -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        Self(id)
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.0)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_ne;

    use super::*;

    #[test]
    fn will_be_uniquely_allocated() {
        let id1 = Id::default();
        let id2 = Id::default();
        assert_ne!(id1, id2);
    }

    #[test]
    fn can_be_displayed() {
        let id = Id::default();
        let id_string = id.to_string();
        let Id(id) = id;
        assert_eq!(id_string, format!("Id({})", id));
    }
}
