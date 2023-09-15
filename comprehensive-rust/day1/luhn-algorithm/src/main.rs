pub fn luhn(cc_number: &str) -> bool {

    // Ignore all spaces. Reject number with less than two digits.
    let clean_cc_number = cc_number.replace(' ', "");
    if clean_cc_number.len() < 2 {
        return false
    };

    // Reject non digit
    for char in clean_cc_number.chars() {
        if !char.is_ascii_digit() {
            return false
        }
    }

    // Moving from right to left, double every second digit: for the number 1234, we double 3 and 1. For the number 98765, we double 6 and 8.
    let mut vec_cc_number: Vec<char> = vec![];
    let mut res_cc_number: Vec<u32> = vec![];

    for digit in clean_cc_number.chars() {
        vec_cc_number.push(digit)
    }

    let mut count = 0;
    while !vec_cc_number.is_empty() {
        count += 1;
        let number = vec_cc_number.last();

        match number.map(|c|c.to_digit(10).unwrap()) {
            // After doubling a digit, sum the digits if the result is greater than 9. So doubling 7 becomes 14 which becomes 1 + 4 = 5.
            Some(x) if count % 2 == 0 => {
                let mut doubled = x * 2;
                if doubled > 9 {
                    doubled = doubled % 10 + doubled / 10;
                }
                res_cc_number.push(doubled);
            },
            Some(x) if count % 2 > 0 => res_cc_number.push(x),
            _ => return false
        }

        vec_cc_number.pop();
    }

    // Sum all the undoubled and doubled digits.
    let mut sum: u32 = 0;
    for digit in res_cc_number {
        sum += digit;
    }

    // The credit card number is valid if the sum ends with 0
    if sum % 10 == 0 { return true; };

    false
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
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

#[allow(dead_code)]
fn main() {}