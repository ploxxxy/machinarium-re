fn get_level_hash(input: &str) -> u32 {
    let mut hash: u32 = 0;
    let mut index: u32 = 1;

    for byte in input.bytes() {
        let shift = index & 0x1F;
        index += 1;

        let hash_component = (byte as u32) + ((byte as u32) << shift);
        hash += hash_component;
    }

    hash + index
}

fn main() {
    let string = std::env::args().nth(1).expect("no string given");
    let hash = get_level_hash(&string);

    print!("{}.jpg", hash)
}
