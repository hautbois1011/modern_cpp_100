use std::cmp;

/// nまでの3または5で割り切れる数の総和を返します
pub fn sum_mod_3_5(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

/// aとbの最大公約数を返します
pub fn gcd(a: u32, b: u32) -> u32 {
    let mut m = cmp::max(a, b);
    let mut n = cmp::min(a, b);

    loop {
        if n == 0 {
            break;
        }
        let tmp = n;
        n = m % n;
        m = tmp;
    }

    m
}
