fn main() {
    let name: &str = "my name";

    let binary = 0b1111_0000;
    let octet = 0o77;
    let decimal = 1_000_000;
    let hexadecimal = 0xff;
    let byte: u8 = b'A';

    let x: f64 = 100.234;
    println!("x is {}", x);

    let f: bool = true;
    println!("{}", f);

    let c: char = 'A';

    let s = "Hello Rust world.";
    println!("{}", s);

    let dog = "DOG";
    let cat = "CAT";
    println!("{} and {}", dog, cat);

    let s = String::from("Hello Rust world.");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = "Rust";
    let s3 = String::from("world.");

    // let s = s1 + " " + &s2 + " " + &s3;
    // println!("{}", s);
    let s = format!("{} {} {}", s1, s2, s3);
    println!("{}", s);
}

fn add(x: i32, y: i32) -> i32 {
    println!("call add");
    x + y
}
