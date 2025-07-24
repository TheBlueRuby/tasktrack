use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Operation {
    List,
    Show(ReadArgs),
    Add(AddArgs),
    Update(UpdateArgs),
    Remove(ReadArgs),
}

#[derive(Args, Debug)]
pub struct ReadArgs {
    #[arg(short, long)]
    id: String,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[arg(short, long)]
    id: String,

    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    description: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateArgs {
    #[arg(short, long)]
    id: String,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    description: Option<String>,
}