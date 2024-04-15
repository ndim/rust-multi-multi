fn main() {
    println!(
        "{}: Hello, world! crate={}",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_CRATE_NAME")
    );
}
