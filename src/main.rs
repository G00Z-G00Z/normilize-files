use clap::Parser;
use path_crawler::PathCrawler;
mod cli;
mod path_crawler;
mod transformations;

fn main() {
    let args = cli::Args::parse();

    let path_crawler = PathCrawler::build(transformations::to_snake_case);

    match path_crawler.apply_transformations(&args.path) {
        Ok(_) => {}
        Err(er) => {
            eprintln!("{:?}", er);
        }
    }
}
