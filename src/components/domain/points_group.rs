#[derive(Clone, Copy, PartialEq)]
pub enum PointsGroup {
    CardsCount,
    CoinsCount,
    Settebello,
    Premiera,
}

impl std::fmt::Display for PointsGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                PointsGroup::CardsCount => "cards-count",
                PointsGroup::CoinsCount => "coins-count",
                PointsGroup::Settebello => "settebello",
                PointsGroup::Premiera => "premiera",
            }
        })
    }
}