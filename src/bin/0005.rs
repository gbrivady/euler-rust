const PRIMES: [u64; 30] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113,
];

fn brute_force() {
    // We only need to check if it is divisible by all primes <= 20, and their power that are <= 20 (4 and 8 and 16, 9).
    let dividers: Vec<u64> = vec![2, 3, 4, 5, 7, 9, 11, 13, 16, 17, 19];
    // let dividers: Vec<u64> = vec![2, 3, 4, 5, 7, 8, 9];

    for i in 21u64..(dividers.iter().fold(1u64, |acc, n| acc * n)) {
        let mut d = dividers.iter();
        if loop {
            match d.next() {
                Some(d) => {
                    if i % d != 0 {
                        break false;
                    }
                }
                None => break true,
            }
        } {
            println!("Found {}, checking reminders:", i);
            for d in 1..20 {
                print!("{} ", i % d);
            }
            return;
        }
    }
}

fn ulogn(n: &u64, m: &u64) -> u64 {
    if *m == 0 {
        panic!("log of 0 is undefined");
    }
    let mut log: u64 = 0;
    let mut n_pow = 1;
    while n_pow < *m {
        log += 1;
        n_pow *= n;
    }
    return log - (!(n_pow == *m) as u64);
}

fn smart_force(max_divider: &u64) -> u64 {
    let mut acc: u64 = 1;

    for p in PRIMES.iter().take_while(|&p| p <= max_divider) {
        acc *= p.pow(ulogn(p, max_divider) as u32);
    }
    return acc;
}

fn main() {
    println!("Brute force results: ");
    brute_force();
    println!(
        "\nLess dumb version, for 10: {} and 20: {}",
        smart_force(&10),
        smart_force(&20)
    );
}
