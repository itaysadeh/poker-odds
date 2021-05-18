use crate::card;

/// a player's 5 best cards from the board
pub type Showdown = Vec<u8>;
//
pub struct Hand {
    pub hand_type: u8,
    pub cards: Showdown,
}

/// returns {anmount} cards with the highest ranks
fn find_kickers(flags: u64, amount: usize) -> Vec<u8> {
    let mut kickers: Vec<u8> = Vec::with_capacity(amount);
    for i in (0..52).rev() {
        if (flags & mask_c(i)) != 0 {
            kickers.push(i);
            if kickers.len() == amount {
                break;
            }
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
// 4 ways to choose 3 bits in a nibble
fn get_ind_3(flags: u8, rank: u8) -> Vec<u8> {
    match flags {
        14 => vec![card::ind(rank, 3), card::ind(rank, 2), card::ind(rank, 1)],
        13 => vec![card::ind(rank, 3), card::ind(rank, 2), card::ind(rank, 0)],
        11 => vec![card::ind(rank, 3), card::ind(rank, 1), card::ind(rank, 0)],
        7  => vec![card::ind(rank, 2), card::ind(rank, 1), card::ind(rank, 0)],
    }
}
// 6 ways to choose 2 bits in a nibble
fn get_ind_2(flags: u8, rank: u8) -> Vec<u8> {
    match flags {
        12 => vec![card::ind(rank, 3), card::ind(rank, 2)],
        10 => vec![card::ind(rank, 3), card::ind(rank, 1)],
        9  => vec![card::ind(rank, 3), card::ind(rank, 0)],
        6  => vec![card::ind(rank, 2), card::ind(rank, 1)],
        5  => vec![card::ind(rank, 2), card::ind(rank, 0)],
        3  => vec![card::ind(rank, 1), card::ind(rank, 0)],
    }
}

fn mask_c(card: u8) -> u64 {
    return 1 << card;
}

fn mask_r(rank: u8) -> u64 {
    return 0xF << rank * 4;
}

// all functions return either a Showdown or None
// if the type can't be formed from the board (flags)

pub fn straight_flush(flags: u64) -> Option<Showdown> {
    let mask: u64 = 0x8888800000000;
    for i in 0..36 {
        if flags & (mask >> i) == (mask >> i) {
            return Some(vec![51 - i, 47 - i, 43 - i, 39 - i, 35 - i]);
        }
    }
    let mask: u64 = 0x8000000008888;
    for s in 0..4 {
        if flags & (mask >> s) == (mask >> s) {
            return Some(vec![51 - s, 15 - s, 11 - s, 7 - s, 3 - s]);
        }
    }
    return None;
}
pub fn four_of_a_kind(mut flags: u64) -> Option<Showdown> {
    let mut r4: u8 = 0xFF;
    for r in (0..13).rev() {
        if flags & (mask_r(r)) == mask_r(r) {
            r4 = r * 4;
            flags &= !mask_r(r);
            break;
        }
    }
    if r4 == 0xFF {
        return None;
    }
    let kickers = find_kickers(flags, 1);
    let cards = vec![kickers[0], r4 + 3, r4 + 2, r4 + 1, r4 + 0];
    cards.sort_unstable();
    return Some(cards);
}

pub fn full_house(mut flags: u64) -> Option<Showdown> {
    let mut cards = Vec::with_capacity(5);

    for r in (0..13).rev() {
        let rflags = (0xF & (flags >> r * 4)) as u8;
        if count_bits(rflags) == 3 {
            cards.extend(get_ind_3(rflags, r));
            for i in 0..3 {
                flags &= !(mask_c(cards[i]));
            }
            break;
        }
    }
    // for r in (0..13).rev() {
    //     let rflags = (0xF & (flags >> r * 4)) as u8;
    //     if count_bits(rflags) == 2 {
    //         //cards2 = get_ind_2(rflags, r);
    //         //r2 = r;
    //         break;
    //     }
    // }
    // if r3 != 0xFF && r2 != 0xFF {
    //     if r3 < r2 {
    //         return Some((cards2[0], cards2[1], cards3[0], cards3[1], cards3[2]));
    //     } else {
    //         return Some((cards3[0], cards3[1], cards3[2], cards2[0], cards2[1]));
    //     }
    // }
    return None;
}

pub fn flush(flags: u64) -> Option<Showdown> {
    let mut cards: Vec<u8> = Vec::with_capacity(5);

    for s in (0..4).rev() {
        for r in (0..13).rev() {
            let ind: u8 = card::ind(r, s);
            if (flags & mask_c(ind)) != 0 {
                cards.push(ind);
                if cards.len() == 5 {
                    //return Some((cards[0], cards[1], cards[2], cards[3], cards[4]));
                }
            }
        }
        cards.clear();
    }
    return None;
}

pub fn straight(flags: u64) -> Option<Showdown> {
    // let mut cards: Vec<u8> = Vec::with_capacity(5);

    // // checks if there's a straight and finds it's rank
    // for r in 0..13 {
    //     let mut ranks: u8 = 0;
    //     for s in 0..4 {
    //         if (flags & (mask_c(card::ind(r, s)))) != 0 {
    //             ranks |= ()
    //         }
    //     }
    //     if (flags & mask_r(r)) != 0 {

    //     }
    // }
    // // checks for low ace straight
    // if rank == 0xFF {
    //     if (flags & 0xF000000000000) != 0
    //         && (flags & 0x000000000F000) != 0
    //         && (flags & 0x0000000000F00) != 0
    //         && (flags & 0x00000000000F0) != 0
    //         && (flags & 0x000000000000F) != 0
    //     {
    //         rank = 3;
    //     } else {
    //         return None;
    //     }
    // }
    // assert_ne!(rank, 0xFF);
    // // finds the cards that form the hand
    // for r in (rank - 4..=rank).rev() {
    //     if cards.len() == 5 {
    //         break;
    //     }
    //     for s in (0..4).rev() {
    //         if (flags & (1 << card::ind(r, s))) != 0 {
    //             cards.push(card::ind(r, s));
    //             break;
    //         }
    //     }
    // }
    // return Some((cards[0], cards[1], cards[2], cards[3], cards[4]));
    return None;
}

pub fn three_of_a_kind(mut flags: u64) -> Option<Showdown> {
    for r in (0..13).rev() {
        let rflags = (0xF & (flags >> r * 4)) as u8;
        if count_bits(rflags) == 3 {
            let cards = get_ind_3(rflags, r);
            flags &= !(mask_c(cards[0]) | mask_c(cards[1]) | mask_c(cards[2]));
            let kickers = find_kickers(flags, 2);
            
        }
    }
    return None;
}

pub fn two_pair(mut flags: u64, pairs: u16) -> Option<Showdown> {
    if pairs == 0 {
        return None;
    }

    let mut cards: Vec<u8> = Vec::with_capacity(5);
    let mut pair_amount: u8 = 0;

    // finds the pairs with the highest rank
    for r in (0..13).rev() {
        if cards.len() == 4 {
            break;
        }
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
    if pair_amount < 2 {
        return None;
    }
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
    return None;
    //return Some((cards[0], cards[1], cards[2], cards[3], kicker));
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
    return None;
    //return Some((cards[0], cards[1], cards[2], cards[3], cards[4]));
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
    return None;
    //return Some((cards[0], cards[1], cards[2], cards[3], cards[4]));
}
