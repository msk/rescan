pub fn myisupper(c: u8) -> bool {
    (c >= b'A') && (c <= b'Z')
}

pub fn myislower(c: u8) -> bool {
    (c >= b'a') && (c <= b'z')
}

pub fn mytolower(c: u8) -> u8 {
    if myisupper(c) {
        c + 0x20
    } else {
        c
    }
}

pub fn mytoupper(c: u8) -> u8 {
    if myislower(c) {
        c - 0x20
    } else {
        c
    }
}

/// Returns `true` if this character hash different uppercase and lowercase forms.
pub fn ourisalpha(c: u8) -> bool {
    mytolower(c) != mytoupper(c)
}
