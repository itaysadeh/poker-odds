fn main() {
    let mut cards: Vec<u8> = vec![1, 8, 4, 25, 21, 14, 50];

    poker_odds::hand::eval(&mut cards);
}
