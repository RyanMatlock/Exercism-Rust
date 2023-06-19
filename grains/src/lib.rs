const BOARD_MIN: u32 = 1;
const BOARD_MAX: u32 = 64;

pub fn square(s: u32) -> u64 {
    const BASE: u64 = 2;

    if BOARD_MIN <= s && s <= BOARD_MAX {
        BASE.pow((s - 1).into())
    } else {
        panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    let board: Vec<u32> = (BOARD_MIN..=BOARD_MAX).collect();
    board.iter()
        .map(|n| square(*n))
        .sum()
}
