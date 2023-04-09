use rand::Rng;
use std::io;

fn main() {
    println!("Guess then number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {secret_number}");

    loop {
        println!("Pick number: ");
        // guess must be new string each time, because read_line appends to extisings string
        let mut guess = String::new();
        // throws error (panic)
        io::stdin().read_line(&mut guess).expect("Error!");

        // ResulfF = MonadError?
        let guess1: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_parse_int_error) => {
                println!("Not a number!");
                continue;
            }
        };

        match guess1.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too little!"),
            std::cmp::Ordering::Equal => {
                println!("Spot on!");
                break;
            }
            std::cmp::Ordering::Greater => println!("To much!"),
        }
    }
}
