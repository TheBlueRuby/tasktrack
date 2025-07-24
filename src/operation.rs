use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Operation {
    List,
    Show(ReadArgs),
    Add(AddArgs),
    Update(UpdateArgs),
    Remove(ReadArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ReadArgs {
    #[arg(short, long)]
    pub id: String,
}

#[derive(Args, Debug, Clone)]
pub struct AddArgs {
    #[arg(short, long)]
    pub id: String,

    #[arg(short, long)]
    pub name: String,

    #[arg(short, long)]
    pub description: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct UpdateArgs {
    #[arg(short, long)]
    pub id: String,

    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long)]
    pub description: Option<String>,
}