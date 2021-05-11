fn main() {
    let perm: Vec<u8> = vec![0, 1, 2, 3];
    println!("Index of {:?} is {}", perm, poker_odds::index::ind_perm(&perm));
}
