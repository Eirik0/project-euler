pub fn solve() -> u32 {
    let mut sum = 0;
    let mut f_n1 = 1;
    let mut f_n2 = 2;
    while f_n2 < 4000000 {
        if f_n2 % 2 == 0 {
            sum += f_n2
        }
        let f_n0 = f_n1;
        f_n1 = f_n2;
        f_n2 = f_n0 + f_n1;
    }
    sum
}
