pub(crate) fn myisupper(c: u8) -> bool {
    (c >= b'A') && (c <= b'Z')
}

pub(crate) fn myislower(c: u8) -> bool {
    (c >= b'a') && (c <= b'z')
}

pub(crate) fn mytolower(c: u8) -> u8 {
    if myisupper(c) {
        c + 0x20
    } else {
        c
    }
}

pub(crate) fn mytoupper(c: u8) -> u8 {
    if myislower(c) {
        c - 0x20
    } else {
        c
    }
}

/// Returns `true` if this character hash different uppercase and lowercase forms.
pub(crate) fn ourisalpha(c: u8) -> bool {
    mytolower(c) != mytoupper(c)
}
