mod cli;
mod constants;
mod convert;
mod parse;
mod repl;
mod units;

use clap::Parser;
use cli::Cli;
use cli::run_once;

use crate::repl::run_repl;

fn main() {
    let cli = Cli::parse();
    if let Some(expr) = cli.expr {
        run_once(expr.as_str())
    } else {
        if let Err(e) = run_repl() {
            eprintln!("Error: {}", e);
        }
    }
}
