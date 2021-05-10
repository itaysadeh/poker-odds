// stores 0!..13!
pub const FACTORIAL: [u32; 13] = [1,1,2,6,24,120,720,5040,40320,362880,3628800,39916800,479001600];

// calculates nCk
pub fn nck(n: u8, k: u8) -> u32 {
    if n < k {
        return 0;
    }
    if n == k {
        return 1;
    }
    return FACTORIAL[n as usize] / (FACTORIAL[k as usize] * FACTORIAL[(n - k) as usize]);
}

// calculates nPk
pub fn npk(n: u8, k: u8) -> u32 {
    if n < k {
        return 0;
    }
    if n == k {
        return 1;
    }
    return FACTORIAL[n as usize] / FACTORIAL[(n - k) as usize];
}