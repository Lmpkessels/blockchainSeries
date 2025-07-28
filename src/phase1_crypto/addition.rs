fn addition<const N: usize>(a: [u32; N]) -> u32 {
    let mut result = 0;
    
    for i in 0..N {
        result += a[i];
    }

    result
}

fn bitmix32(x: u64, y: u64, z: u64, w: u64) -> u64 {
    const MOD: u64 = 1u64 << 32;

    let left = (x >> 2) & (y << 5);
    let right = (z ^ w) + (x & y);

    (left & right) % MOD
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn goes_digit_by_digit_adds_up_to_a_whole_and_returns_result() {
        let a = [21, 11, 12, 44, 81];
        let result = addition(a);
        let expected = 169;

        assert_eq!((result), (expected));
    }

    #[test]
    fn returns_modulo_after_going_through_logic() {
        let x = 33u64;
        let y = 1231u64;
        let z = 123u64;
        let w = 1122u64;
        
        let left = (x >> 2) & (y << 5);
        let right = (x ^ w) + (x & y);
        
        let result = low(x, y, z, w);
        let expected = left & right;
        
        assert_eq!((result), (expected));
    }
}