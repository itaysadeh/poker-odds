use crate::card;
use crate::eval::hand;
// TODO fix straight() for low ace

type Board = Vec<u8>;

// TODO move to hand
pub struct Hand {
    hand_type: u8,
    cards: hand::Showdown,
}

// expects a vector of 5 to 7 cards
pub fn get_showdown(board: &mut Board) -> Hand {
    more_asserts::assert_lt!(board.len(), 8);
    more_asserts::assert_ge!(board.len(), 4);

    let mut mults: [u8; 13] = [0; 13];
    let mut suits: [u8; 04] = [0; 04];
    let mut flags: u64 = 0;
    let mut score: u32 = 0;
    
    let (mut has4, mut has3, mut has2) = (0u16, 0u16, 0u16);
    
    for card in board.iter() {
        mults[card::rank(*card) as usize] += 1;
        suits[card::suit(*card) as usize] += 1;
        flags |= 1 << card;
    }

    for (rank, amount) in mults.iter().enumerate() {
        match *amount {
            2 => has2 |= (1 << rank) as u16,
            3 => has3 |= (1 << rank) as u16,
            4 => has4 |= (1 << rank) as u16,
            _ => (),
        }
    }

    let mut hand = hand::straight_flush(flags);
    let mut hand_type = 8;

    if hand != None {
        return Hand {
            hand_type: hand_type, cards: hand.unwrap()
        };
    }
    hand = hand::four_of_a_kind(flags, has4);
    hand_type -= 1;
    if hand != None {
        return Hand {
            hand_type: hand_type, cards: hand.unwrap()
        };
    }
    hand = hand::full_house(flags, has3, has2);
    hand_type -= 1;
    if hand != None {
        return Hand {
            hand_type: hand_type, cards: hand.unwrap()
        };
    }
    hand_type -= 1;
    hand = hand::flush(flags, &suits);
    if hand != None {
        return Hand {
            hand_type: hand_type, cards: hand.unwrap()
        };
    }
    hand_type -= 1;
    hand = hand::straight(flags);
    if hand != None {
        return Hand {
            hand_type: hand_type, cards: hand.unwrap()
        };
    }
    hand_type -= 1;
    hand = hand::three_of_a_kind(flags, has3);
    if hand != None {
        return Hand {
            hand_type: hand_type, cards: hand.unwrap()
        };
    }
    hand_type -= 1;
    hand = hand::two_pair(flags, has2);
    if hand != None {
        return Hand {
            hand_type: hand_type, cards: hand.unwrap()
        };
    }
    hand_type -= 1;
    hand = hand::one_pair(flags, has2);
    if hand != None {
        return Hand {
            hand_type: hand_type, cards: hand.unwrap()
        };
    }
    hand_type -= 1;
    hand = hand::high_card(flags);
    return Hand {
        hand_type: hand_type, cards: hand.unwrap()
    };
}

