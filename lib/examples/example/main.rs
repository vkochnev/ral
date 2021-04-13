extern crate alloc;

use crate::cpu32::peripherals::some;
use crate::cpu32::types::EnumType;
use crate::cpu32::types::EnumType::{One, Three, Two};

mod cpu32;

fn main() {
    // Do not read or write as it will result in UB due to incorrect address in example
    let mut either = some::either().unwrap();
    let either3 = either.get_either3();

    let either2 = either.is_either2_set();
    let either1: EnumType = either.get_either1().unwrap();
    either
        .set_either0(if either2 {
            either.get_either5() as u8
        } else {
            0
        })
        .unset_either2()
        .set_either4(either3 + 10)
        .set_either1(if either1 == Two { One } else { Three })
        .unwrap();
}
