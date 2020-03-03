mod chap1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_mod_3_5_test() {
        assert_eq!(chap1::sum_mod_3_5(16), 3 + 5 + 6 + 9 + 10 + 12 + 15);
    }

    #[test]
    fn gcd_test() {
        assert_eq!(chap1::gcd(3, 5), 1);
        assert_eq!(chap1::gcd(48, 40), 8);
    }
}
