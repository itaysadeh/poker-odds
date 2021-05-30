use crate::eval::hand;
use crate::eval::index;

/// hand score is a value from 0..=8,121,287
pub fn score_hand(hand: &hand::Hand) -> u32 {
    let mut score: u32 = 0;
    score += 1_000_000 * hand.htype as u32;
    score += 10_000 * hand.hrank as u32;
    score += index::ind_nck(&hand.kickers) as u32;

    return score;
}
