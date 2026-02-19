use crate::convert::convert;
use crate::parse::parse_expr;
use clap::Parser;
#[derive(Debug, Parser)]
#[command(
    name = "unitrepl",
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct Cli {
    pub expr: Option<String>,
}

pub fn run_once(line: &str) {
    match parse_expr(line) {
        Ok(expr) => {
            println!("{}", convert(&expr));
        }
        Err(e) => {
            println!("{e}");
        }
    }
}
