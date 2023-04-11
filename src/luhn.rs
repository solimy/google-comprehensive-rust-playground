#![allow(unused_variables, dead_code)]
pub fn luhn(cc_number: &str) -> bool {
    let mut digits = 0;
    let mut total = 0;
    for character in cc_number.chars().rev() {
        if character.is_digit(10) {
            let digit = character.to_digit(10).unwrap();
            if digits % 2 == 1 {
                let doubled = digit * 2;
                if doubled > 9 {
                    total += doubled - 9;
                } else {
                    total += doubled;
                }
            } else {
                total += digit;
            }
            digits += 1;
        } else if !character.is_whitespace() {
            return false;
        }
    }
    if digits < 2 {
        return false;
    }
    total % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
