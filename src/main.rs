use clap::Parser;
mod cli;
fn main() {
    println!("Hello, world!");
    let args = cli::Args::parse();
}
