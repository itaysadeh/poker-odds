fn main() {
    let mut cards: Vec<u8> = vec![51, 50, 4, 6, 21, 22, 23];

    poker_odds::hand::eval(&mut cards);
}
