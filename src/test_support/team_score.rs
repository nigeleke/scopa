#[derive(Clone, Copy, Default)]
pub struct TeamScore {
    pub(crate) scopas: usize,
    pub(crate) cards: bool,
    pub(crate) coins: bool,
    pub(crate) settebello: bool,
    pub(crate) premiera: bool,
}

impl TeamScore {
    pub fn scopas(mut self, count: usize) -> Self {
        self.scopas = count;
        self
    }

    pub fn cards(mut self) -> Self {
        self.cards = true;
        self
    }

    pub fn coins(mut self) -> Self {
        self.coins = true;
        self
    }

    pub fn settebello(mut self) -> Self {
        self.settebello = true;
        self
    }

    pub fn premiera(mut self) -> Self {
        self.premiera = true;
        self
    }
}
