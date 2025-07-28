fn f(x: u32) -> u32 {
    3 * x + 2
}

fn a(a: u32, x: u32) -> u32 {
    a * x + 1
}

fn x(x: u32) -> u32 {
    if x > 0 {
        x
    } else {
        0
    }
}

fn xor(x: u32, y: u32) -> u32 {
    x ^ y
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

fn f_n(x: u32, y: u32, m: u32) -> u32 {
    ((x ^ y) * 3 + (x & y) * 2) % m
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multiplies_x_by_3_then_adds_2() {
        let result = f(8);
        let expected = 26;

        assert_eq!((result), (expected));
    }

    #[test]
    fn multiplies_a_by_x_then_adds_1() {
        let result = a(16, 22);
        let expected = 353;
        
        assert_eq!((result), (expected));
    }

    #[test]
    fn receive_input_if_input_is_greater_then_0_else_0() {
        let input_0 = x(0);
        let expected_0 = 0;
        let input_32 = x(32);
        let expected_32 = 32;
        
        assert_eq!((input_0), (expected_0));
        assert_eq!((input_32), (expected_32));
    }

    #[test]
    fn receive_u32_after_applying_xor_bit_by_bit() {
        let result = xor(21, 22);
        let expected = 3;

        assert_eq!((result), (expected));
    }

    #[test]
    fn applies_and_bit_by_bit_then_xor_then_repeats() {
        let result = maj(12, 22, 11);
        let expected = 14;

        assert_eq!((result), (expected));
    }

    #[test]
    fn applies_XOR_on_yandz_times_2_then_AND_on_yandz_times_2_add_up_to_the_remainder_of_m() {
        let result = f_n(33, 32, 11);
        let expected = 1;

        assert_eq!((result), (expected));
    }

}