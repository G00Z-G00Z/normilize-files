use clap::Parser;

/// Normilizes to snake case
#[derive(Parser, Debug)]
#[command(author = "G00Z-G00Z", version, about, long_about = None)]
pub struct Args {
    /// Path
    pub path: String,

}
