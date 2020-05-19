/// Print reversed number removed zero if it starts with.
pub fn reverse_number(mut given_number :i32) -> i32 {
    let mut currnet_digit;
    let mut result = 0;

    while given_number > 0 {
        currnet_digit = given_number % 10;
        if currnet_digit != 0 {
            result = result * 10 + currnet_digit;
        } else if currnet_digit == 0 && result != 0 {
            // add zero if it's in middle of result.
            result = result * 10 + currnet_digit;
        }
        given_number = given_number / 10;
    }

    result
}

#[cfg(test)]
mod tests {
    // import all of pub items in this file
    use super::*;

    #[test]
    fn no_zero() {
        assert_eq!(reverse_number(123), 321);
    }

    #[test]
    fn zero_at_tail() {
        assert_eq!(reverse_number(1230), 321);
    }

    #[test]
    fn zero_at_tail_consecutively() {
        assert_eq!(reverse_number(12300), 321);
    }

    #[test]
    fn zero_at_middle() {
        assert_eq!(reverse_number(204), 402);
    }

    #[test]
    fn only_zero() {
        assert_eq!(reverse_number(0), 0);
    }

    #[test]
    fn only_none_zero() {
        assert_eq!(reverse_number(8), 8);
    }
}
