// calculates nCk
pub fn nck(n: u8, k: u8) -> u32 {
    if n < k {
        return 0;
    }
    if n == k {
        return 1;
    }
    let mut new_n: u64 = n as u64;
    let mut new_k: u64 = k as u64;

    for i in (n-(k-1))..n {
        new_n *= i as u64;
        new_k *= (n-i) as u64;
    }
    return (new_n / new_k) as u32;
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
