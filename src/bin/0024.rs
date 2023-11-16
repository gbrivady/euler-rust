const INPUT_ALPHABET: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn next_num(w: Vec<u32>, a: Vec<u32>, nth: &mut u32) -> bool {
    if a.len() == 0 {
        *nth += 1;
        if *nth == 1000000 {
            for j in w {
                print!("{j}");
            }
            println!();
            return true;
        }
        return false;
    }

    for (i, d) in a.iter().enumerate() {
        let mut cur_a = a.clone();
        cur_a.remove(i);
        if next_num([w.clone(), vec![*d]].concat(), cur_a, nth) {
            return true;
        }
    }
    return false;
}
fn main() {
    next_num(Vec::<u32>::new(), INPUT_ALPHABET.to_vec(), &mut 0);
}
