use poker_odds::hand;

fn main() {
    let mut cards: Vec<u8> = vec![51, 50, 4, 6, 21, 22, 23];

    hand::eval(&mut cards);
}
