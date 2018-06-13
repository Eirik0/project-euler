pub fn solve() -> u64 {
    let mut max = 0;
    for i in (100..1000).rev() {
        if max / i > 1000 {
            continue;
        }
        for j in (100..i).rev() {
            let product = i * j;
            if product > max && is_palendrome(product) {
                max = product;
            }
        }
    }
    max
}

fn is_palendrome(n: u64) -> bool {
    let mut next_pow_ten = 10;
    let mut num_digits = 1;
    while next_pow_ten < n {
        next_pow_ten *= 10;
        num_digits += 1;
    }

    let mut digits = Vec::with_capacity(num_digits);

    let mut n = n;

    for _ in 0..num_digits {
        digits.push(n % 10);
        n = n / 10;
    }


    for i in 0..(num_digits >> 1) {
        if &digits[i] != &digits[num_digits - i - 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_is_palendrome() {
        assert!(is_palendrome(1));
        assert!(is_palendrome(11));
        assert!(is_palendrome(121));
        assert!(is_palendrome(1221));
        assert!(is_palendrome(12321));
        assert!(!is_palendrome(12));
        assert!(!is_palendrome(123));
        assert!(!is_palendrome(1234));
        assert!(!is_palendrome(12345));
    }
}