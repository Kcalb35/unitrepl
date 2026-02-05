use rustyline::DefaultEditor;
use rustyline::Result;
use rustyline::error::ReadlineError;

pub fn run_repl()->Result<()>{
    println!("Welcome to the unit repl!");
    let mut rl = DefaultEditor::new()?;
    loop {
        match rl.readline("unit> "){
            Ok(line)=>{
                let line = line.trim();
                if line.is_empty(){
                    continue;
                }
                rl.add_history_entry(line)?;
                match line {
                    "exit"|"quit"=>break,
                    "help"=>{
                        println!("Help");
                    }
                    _=>{
                        println!("{}", line);
                    }
                }
            }
            Err(ReadlineError::Eof)=>break,
            Err(ReadlineError::Interrupted)=>{
                println!("^C");
                continue;
            }
            Err(e)=>{
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
