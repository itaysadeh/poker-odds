use super::math;

// returns an index between 0 and n! - 1 from a perm of size n
pub fn perm(perm: &Vec<u8>) -> u32 {
    let n: usize = perm.len();
    let mut lehmer: Vec<u8> = perm.clone();

    for i in 1..n {
        for j in (0..i).rev(){
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