// use crate::stuff::print_stuff;

mod stuff;
use stuff::print_stuff;

fn main() {
    println!("{}: Hello, world!", env!("CARGO_BIN_NAME"));
    print_stuff();
}
