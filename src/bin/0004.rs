fn is_palindrome(n: &u64) -> bool {
    let s = n.to_string();
    s.char_indices()
        .zip(s.char_indices().rev())
        .take_while(|((i, _), (j, _))| i < j)
        .all(|((_, ci), (_, cj))| ci == cj)
}

fn main() {
    // Product of two three digit number is between 10000 and 998001
    // Then there is a chance there exist a palindrome of size 6.
    // Then it would be a multiple of 11.
    // So we check all products of size 6, that contains 11.
    let mut palindromes: Vec<u64> = Vec::<u64>::new();

    for i in (110..1000).step_by(11) {
        for j in (100000u64 / i)..999u64 {
            let n: u64 = i * j;
            if is_palindrome(&n) {
                palindromes.push(n);
            }
        }
    }
    match palindromes.iter().max() {
        Some(n) => println!("{}", n),
        _ => println!("There is no 6 digit palindrome that is a product of 2 3-digits number"),
    }
}
