// calculates nCk
fn nck(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }
    if n == k {
        return 1;
    }
    // n! / (k! * (n-k)!) = (n! / n-k!) / k!
    let mut new_n = n;
    let mut new_k = k;

    for i in (n-(k-1))..n {
        new_n *= i;
        new_k *= n-i;
    }
    return new_n / new_k;
}

// returns an index between 0 and nCk - 1 from a combination
// always expects the combination to be in an ascending order
pub fn ind_nck(comb: &Vec<u8>) -> usize {
    let k = comb.len();

    let mut rank = 0;
    for i in (1..=k).rev() {
        rank += nck(comb[i-1] as usize, i);
    }
    return rank;
}

// // calculates nPk
// pub fn npk(n: u8, k: u8) -> u32 {
//     if n < k {
//         return 0;
//     }
//     if n == k {
//         return 1;
//     }
//     return FACTORIAL[n as usize] / FACTORIAL[(n - k) as usize];
// }
