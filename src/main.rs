mod args;
mod commands;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    args.command.execute();
}
