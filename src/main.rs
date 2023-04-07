mod ledger;
mod entry;

use entry::EntryArg;
use clap::{Parser, Subcommand};
use ledger::Ledger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Output an overview of the ledger
    Status,
    In(EntryArg),
    Out(EntryArg),
}

fn main() {
    let mut ledger = Ledger::new();
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::In(entry)) => {
            ledger.deposit(entry.desc.as_str(), entry.value);
        },
        Some(Command::Out(entry)) => {
            ledger.withdraw(entry.desc.as_str(), entry.value);
        },
        _ => {
            dbg!(&ledger);
        }
    }
}
