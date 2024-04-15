pub fn print_stuff() {
    println!(
        "  stuff: printing stuff: crate={}",
        env!("CARGO_CRATE_NAME")
    );
}
