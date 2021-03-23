pub fn myisupper(c: u8) -> bool {
    (b'A'..=b'Z').contains(&c)
}

pub fn myislower(c: u8) -> bool {
    (b'a'..=b'z').contains(&c)
}

#[must_use]
pub fn mytolower(c: u8) -> u8 {
    if myisupper(c) {
        c + 0x20
    } else {
        c
    }
}

#[must_use]
pub fn mytoupper(c: u8) -> u8 {
    if myislower(c) {
        c - 0x20
    } else {
        c
    }
}

/// Returns `true` if this character hash different uppercase and lowercase forms.
#[must_use]
pub fn ourisalpha(c: u8) -> bool {
    mytolower(c) != mytoupper(c)
}
