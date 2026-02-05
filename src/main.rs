mod cli;
mod error;
mod repl;

use clap::Parser;
use cli::Cli;
use cli::run_once;

use crate::repl::run_repl;

fn main(){
    let cli = Cli::parse();
    if let Some(expr) = cli.expr {
        if let Err(e) = run_once(expr.as_str()) {
            eprintln!("Error: {}", e);
        }
    } else {
        if let Err(e) = run_repl() {
            eprintln!("Error: {}", e);
        }
    }
}
