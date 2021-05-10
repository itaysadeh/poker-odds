mod index;

fn main() {
    let vec: Vec<u8> = vec![0, 1, 2];
    let ind: u32 = index::perm(&vec);
    println!("index of {:?} is {}", vec, ind);
}
