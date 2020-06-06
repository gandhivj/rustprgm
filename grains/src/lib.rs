pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => unsafe_square(s),
        _ => panic!("Square must be between 1 and 64")
    }
}

pub fn unsafe_square(s: u32) -> u64 {
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
