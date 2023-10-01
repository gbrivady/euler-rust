fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let w: u64 = args[1].parse().unwrap();
    let h: u64 = args[2].parse().unwrap();
    // We need to compute m among m+n
    let mut old_coef: Vec<u64> = Vec::new();
    for n in 0..=(w + h) {
        let mut new_coef: Vec<u64> = Vec::new();
        for k in 0..=std::cmp::min(n, w) {
            if k == 0 || k == n {
                new_coef.push(1);
            } else {
                new_coef.push(old_coef[(k - 1) as usize] + old_coef[k as usize]);
            }
        }
        old_coef = new_coef;
    }
    println!("{}", old_coef[w as usize]);
}
