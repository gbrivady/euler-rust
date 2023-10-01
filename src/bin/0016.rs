fn main() {
    let power: u64 = 1000;
    let n: usize = (1f32 + (power as f32) * 2f32.log10()).floor() as usize;

    let mut result: Vec<u64> = vec![0; n];

    result[n - 1] = 1;

    for _ in 1..=power {
        let mut carry: u64 = 0;
        for j in (0..n).rev() {
            let r = result[j] * 2 + carry;
            result[j] = r % 10;
            carry = r / 10;
        }
    }

    println!("{}", result.iter().sum::<u64>());
}
