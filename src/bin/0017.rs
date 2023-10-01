const UNITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const SPEC_TENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const DEC: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

fn integer_to_english(n: u64) -> String {
    if n == 1000 {
        return String::from("onethousand");
    }
    match (n / 100, (n % 100) / 10, n % 10) {
        (0, 0, c) => String::from(UNITS[(c - 1) as usize]),
        (0, 1, c) => String::from(SPEC_TENS[c as usize]),
        (0, b, 0) => String::from(DEC[(b - 2) as usize]),
        (0, b, c) => String::from(DEC[(b - 2) as usize]) + &String::from(UNITS[(c - 1) as usize]),
        (a, 0, 0) => String::from(UNITS[(a - 1) as usize]) + &String::from("hundred"),
        (a, _, _) => {
            String::from(UNITS[(a - 1) as usize])
                + &String::from("hundredand")
                + &integer_to_english(n % 100)
        }
    }
}

fn main() {
    let mut sum_letters: u64 = 0;

    for i in 1..=1000 {
        sum_letters += integer_to_english(i).len() as u64;
    }
    println!("{sum_letters}")
}
