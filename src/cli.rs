use crate::error::AppError;
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

pub fn run_once(line: &str) -> Result<(), AppError> {
    println!("Running once with line: {}", line);
    Ok(())
}
