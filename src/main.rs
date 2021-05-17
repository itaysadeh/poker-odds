use poker_odds::eval;

fn main() {
    let mut cards: Vec<u8> = vec![0, 1, 6, 4, 5, 22, 21];

    eval::evaluator::test(&mut cards);
}
