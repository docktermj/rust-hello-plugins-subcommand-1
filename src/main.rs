use rust_hello_plugins_subcommand_1::execute;

fn main() {
    println!("Hello, world!");
    if let Err(e) = execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
