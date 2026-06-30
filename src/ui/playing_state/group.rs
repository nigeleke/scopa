#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Scopa,
    CardsCount,
    CoinsCount,
    Settebello,
    Premiera,
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Group::Scopa => "scopa",
            Group::CardsCount => "cards-count",
            Group::CoinsCount => "coins-count",
            Group::Settebello => "settebello",
            Group::Premiera => "premiera",
        }
        .fmt(f)
    }
}
