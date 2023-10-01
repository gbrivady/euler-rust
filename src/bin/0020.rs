fn main() {
    let n: usize = 200;

    let mut result: Vec<u64> = vec![0; n];

    result[n - 1] = 1;

    for i in 1..=100 {
        let mut carry: u64 = 0;
        for j in (0..n).rev() {
            let r = result[j] * i + carry;
            result[j] = r % 10;
            carry = r / 10;
        }
    }

    println!("{}", result.iter().sum::<u64>());
}
