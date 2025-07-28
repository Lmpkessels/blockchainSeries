fn checker(amount: u32) -> bool {
    if amount % 3 == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_true_when_remainder_of_modular_is_0() {
        let input = checker(9);
        let expected = true;

        assert_eq!((input), (expected));
    }

    #[test]
    fn returns_false_when_ramainder_of_modular_is_not_0() {
        let input = checker(11);
        let expected = false;

        assert_eq!((input), (expected));
    }
}