#![doc = "Some peripheral"]

mod either;
pub use either::*;

// address is intentionally incorrect
const BASE_ADDRESS: usize = 0x0000_0000;
