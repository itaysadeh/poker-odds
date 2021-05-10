pub fn rank(ind: u8) -> u8 {
    (ind - (ind % 4)) >> 2
}

pub fn suit(ind: u8) -> u8 {
    ind % 4
}

pub fn ind(rank: u8, suit: u8) -> u8 {
    rank * 4 + suit
}