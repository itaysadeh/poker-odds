use poker_odds::eval;

fn main() {
    let cards: Vec<u8> = vec![0, 4, 8, 12, 16, 1, 2];
    let board = eval::hand::cards_to_boardarr(&cards);
    let hand  = eval::hand::high_card(&board);

    if hand.is_none() {
        println!("None");
    } else {
        let hand = hand.unwrap();
        println!("type: {}, rank: {}, kickers: {:?}",
                hand.hand_type, hand.hand_rank, hand.kickers);
        println!("Score: {}", eval::evaluator::score_hand(&hand));
    }
}
