use crate::card;

// returns n cards with the highest rank
fn find_kickers(flags: &u64, amount: usize) -> Vec<u8> {
    let mut kickers: Vec<u8> = Vec::with_capacity(amount);
    for i in (0..52).rev() {
        if kickers.len() == amount {
            break; 
        }
        if (flags & (1 << i)) != 0 {
            kickers.push(i);
        }
    }
    assert_eq!(kickers.len(), amount);
    return kickers;
}

fn count_bits(mut flags: u8) -> usize {
    let mut count = 0;
    while flags != 0 {
        count += flags & 1;
        flags >>= 1;
    }
    return count as usize;
}

fn get_suits_3(flags: u8) -> [u8; 3] {
    match flags {
        7  => [2, 1, 0],
        11 => [3, 1, 0],
        13 => [3, 2, 0],
        14 => [3, 2, 1],
    }
}

fn get_suits_2(flags: u8) -> [u8; 2] {
    match flags {
        3  => [1, 0],
        5  => [2, 0],
        6  => [2, 1],
        9  => [3, 0],
        10 => [3, 1],
        12 => [3, 2],
    }
}

fn mask_card(card: u8) -> u64 {
    return 1 << card;
}

fn mask_rank(rank: u8) -> u64 {
    return 0xF << rank * 4;
}

pub type Showdown = (u8, u8, u8, u8, u8);

pub fn straight_flush(flags: u64) -> Option<Showdown> {
    let mask: u64 = 0x8888800000000;
    for i in 0..36 {
        if flags & (mask >> i) == (mask >> i) {
            return Some(( 51-i, 51-i-4, 51-i-8, 51-i-12, 51-i-16 ));
        }
    }
    let mask_low_ace: u64 = 0x8000000008888;
    for s in 0..4 {
        if flags & (mask_low_ace >> s) == (mask_low_ace >> s) {
            return Some(( 15-s, 15-s-4, 15-s-8, 15-s-12, 51-s ));
        }
    }
    return None;
}

pub fn four_of_a_kind(mut flags: u64) -> Option<Showdown> {
    let mut rank4: u8 = 0xFF;
    for r in (0..13).rev() {
        if flags & (mask_rank(r)) == mask_rank(r) {
            flags &= !mask_rank(r);
            break;
        }
    }
    if rank4 != 0xFF {
        let kicker = find_kickers(&flags, 1);
        return Some((
            card::ind(rank4, 3), card::ind(rank4, 2),
            card::ind(rank4, 1), card::ind(rank4, 0), kicker[0],
        ));
    }
    return None;
}

pub fn full_house(mut flags: u64) -> Option<Showdown> {
    let mut cards3: [u8; 3] = [0xFF; 3];
    let mut cards2: [u8; 2] = [0xFF; 2];
    let (mut has3, mut has2) = (false, false);

    for r in (0..13).rev() {
        if count_bits((0xF & (flags >> r*4)) as u8) == 3 {
            let suits = get_suits_3((0xF & (flags >> r*4)) as u8);
            has3 = true;
            cards3[0] = card::ind(r, suits[0]);
            cards3[1] = card::ind(r, suits[1]);
            cards3[2] = card::ind(r, suits[2]);
            flags &= !(mask_card(cards3[0]) | mask_card(cards3[1]) | mask_card(cards3[2]));
            break;
        }
    }
    for r in (0..13).rev() {
        if count_bits((0xF & (flags >> r*4)) as u8) == 2 {
            let suits = get_suits_3((0xF & (flags >> r*4)) as u8);
            cards2[0] = card::ind(r, suits[0]);
            cards2[1] = card::ind(r, suits[1]);
            flags &= !(mask_card(cards2[0]) | mask_card(cards2[1]));
        }
    }

    return Some((
        cards3[0], cards3[1], cards3[2], cards2[0], cards2[1],
    ));
}

pub fn flush(flags: u64, suits: &[u8; 4]) -> Option<Showdown> {
    let mut cards: Vec<u8> = Vec::with_capacity(5);
    let mut suit = 0xFF;
    // checks if there's a flush
    for s in 0..4 {
        if suits[s] >= 5 {
            suit = s as u8;
        }
    }
    if suit == 0xFF {
        return None;
    }
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
        cards[0], cards[1], cards[2], cards[3], cards[4]
    ));
}

pub fn straight(flags: u64) -> Option<Showdown> {
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
    // finds the cards that form the hand
    for r in (rank-4..=rank).rev() {
        if cards.len() == 5 {
            break;
        }
        for s in (0..4).rev() {
            if (flags & (1 << card::ind(r, s))) != 0 {
                cards.push(card::ind(r, s));
                break;
            }
        }
    }
    return Some((
        cards[0], cards[1], cards[2], cards[3], cards[4]
    ));
}

pub fn three_of_a_kind(mut flags: u64, threes: u16) -> Option<Showdown> {
    if threes == 0 {
        return None;
    }

    let mut cards3: Vec<u8> = Vec::with_capacity(3);
    let mut rank3: u8 = 0xFF;

    // finds the rank of the hand
    for r in (0..13).rev() {
        if (threes & (1 << r)) != 0 {
            rank3 = r;
            break;
        }
    }
    // finds the 3 cards that form the hand
    for s in (0..4).rev() {
        if cards3.len() == 3 { break; }
        let ind: u8 = card::ind(rank3, s);
        if (flags & (1 << ind)) != 0 {
            cards3.push(ind);
            flags &= !(1 << ind);
        }
    }
    // finds the two kickers with the highest rank
    let kickers = find_kickers(&flags, 2);

    return Some((
        cards3[0], cards3[1], cards3[2], kickers[0], kickers[1]
    ));
}

pub fn two_pair(mut flags: u64, pairs: u16) -> Option<Showdown> {
    if pairs == 0 {
        return None;
    }

    let mut cards: Vec<u8> = Vec::with_capacity(5);
    let mut pair_amount: u8 = 0;

    // finds the pairs with the highest rank
    for r in (0..13).rev() {
        if cards.len() == 4 { break; }
        if (pairs & (1 << r)) != 0 {
            // found a pair with a rank of r
            pair_amount += 1;
            for s in (0..4).rev() {
                let ind: u8 = card::ind(r, s);
                if (flags & (1 << ind)) != 0 {
                    cards.push(ind);
                    flags &= !(1 << ind);
                }
            }
        }
    }
    if pair_amount < 2 { return None; }
    assert_eq!(cards.len(), 4);
    // finds the highest kicker
    let mut kicker: u8 = 0xFF;
    for card in (0..52).rev() {
        if (flags & (1 << card)) != 0 {
            kicker = card;
            break;
        }
    }
    assert_eq!(cards.len(), 5);
    return Some((
        cards[0], cards[1], cards[2], cards[3], kicker
    ));
}

pub fn one_pair(mut flags: u64, pairs: u16) -> Option<Showdown> {
    if pairs == 0 {
        return None;
    }

    let mut cards: Vec<u8> = Vec::with_capacity(5);
    // finds the pair with the highest rank
    for r in (0..13).rev() {
        if cards.len() == 2 {
            break;
        }
        if (pairs & (1 << r)) != 0 {
            for s in (0..4).rev() {
                let ind: u8 = card::ind(r, s);
                if (flags & (1 << ind)) != 0 {
                    cards.push(ind);
                    flags &= !(1 << ind);
                }
            }
        }
    }
    // finds 3 kickers with the highest rank
    for card in (0..52).rev() {
        if cards.len() == 5 {
            break;
        }
        if (flags & (1 << card)) != 0 {
            cards.push(card);
        }
    }
    assert_eq!(cards.len(), 5);
    return Some((
        cards[0], cards[1], cards[2], cards[3], cards[4]
    ));
}

pub fn high_card(flags: u64) -> Option<Showdown> {
    let mut cards: Vec<u8> = Vec::with_capacity(5);
    for card in (0..52).rev() {
        if cards.len() == 5 {
            break;
        }
        if (flags & (1 << card)) != 0 {
            cards.push(card);
        }
    }
    return Some((
        cards[0], cards[1], cards[2], cards[3], cards[4]
    ));
}