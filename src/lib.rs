/// Print reversed number removed zero if it starts with.
pub fn reverse_number(_given_number :i32) -> i32 {
    _given_number
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
