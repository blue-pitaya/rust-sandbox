use std::io;

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");

    println!("{}", first_word(&mut input));
}
