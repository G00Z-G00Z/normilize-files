use clap::Parser;

/// Normilizes to snake case
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path
    pub path: String,

    /// Is the transform recursive
    #[arg(short, long)]
    pub recursive: bool,
}
