// main.rs

use std::io;

fn main()
{
    println!("Hello, {name} world!!\nThis is {ln} and {bs}!\nProject version = {prv}",
                name = "hellot",
                ln = "Rust",
                bs = "Cargo",
                prv = "1.0.0");

    let mut guess = String::new();

    loop {
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let String q = "q";
    match guess {
        q =>  break,
    }
    }
}
