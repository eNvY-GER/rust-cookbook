//use statement is like an import statement in java
use chrono::prelude::*;

fn main() {
    //let is the way variables are defined in Rust
    let now: DateTime<Local> = Local::now();
    //there are different formatting strings for variables which do not implement the display trait ({} does not work)
    //:#? is the pretty print formatter while :? is the debug formatter
    println!("Hello world from Rust! It is {:#?}", now.weekday());
}
