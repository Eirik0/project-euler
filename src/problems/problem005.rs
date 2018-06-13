use std::collections::HashMap;

pub fn solve() -> u64 {
    let all_divisors = [
        vec![1],
        vec![2],
        vec![3],
        vec![2, 2],
        vec![5],
        vec![2, 3],
        vec![7],
        vec![2, 2, 2],
        vec![3, 3],
        vec![2, 5],
        vec![11],
        vec![2, 2, 3],
        vec![13],
        vec![2, 7],
        vec![3, 5],
        vec![2, 2, 2, 2],
        vec![17],
        vec![2, 2, 3],
        vec![19],
        vec![2, 2, 5]
    ];
    
    let mut divisors = HashMap::new();

    for div_list in all_divisors.into_iter() {
        let mut count_map = HashMap::new();
        for divisor in div_list.into_iter() {
            let count = count_map.entry(divisor).or_insert(0);
            *count += 1;
        }
        for (key, value) in count_map.into_iter() {
            let count = divisors.entry(key).or_insert(value);
            if value > *count {
                *count = value;
            }
        }
    }

    let mut product = 1;
    for (key, value) in divisors.into_iter() {
        for _ in 0..value {
            product *= key;
        }
    }
    product
}