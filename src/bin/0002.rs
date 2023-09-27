fn main() {
    let mut u1: u64 = 1;
    let mut u2: u64 = 2;
    let mut sum: u64 = 0;

    while u1 < 4000000 {
        if u1 % 2 == 0 {
            sum += u1;
        }
        let temp = u1;
        u1 = u2;
        u2 += temp;
    }
    println!("{}", sum);
}
