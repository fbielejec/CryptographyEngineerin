pub mod chapter_10 {

    pub fn extended_euclidean_inner(a: i32, b: i32) -> (i32, i32, i32) {
        let (gcd, x, y) = extended_euclidean(b, a % b);
        (gcd, y, x - (a / b) * y)
    }

    pub fn extended_euclidean(a: i32, b: i32) -> (i32, i32, i32) {
        if b == 0 {
            return (a, 1, 0);
        }

        let (gcd, mut u, _) = extended_euclidean_inner(a, b);

        // Part below ensures u is a positive number
        // as it's cleaner when finding the modulo inverse
        while u < 0 {
            u += b / gcd;
        }

        // use equation below to find v such that:
        // a*u + b*v = gcd
        let v = (gcd - a * u) / b;
        (gcd, u, v)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // find modularInverse(a, m) meaning find x' such that (ax' = 1 mod m)
        //
        // (gcd, u, v) = extendedEuclid(a, m)
        // so we get gcd and u and v satisfying the bezouts identity
        // if gcd != 1 then a and m are not relatively prime and the modular inverse does not exist
        // if gcd  = 1 then the inverse a^{-1} = u (mod m)
        #[test]
        fn test_extended_euclid_1() {
            let a = 3;
            let m = 11;

            let (gcd, x, _y) = extended_euclidean(a, m);

            assert!(gcd == 1);
            assert_eq!(4, x % m); // 3*4 mod 11 = 1
        }

        #[test]
        fn test_extended_euclid_2() {
            let a = 397;
            let m = 2357;

            let (gcd, u, v) = extended_euclidean(a, m);

            // au + mv = gcd(a, m)
            assert_eq!(gcd, a * u + m * v);

            assert!(gcd == 1);

            // assert_eq!(extended_euclidean(397, 2357), (1, -754, 127));

            assert_eq!(1, (a * u) % m);
        }

        #[test]
        fn test_extended_euclid_3() {
            let a = 1583;
            let m = 7918;

            let (gcd, u, v) = extended_euclidean(a, m);

            assert_eq!(gcd, a * u + m * v);

            assert!(gcd == 1);

            assert_eq!(1, (a * u) % m);
        }
    }
}
