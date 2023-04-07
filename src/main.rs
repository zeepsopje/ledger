mod entry;
mod ledger;

use ledger::Ledger;
use entry::{Entry, Value};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
}

fn main() {
    let args = Args::parse();
}
