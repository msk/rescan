//! Dump code for character classes (expressed as `CharReach` objects).

use rescan_util::CharReach;
use std::fmt::{Error, Write};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CcOutput {
    /// Unescaped text output
    Text,
    /// Escaped DOT label output
    Dot,
}

// These characters must always be escaped.
const ESCAPED: [u8; 5] = *b"^-[].";
const DOT_SINGLE_ESCAPED: [u8; 2] = *br#""'"#;

fn describe_char<W: Write>(f: &mut W, c: u8, out_type: CcOutput) -> Result<(), Error> {
    let backslash = if out_type == CcOutput::Dot {
        r#"\\"#
    } else {
        r#"\"#
    };

    if 0x21 <= c && c < 0x7F && c != b'\\' {
        if ESCAPED.contains(&c) {
            f.write_str(backslash)?;
            f.write_char(c.into())
        } else if out_type == CcOutput::Dot && DOT_SINGLE_ESCAPED.contains(&c) {
            f.write_char('\\')?;
            f.write_char(c.into())
        } else {
            f.write_char(c.into())
        }
    } else if c == 0x09 {
        f.write_str(backslash)?;
        f.write_char('t')
    } else if c == 0x0a {
        f.write_str(backslash)?;
        f.write_char('n')
    } else if c == 0x0d {
        f.write_str(backslash)?;
        f.write_char('r')
    } else {
        f.write_str(&format!("{}x{:2x}", backslash, c))
    }
}

/// Prints a contiguous range of byte characters.
///
/// # Panics
///
/// Panics if `c1 > c2`.
fn describe_range<W: Write>(f: &mut W, c1: u8, c2: u8, out_type: CcOutput) -> Result<(), Error> {
    if c1 == c2 {
        describe_char(f, c1, out_type)
    } else if c2 - c1 < 4 {
        // Render as individual chars.
        let mut c1 = c1;
        loop {
            describe_char(f, c1, out_type)?;
            c1 += 1;
            if c1 > c2 {
                break;
            }
        }
        Ok(())
    } else {
        // Range
        describe_char(f, c1, out_type)?;
        f.write_char('-')?;
        describe_char(f, c2, out_type)
    }
}

fn extract_mnemonic<W: Write>(
    f: &mut W,
    cr: &CharReach,
    out_type: CcOutput,
) -> Result<Option<CharReach>, Error> {
    let backslash = if out_type == CcOutput::Dot {
        r#"\\"#
    } else {
        r#"\"#
    };

    // \w (word characters: any letter, digit, or underscore)
    let words =
        CharReach::from_bytes(b"_0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWYXZ");
    if words.is_subset_of(cr) {
        f.write_str(backslash)?;
        f.write_char('w')?;
        return Ok(Some(*cr & !words));
    }

    // \d (digits)
    let digits = CharReach::from_bytes(b"0123456789");
    if digits.is_subset_of(cr) {
        f.write_str(backslash)?;
        f.write_char('d')?;
        return Ok(Some(*cr & !digits));
    }

    // \s (whitespace)
    let whitespace = CharReach::from_bytes(b"\x09\x0a\x0b\x0c\x0d\x20");
    if whitespace.is_subset_of(cr) {
        f.write_str(backslash)?;
        f.write_char('s')?;
        return Ok(Some(*cr & !whitespace));
    }

    Ok(None)
}

fn contiguous_range(cr: &CharReach) -> Option<(u8, u8)> {
    let first = if let Some(pos) = cr.find_first() {
        pos
    } else {
        return None;
    };
    let mut last = first;
    while let Some(c) = cr.find_next(last) {
        if c != last + 1 {
            return None;
        }
        last = c;
    }
    Some((first, last))
}

fn describe_class_int<W: Write>(
    f: &mut W,
    incr: &CharReach,
    max_length: usize,
    out_type: CcOutput,
) -> Result<usize, Error> {
    // Approx size of output
    let mut i = 0;
    // One we can break
    let mut cr = incr.clone();

    // If we can be rendered as a single range, do it.
    if let Some((first, last)) = contiguous_range(&cr) {
        describe_range(f, first, last, out_type)?;
        return Ok(2);
    }

    // Extract any mnemonics.
    while let Some(remaining) = extract_mnemonic(f, &cr, out_type)? {
        i += 1;
        if i == max_length {
            f.write_str("...]")?;
            return Ok(max_length);
        }
        cr = remaining;
    }

    if cr.none() {
        // all mnemonics
        return Ok(i);
    }

    // Render character class as a series of ranges.
    if let Some(mut c_start) = cr.find_first() {
        let mut c_last = c_start;
        while let Some(c) = cr.find_next(c_last) {
            if c != c_last + 1 {
                describe_range(f, c_start, c_last, out_type)?;
                c_start = c;
                i += 1;
                if i == max_length {
                    f.write_str("...]")?;
                    return Ok(max_length);
                }
            }
            c_last = c;
        }
        if c_last == 0xff {
            describe_range(f, c_start, c_last, out_type)?;
            i += 1;
        }
    }
    Ok(i)
}

pub fn describe_class<W: Write>(
    f: &mut W,
    incr: &CharReach,
    max_length: usize,
    out_type: CcOutput,
) -> Result<(), Error> {
    if incr.all() {
        return f.write_str("<any>");
    }

    if incr.none() {
        return f.write_str("<empty>");
    }

    if incr.count() == 1 {
        return describe_char(f, incr.find_first().expect("should be available"), out_type);
    }
    if (!*incr).count() == 1 {
        f.write_str("[^")?;
        describe_char(
            f,
            (!*incr).find_first().expect("should be available"),
            out_type,
        )?;
        return f.write_char(']');
    }

    // build up a normal string and a negated one, and see which is shorter
    let mut out = String::new();
    let out_count = describe_class_int(&mut out, incr, max_length, out_type)?;

    let mut neg = String::new();
    describe_class_int(&mut neg, &!*incr, max_length, out_type)?;

    if out.len() <= neg.len() {
        if out_count > 1 {
            f.write_char('[')?;
            f.write_str(&out)?;
            f.write_char(']')
        } else {
            f.write_str(&out)
        }
    } else {
        f.write_str(&format!("[^{}]", neg))
    }
}
