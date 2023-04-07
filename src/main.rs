mod ledger;
mod entry;

use std::{fs, env};
use entry::EntryArg;
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
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
    let path = format!("{}/.config/ledger", env!("HOME"));
    fs::create_dir_all(&path).unwrap();
    let mut ledger = Ledger::from_file(
        &format!("{}/ledger.json", &path)
    ).unwrap_or(Ledger::new());
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

    match &cli.command {
        Some(Command::Status) => {},
        None => {},
        _ => {
            ledger.save(&format!("{}/.config/ledger/ledger.json", env!("HOME"))).expect("Couldn't save changes");
        }
    }
}
