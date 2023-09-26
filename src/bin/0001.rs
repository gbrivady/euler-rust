fn main() {
    let mut sum: u64 = 0;
    for i in 0..334 {
        sum += 3 * i;
    }
    for j in 0..200 {
        if j % 3 != 0 {
            sum += 5 * j;
        }
    }
    print!("{}", sum);
}
