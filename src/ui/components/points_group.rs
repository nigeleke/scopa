#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PointsGroup {
    Scopa,
    CardsCount,
    CoinsCount,
    Settebello,
    Premiera,
}

impl std::fmt::Display for PointsGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Self::Scopa => "scopa",
                Self::CardsCount => "cards-count",
                Self::CoinsCount => "coins-count",
                Self::Settebello => "settebello",
                Self::Premiera => "premiera",
            }
        })
    }
}
