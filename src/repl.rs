use crate::constants::*;
use crate::convert::convert;
use crate::parse::ConversionExpr;
use crate::parse::parse_expr;
use rustyline::DefaultEditor;
use rustyline::Result;
use rustyline::error::ReadlineError;

fn print_help() {
    println!(
        r#"Welcome to the unit repl!
Supported commands:
exit|quit:   exit the repl
help:        print this help message
list:        list the supported basic units
<expr>:      convert the expression

Examples:
10 km to m"#
    );
}

pub fn print_units_grouped(arg: Option<&str>) {
    fn print_group(title: &str, units: &[(&'static str, f64)]) {
        let mut buffer = ryu::Buffer::new();
        println!("[{}] ", title);
        for &(name, factor) in units {
            println!("{} {}", name, buffer.format(factor));
        }
    }
    match arg {
        None => {
            for (&k, &v) in UNIT_GROUP_MAP.iter() {
                print_group(k, v);
            }
        }
        Some(t) => {
            let key = t.trim().to_lowercase();
            match UNIT_GROUP_MAP.get(key.as_str()) {
                None => {
                    println!("Unknown group: {key}");
                }
                Some(&units) => {
                    print_group(&key, units);
                }
            }
        }
    }
}

enum ReplCmd<'a> {
    Help,
    Exit,
    List(Option<&'a str>),
    Expr(ConversionExpr),
    Invalid(String),
    Empty,
}

fn parse_repl(line: &str) -> ReplCmd<'_> {
    let line = line.trim();
    if line.is_empty() {
        return ReplCmd::Empty;
    }
    let mut it = line.split_whitespace();
    let head = it.next().unwrap();
    match head {
        "help" => ReplCmd::Help,
        "list" => {
            let arg = it.next();
            if it.next().is_some() {
                return ReplCmd::Invalid(String::from("usage: list [group]"));
            }
            ReplCmd::List(arg)
        }
        "exit" | "quit" => ReplCmd::Exit,
        _ => match parse_expr(line) {
            Ok(expr) => ReplCmd::Expr(expr),
            Err(e) => ReplCmd::Invalid(e.format_repl()),
        },
    }
}

pub fn run_repl() -> Result<()> {
    use ReplCmd::*;
    println!("Welcome to the unit repl!");
    let mut rl = DefaultEditor::new()?;
    loop {
        match rl.readline("unit> ") {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line)?;
                match parse_repl(line) {
                    Help => {
                        print_help();
                    }
                    List(arg) => {
                        print_units_grouped(arg);
                    }
                    Exit => {
                        break;
                    }
                    Expr(expr) => {
                        println!("{}", convert(&expr));
                    }
                    Invalid(msg) => {
                        println!("{}", msg);
                    }
                    Empty => {
                        continue;
                    }
                }
            }
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
