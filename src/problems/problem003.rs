pub fn solve() -> u64 {
    let n: u64 = 600851475143;
    let sqrt_n = (n as f64).sqrt() as u64;
    let mut d: u64 = 3;
    let mut prime_divisors: Vec<u64> = Vec::new();
    while d <= sqrt_n {
        if n % d == 0 {
            if prime_divisors.iter().all(|&p| (d % p != 0)) {
                prime_divisors.push(d);
            }
        }
        d += 2;
    }
    *prime_divisors.iter().last().unwrap()
}
