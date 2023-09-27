fn main() {
    let input: u64 = 600851475143;
    let max_search: u64 = (input as f64).sqrt() as u64;

    let mut primes = Vec::<u64>::new();

    primes.push(2);

    for i in 3..max_search {
        let mut v = primes.iter();
        if loop {
            match v.next() {
                None => break true,
                Some(n) => {
                    if i % n == 0 {
                        break false;
                    }
                }
            }
        } {
            primes.push(i);
        }
    }
    for p in primes.iter().rev() {
        if input % p == 0 {
            println!("{}", p);
            break;
        }
    }
}
