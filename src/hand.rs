use crate::card;

type Board = Vec<u8>;
type Showdown = (u8, u8, u8, u8, u8);

enum HandType {
    HIGH            = 0,
    PAIR            = 1,
    TWOPAIR         = 2,
    THREE           = 3,
    STRAIGHT        = 4,
    FLUSH           = 5,
    FULLHOUSE       = 6,
    FOUR            = 7,
    STRAIGHTFLUSH   = 8,
}

struct Hand {
    hand_type: HandType,
    cards: Showdown,
}

// expects a vector of 7 cards
pub fn eval(board: &mut Board) {
    assert_eq!(board.len(), 7);

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

    let x: Option<Showdown> = straight(flags);

    if x == None {
        println!("None");
    } else {
        let result: Hand = Hand {
            hand_type: HandType::FOUR,
            cards: x.unwrap(),
        };
        println!(
            "Hand type: {:?}, cards: {:?}",
            result.hand_type as u8, result.cards
        );
    }
}

fn straight_flush(flags: u64) -> Option<Showdown> {
    let mask: u64 = 0x8888800000000;
    // 52 - (4 * 4) = 36
    for i in 0..36 {
        if flags & (mask >> i) == (mask >> i) {
            return Some(( 51-i, 51-i-4, 51-i-8, 51-i-12, 51-i-16 ));
        }
    }
    // checks for low ace straight flush
    let mask_low_ace: u64 = 0x8000000008888;
    for s in 0..4 {
        if flags & (mask_low_ace >> s) == (mask_low_ace >> s) {
            return Some(( 15-s, 15-s-4, 15-s-8, 15-s-12, 51-s ));
        }
    }
    return None;
}

fn four_of_a_kind(mut flags: u64, fours: u16) -> Option<Showdown> {
    if fours == 0 { return None; }

    let mut rank4: u8 = 0xFF;
    let mut rank1: u8 = 0xFF;
    // finds the rank of the hand
    for r in (0..13).rev() {
        if (fours & (1 << r)) != 0 {
            rank4 = r;
            break;
        }
    }
    // unsets the four cards that form the hand
    flags &= !(0xF << rank4 * 4);
    // finds the kicker with the highest rank
    for card in (0..52).rev() {
        if (flags & (1 << card)) != 0 {
            rank1 = card;
            break;
        }
    }
    assert_ne!(rank4, 0xFF); assert_ne!(rank1, 0xFF);

    return Some((
        card::ind(rank4, 3), card::ind(rank4, 2), card::ind(rank4, 1),
        card::ind(rank4, 0), rank1,
    ));
}

fn full_house(flags: u64, mut threes: u16, pairs: u16) -> Option<Showdown> {
    if threes == 0 { return None; }

    let mut cards3: Vec<u8> = Vec::with_capacity(3);
    let mut cards2: Vec<u8> = Vec::with_capacity(2);
    let mut rank3: u8 = 0xFF;
    let mut rank2: u8 = 0xFF;

    // finds the rank of the hand
    for r in (0..13).rev() {
        if (threes & (1 << r)) != 0 {
            rank3 = r;
            threes &= !(1 << r);
            break;
        }
    }
    // checks if there are any pairs or threes remaining
    if (threes | pairs) == 0 { return None; }
    // finds the pair with the highest rank
    for rank in (0..13).rev() {
        if ((threes | pairs) & (1 << rank)) != 0 {
            rank2 = rank;
            break;
        }
    }
    // finds the cards that form the hand
    for s in (0..4).rev() {
        if (flags & (1 << card::ind(rank3, s))) != 0 {
            cards3.push(card::ind(rank3, s));
        }
        if (flags & (1 << card::ind(rank2, s))) != 0 {
            cards2.push(card::ind(rank2, s));
        }
    }
    assert_ne!(rank3, 0xFF); assert_ne!(rank2, 0xFF);

    return Some((
        cards3[0], cards3[1], cards3[2],
        cards2[0], cards2[1],
    ));
}

fn flush(flags: u64, suits: &[u8; 4]) -> Option<Showdown> {
    let mut cards: Vec<u8> = Vec::with_capacity(5);
    let mut suit = 0xFF;
    // checks if there's a flush
    for s in 0..4 {
        if suits[s] >= 5 {
            suit = s as u8;
        }
    }
    if suit == 0xFF { return None; }
    // finds the cards with the highest ranks that form the hand
    for rank in (0..13).rev() {
        if (flags & (1 << card::ind(rank, suit))) != 0 {
            cards.push(card::ind(rank, suit));
        }
        if cards.len() == 5 {
            break;
        }
    }
    assert_eq!(cards.len(), 5);

    return Some((
        cards[0], cards[1], cards[2],
        cards[3], cards[4]
    ));
}

fn straight(flags: u64) -> Option<Showdown> {
    let mut cards: Vec<u8> = Vec::with_capacity(5);
    let mut rank: u8 = 0xFF;

    // checks if there's a straight and finds it's rank
    for r in (0..=8).rev() {
        if (flags & (0xF << r + 00)) != 0 &&
           (flags & (0xF << r + 04)) != 0 &&
           (flags & (0xF << r + 08)) != 0 &&
           (flags & (0xF << r + 12)) != 0 &&
           (flags & (0xF << r + 16)) != 0 {
            rank = r + 4;
            break;
        }
    }
    // checks for low ace straight
    if rank == 0xFF {
        if (flags & 0xF000000000000) != 0 &&
           (flags & 0x000000000F000) != 0 &&
           (flags & 0x0000000000F00) != 0 &&
           (flags & 0x00000000000F0) != 0 &&
           (flags & 0x000000000000F) != 0 {
            rank = 3;
        }
        else {
            return None;
        }
    }
    assert_ne!(rank, 0xFF);
    // finds the cards that make the hand
    let mut rng = (0..rank+1).rev();
    if rank == 3 {
        rng = 
    }
    for r in (rank-4..=rank).rev() {
        if cards.len() == 5 { break; }
        for s in (0..4).rev() {
            if (flags & (1 << card::ind(r, s))) != 0 {
                cards.push(card::ind(r, s));
                break;
            }
        }
    }

    return Some((
        cards[0], cards[1], cards[2],
        cards[3], cards[4]
    ));
}