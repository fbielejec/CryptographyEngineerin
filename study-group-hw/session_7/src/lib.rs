pub fn greatest_common_divisor(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return greatest_common_divisor(b, a % b);
}

pub fn extended_euclid(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        return (a, 1, 0);
    } else {
        let (gcd, x, y) = extended_euclid(b, a % b);
        return (gcd, y, x - (a / b) * y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let a = 21i32;
        let b = 36i32;
        assert_eq!(3, greatest_common_divisor(a, b))
    }

    // find modularInverse(a, m) meaning find x' such that (ax' = 1 mod m)
    //
    // (gcd, x, y) = extendedEuclid(a, m)
    // so we get gcd and x and y satisfying the bezouts identity
    // if gcd != 1 then a and m are not relatively prime and the modular inverse does not exist
    // if gcd  = 1 then the inverse x'= x % m
    #[test]
    fn test_extended_euclid() {
        let a = 3i32;
        let m = 11i32;

        let (gcd, x, _y) = extended_euclid(a, m);

        assert!(gcd == 1);
        assert_eq!(4, x % m);
    }
}
