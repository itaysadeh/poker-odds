use crate::math;

// returns an index between 0 and nCk - 1 from a combination
// always expects the combination to be in an ascending order
pub fn ind_nck(comb: &Vec<u8>) -> u32 {
    let k: usize = comb.len();

    let mut rank: u32 = 0;
    for i in (1..k+1).rev() {
        rank += math::nck(comb[i-1], i as u8);
    }

    return rank;
}
