pub fn or(item_1: u8, item_2: u8) -> u8 {
    item_1 | item_2
}

pub fn and(item_1: u8, item_2: u8) -> u8 {
    item_1 & item_2
}

pub fn xor(item_1: u8, item_2: u8) -> u8 {
    item_1 ^ item_2
}

pub fn nand(item_1: u8, item_2: u8) -> u8 {
    !(and(item_1, item_2))
}

pub fn nor(item_1: u8, item_2: u8) -> u8 {
    !(or(item_1, item_2))
}

pub fn nxor(item_1: u8, item_2: u8) -> u8 {
    !(xor(item_1, item_2))
}
