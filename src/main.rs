use clap::Parser;

pub mod operation;
use crate::operation::Operation;

pub mod task;
pub mod fileio;
pub mod util;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    // operation to perform
    #[command(subcommand)]
    operation: Option<Operation>,
}

fn main() {
    let cli = Cli::parse();
    match &cli.operation {
        None => {
            println!("No operation specified. Use --help for more information.");
        }
        Some(op) => match op {
            Operation::List => task::list(),
            Operation::Show(read_args) => task::show(read_args.clone()),
            Operation::Add(add_args) => task::add(add_args.clone()),
            Operation::Update(update_args) => task::update(update_args.clone()),
            Operation::Remove(read_args) => task::remove(read_args.clone()),
            Operation::Summary => task::summary(),
        },
    }
}
