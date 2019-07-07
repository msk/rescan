use nom::{
    bytes::complete::{is_not, tag},
    character::complete::char,
    sequence::delimited,
    IResult,
};

use crate::parser::ParseMode;
use crate::util::compile_error::{CompileError, ErrorKind};

/// Parses a control verb occurring at the beginning of a pattern.
pub(crate) fn read_control_verbs<'a>(
    input: &'a str,
    start: usize,
    mode: &mut ParseMode,
) -> Result<&'a str, CompileError> {
    match control_verb(input) {
        Ok((input, verb)) => {
            match verb {
                "UCP" => {
                    mode.ucp = true;
                }
                "UTF8" | "UTF" => {
                    mode.utf8 = true;
                }
                "NO_AUTO_POSSESS" | "NO_START_OPT" | "UTF16" | "UTF32" | "CR" | "LF" | "CRLF"
                | "ANYCRLF" | "ANY" | "BSR_ANYCRLF" | "BSR_UNICODE" => {
                    return Err(CompileError::new(
                        ErrorKind::LocatedParse,
                        format!("Unsupported control verb {} at index {}", verb, start + 2),
                    ));
                }
                verb => {
                    if verb.starts_with("LIMIT_MATCH=") || verb.starts_with("LIMIT_RECURSION=") {
                        return Err(CompileError::new(
                            ErrorKind::LocatedParse,
                            format!("Unsupported control verb {} at index {}", verb, start + 2),
                        ));
                    }
                    return Err(CompileError::new(
                        ErrorKind::LocatedParse,
                        format!("Unknown control verb {} at index {}", verb, start + 2),
                    ));
                }
            }
            Ok(input)
        }
        Err(_) => Ok(input),
    }
}

fn control_verb(input: &str) -> IResult<&str, &str> {
    delimited(tag("(*"), is_not(")"), char(')'))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_control_verbs_supported() {
        let mut mode = ParseMode::default();
        assert!(!mode.ucp);
        let rest = read_control_verbs(&"(*UCP)test"[..], 0, &mut mode);
        assert!(mode.ucp);
        assert_eq!(rest.expect("invalid test"), "test");
        assert!(!mode.utf8);
        let rest = read_control_verbs(&"(*UTF)test"[..], 0, &mut mode);
        assert!(mode.utf8);
        assert_eq!(rest.expect("invalid test"), "test");
    }

    #[test]
    fn read_control_verbs_unsupported() {
        let mut mode = ParseMode::default();
        let rest = read_control_verbs(&"(*ANY)test"[..], 0, &mut mode);
        assert!(rest
            .expect_err("invalid test")
            .reason
            .starts_with("Unsupported"));
        let rest = read_control_verbs(&"(*LIMIT_MATCH=3)test"[..], 0, &mut mode);
        assert!(rest
            .expect_err("invalid test")
            .reason
            .starts_with("Unsupported"));
    }

    #[test]
    fn read_control_verbs_unknown() {
        let mut mode = ParseMode::default();
        let rest = read_control_verbs(&"(*XXX)test"[..], 0, &mut mode);
        assert!(rest
            .expect_err("invalid test")
            .reason
            .starts_with("Unknown"));
    }
}
