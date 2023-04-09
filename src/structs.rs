#[derive(Debug)]
struct User {
    name: String,
    num: u32,
}

fn main() {
    let u1 = User {
        name: String::from("jeden"),
        num: 10,
    };

    let u2 = User {
        name: String::from("dwa"),
        ..u1
    };

    println!("{:?}", u1);
    println!("{:?}", u2);
}
