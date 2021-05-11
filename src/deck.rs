use rand::seq::SliceRandom;

pub struct Deck {
    pub cards: Vec<u8>
}

impl Deck {
    pub fn new() -> Deck {
        Deck { cards: (0..52).collect() }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> u8 {
        return self.cards.pop().unwrap();
    }

    pub fn reset(&mut self) {
        self.cards = (0..52).collect();
    }
}