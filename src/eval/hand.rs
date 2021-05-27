use crate::card;
use crate::eval::index;

/// hand type and the 5 cards that form the hand
pub struct Hand {
    pub hand_type: u8,
    pub hand_rank: u8,
    pub kickers: Vec<u8>,
}

/// 4 arrays (for each suit) with 14 values for each rank
/// ace is indexed twice at [13] and [0] for low ace straights
pub type BoardArr = [[bool; 14]; 4];

pub fn cards_to_boardarr(cards: &Vec<u8>) -> BoardArr {
    let mut boardarr: [[bool; 14]; 4] = [[false; 14]; 4];

    for card in cards.iter() {
        boardarr[card::suit(*card) as usize][card::rank(*card) as usize + 1] = true;
    }
    for i in 0..4 {
        if boardarr[i][13] {
            boardarr[i][0] = true;
        }
    }

    return boardarr;
}

fn straight_flush(board: &BoardArr) -> Option<Hand> {
    for r in (4..14).rev() {
        for s in 0..4 {
            if board[s][r]
                && board[s][r - 1]
                && board[s][r - 2]
                && board[s][r - 3]
                && board[s][r - 4]
            {
                return Some(Hand {
                    hand_type: 8,
                    hand_rank: (r - 1) as u8,
                    kickers: vec![],
                });
            }
        }
    }
    return None;
}

fn four_of_a_kind(board: &BoardArr) -> Option<Hand> {
    let mut rank: u8 = 0xFF;

    for r in (1..14).rev() {
        if board[0][r] && board[1][r] && board[2][r] && board[3][r] {
            rank = (r - 1) as u8;
            break;
        }
    }
    if rank != 0xFF {
        let mut kickers: Vec<u8> = Vec::with_capacity(1);

        for r in (1..14).rev() {
            if board[0][r] || board[1][r] || board[2][r] || board[3][r] {
                kickers.push((r - 1) as u8);
                return Some(Hand {
                    hand_type: 7,
                    hand_rank: rank,
                    kickers: kickers,
                });
            }
        }
    }
    return None;
}

fn full_house(board: &BoardArr) -> Option<Hand> {
    let mut rank: u8 = 0xFF;

    for r in (1..14).rev() {
        let count: u8 =
            board[0][r] as u8 + board[1][r] as u8 + board[2][r] as u8 + board[3][r] as u8;
        if count == 3 {
            rank = (r - 1) as u8;
        }
    }

    if rank != 0xFF {
        let mut kickers: Vec<u8> = Vec::with_capacity(1);

        for r in (1..14).rev() {
            let count: u8 =
                board[0][r] as u8 + board[1][r] as u8 + board[2][r] as u8 + board[3][r] as u8;
            if r != (rank + 1).into() && count >= 2 {
                kickers.push((r - 1) as u8);
                return Some(Hand {
                    hand_type: 6,
                    hand_rank: rank,
                    kickers: kickers,
                });
            }
        }
    }
    return None;
}

fn flush(board: &BoardArr) -> Option<Hand> {
    let mut kickers: Vec<u8> = Vec::with_capacity(5);

    for s in 0..4 {
        for r in (1..14).rev() {
            if board[s][r] {
                kickers.push((r - 1) as u8);
                if kickers.len() == 5 {
                    kickers.reverse();
                    return Some(Hand {
                        hand_type: 5,
                        hand_rank: 0,
                        kickers: kickers,
                    });
                }
            }
        }
        kickers.clear();
    }
    return None;
}

fn straight(board: &BoardArr) -> Option<Hand> {
    for r in (4..14).rev() {
        if (board[0][r - 0] || board[1][r - 0] || board[2][r - 0] || board[3][r - 0])
            && (board[0][r - 1] || board[1][r - 1] || board[2][r - 1] || board[3][r - 1])
            && (board[0][r - 2] || board[1][r - 2] || board[2][r - 2] || board[3][r - 2])
            && (board[0][r - 3] || board[1][r - 3] || board[2][r - 3] || board[3][r - 3])
            && (board[0][r - 4] || board[1][r - 4] || board[2][r - 4] || board[3][r - 4])
        {
            return Some(Hand {
                hand_type: 4,
                hand_rank: (r - 1) as u8,
                kickers: vec![],
            });
        }
    }
    return None;
}

fn three_of_a_kind(board: &BoardArr) -> Option<Hand> {
    let mut rank: u8 = 0xFF;

    for r in (1..14).rev() {
        let count: u8 =
            board[0][r] as u8 + board[1][r] as u8 + board[2][r] as u8 + board[3][r] as u8;
        if count == 3 {
            rank = (r - 1) as u8;
        }
    }

    if rank != 0xFF {
        let mut kickers: Vec<u8> = Vec::with_capacity(2);
        for r in (1..14).rev() {
            if (r - 1) as u8 != rank && (board[0][r] || board[1][r] || board[2][r] || board[3][r]) {
                kickers.push((r - 1) as u8);
                if kickers.len() == 2 {
                    kickers.reverse();
                    return Some(Hand {
                        hand_type: 3,
                        hand_rank: rank,
                        kickers: kickers,
                    });
                }
            }
        }
    }
    return None;
}

fn two_pair(board: &BoardArr) -> Option<Hand> {
    let mut pairs: Vec<u8> = Vec::with_capacity(2);

    for r in (1..14).rev() {
        let count: u8 =
            board[0][r] as u8 + board[1][r] as u8 + board[2][r] as u8 + board[3][r] as u8;
        if count == 2 {
            pairs.push((r - 1) as u8);
            if pairs.len() == 2 {
                break;
            }
        }
    }

    if pairs.len() == 2 {
        for r in (1..14).rev() {
            if (r - 1) as u8 != pairs[0]
                && (r - 1) as u8 != pairs[1]
                && (board[0][r] || board[1][r] || board[2][r] || board[3][r])
            {
                pairs.reverse();
                return Some(Hand {
                    hand_type: 2,
                    hand_rank: index::ind_nck(&pairs) as u8,
                    kickers: vec![(r - 1) as u8],
                });
            }
        }
    }
    return None;
}

fn pair(board: &BoardArr) -> Option<Hand> {
    let mut rank: u8 = 0xFF;

    for r in (1..14).rev() {
        let count: u8 =
            board[0][r] as u8 + board[1][r] as u8 + board[2][r] as u8 + board[3][r] as u8;
        if count == 2 {
            rank = (r - 1) as u8;
            break;
        }
    }

    if rank != 0xFF {
        let mut kickers: Vec<u8> = Vec::with_capacity(2);
        for r in (1..14).rev() {
            if (r - 1) as u8 != rank && (board[0][r] || board[1][r] || board[2][r] || board[3][r]) {
                kickers.push((r - 1) as u8);
                if kickers.len() == 3 {
                    kickers.reverse();
                    return Some(Hand {
                        hand_type: 1,
                        hand_rank: rank,
                        kickers: kickers,
                    });
                }
            }
        }
    }
    return None;
}

pub fn high_card(board: &BoardArr) -> Option<Hand> {
    let mut kickers: Vec<u8> = Vec::with_capacity(2);
    for r in (1..14).rev() {
        if board[0][r] || board[1][r] || board[2][r] || board[3][r] {
            kickers.push((r - 1) as u8);
            if kickers.len() == 5 {
                break;
            }
        }
    }
    kickers.reverse();

    return Some(Hand {
        hand_type: 0,
        hand_rank: 0,
        kickers: kickers,
    });
}
