use chrono::prelude::*;
use std::io;

fn main() {
    println!("Which day do we have today?");
    let mut day = String::new();
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read froms standard in");
    println!(
        "You have said: {}The correct answer is {:#?}",
        day,
        Local::now().weekday()
    );
}
