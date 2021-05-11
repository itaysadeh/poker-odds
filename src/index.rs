use crate::math;

// returns an index between 0 and n! - 1 from a permutation of size n
pub fn ind_perm(perm: &Vec<u8>) -> u32 {
    let n: usize = perm.len();
    let mut lehmer: Vec<u8> = perm.clone();

    for i in 1..n {
        for j in (0..i).rev() {
            if perm[j] < perm[i] {
                lehmer[i] -= 1;
            }
        }
    }
    let mut rank: u32 = 0;
    for i in 0..n {
        rank += (lehmer[i] as u32) * math::FACTORIAL[n - 1 - i];
    }

    return rank;
}

// returns an index between 0 and nPk - 1 from a partial permutation
pub fn ind_npk(perm: &Vec<u8>) -> u32 {
    let k: usize = perm.len();
    let mut lehmer: Vec<u8> = perm.clone();

    for i in 1..k {
        for j in (0..i).rev() {
            if perm[j] < perm[i] {
                lehmer[i] -= 1;
            }
        }
    }

    let mut rank: u32 = 0;
    for i in 0..k {
        rank += (lehmer[i] as u32) * math::npk((13 - 1 - i) as u8, (k - 1 - i) as u8);
    }

    return rank;
}

// returns an index between 0 and nCk - 1 from a partial combination
// always expects the combination to be in a descending order
pub fn ind_nck(comb: &Vec<u8>) -> u32 {
    let k: usize = comb.len();
    let mut rank: u32 = 0;

    for i in (1..k).rev() {
        rank += math::nck(comb[i - 1], i as u8);
    }

    return rank;
}
