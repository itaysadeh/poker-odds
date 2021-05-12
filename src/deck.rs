use rand::seq::SliceRandom;

pub type Deck = Vec<u8>;

pub fn new() -> Deck {
    return (0..52).collect();
}

pub fn shuffle(deck: &mut Deck) {
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
}

pub fn reset(deck: &mut Deck) {
    *deck = new();
}

pub fn draw_card(deck: &mut Deck) -> u8 {
    return deck.pop().unwrap();
}
