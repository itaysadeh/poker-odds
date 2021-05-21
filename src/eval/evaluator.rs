use crate::eval::hand;
// TODO fix straight() for low ace

type Board = Vec<u8>;

pub fn test(board: &mut Board) {
    let mut flags: u64 = 0;
    for card in board.iter() {
        flags |= 1 << card;
    }
    let x = hand::full_house(flags);
    println!("{:?}", x);
}

// expects a vector of 5 to 7 cards
pub fn get_showdown(board: &mut Board) -> hand::Hand {
    more_asserts::assert_lt!(board.len(), 8);
    more_asserts::assert_ge!(board.len(), 4);

    let mut flags: u64 = 0;

    for card in board.iter() {
        flags |= 1 << card;
    }

    let mut hand = hand::straight_flush(flags);
    let mut hand_type = 8;

    if hand != None {
        return hand::Hand {
            hand_type: hand_type,
            cards: hand.unwrap(),
        };
    }
    hand = hand::four_of_a_kind(flags);
    hand_type -= 1;
    if hand != None {
        return hand::Hand {
            hand_type: hand_type,
            cards: hand.unwrap(),
        };
    }
    hand = hand::full_house(flags);
    hand_type -= 1;
    if hand != None {
        return hand::Hand {
            hand_type: hand_type,
            cards: hand.unwrap(),
        };
    }
    hand_type -= 1;
    hand = hand::flush(flags);
    if hand != None {
        return hand::Hand {
            hand_type: hand_type,
            cards: hand.unwrap(),
        };
    }
    hand_type -= 1;
    hand = hand::straight(flags);
    if hand != None {
        return hand::Hand {
            hand_type: hand_type,
            cards: hand.unwrap(),
        };
    }
    hand_type -= 1;
    hand = hand::three_of_a_kind(flags);
    if hand != None {
        return hand::Hand {
            hand_type: hand_type,
            cards: hand.unwrap(),
        };
    }
    hand_type -= 1;
    hand = hand::two_pair(flags);
    if hand != None {
        return hand::Hand {
            hand_type: hand_type,
            cards: hand.unwrap(),
        };
    }
    hand_type -= 1;
    hand = hand::one_pair(flags);
    if hand != None {
        return hand::Hand {
            hand_type: hand_type,
            cards: hand.unwrap(),
        };
    }
    hand_type -= 1;
    hand = hand::high_card(flags);
    return hand::Hand {
        hand_type: hand_type,
        cards: hand.unwrap(),
    };
}
