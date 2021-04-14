#![doc = "0x10 - Either register"]

use ral_macro::register;

register! {
    use crate::cpu32::types::EnumType;

    #[doc = "0x10 - Either register"]
    either {
        offset: 0x00,
        value_size: 32,
        reset_mask: 0xFFFF_FFFF,
        reset_value: 0x1234_0000,
        fields: {
            #[doc = "Bits 16:31 - Read-only u16 field"]
            #[access = "read-only"]
            either5[16:16] as u16,

            #[doc = "Bits 14:15 - Write-only field"]
            #[access = "write-only"]
            either4[14:2] as u8,

            #[doc = "Bits 11:13 - Read-only field"]
            #[access = "read-only"]
            either3[11:3] as u8,

            #[doc = "Bit 10 - Boolean field"]
            #[access = "read-write"]
            either2[10:1] as bool,

            #[doc = "Bits 8:9 - Enum field"]
            #[access = "read-write"]
            either1[8:2] as EnumType,

            #[doc = "Bits 0:7 - Read-write by default long field"]
            either0[0:8] as u8
        }
    }
}
