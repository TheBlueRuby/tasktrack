use clap::Parser;

pub mod operation;
use crate::operation::Operation;

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
            Operation::List => todo!(),
            Operation::Show(read_args) => todo!("{:?}", read_args),
            Operation::Add(add_args) => todo!("{:?}", add_args),
            Operation::Update(update_args) => todo!("{:?}", update_args),
            Operation::Remove(read_args) => todo!("{:?}", read_args),
        },
    }
}
