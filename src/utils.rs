pub fn round_down(x: u64, k: u64) -> u64 {
    x & !(k - 1)
}

pub fn round_up(x: u64, k: u64) -> u64 {
    (x + (k - 1)) & !(k - 1)
}
