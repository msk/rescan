pub(crate) fn myisupper(c: u8) -> bool {
    (c >= b'A') && (c <= b'Z')
}

pub(crate) fn mytolower(c: u8) -> u8 {
    if myisupper(c) {
        c + 0x20
    } else {
        c
    }
}
