use std::{
    io::{self, Write},
};
use ghostdb::frontend::parser::parse::parse_query;
use ghostdb::frontend::actions::execute_statement;
use ghostdb::backend::global::Global;

fn main() -> Result<(), io::Error> {
    
    let global = Global::new();

    run(global)?;

    Ok(())
}

fn run(mut global: Global) -> Result<(), io::Error> {
    //clear the screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    loop {
        match global.get_current_database_name() {
            Some(n) => {
                print!("\u{1F47B} ({n})> ");
            },
            None => {
                print!("\u{1F47B} > ");
            },
        }
        io::stdout().flush()?;
        
        //get user command
        let mut raw_cmd: String = String::new();
        io::stdin().read_line(&mut raw_cmd)?;
        let cmd = raw_cmd.trim_end().trim_start();

        if cmd == "q" {
            return Ok(());
        }
        
        let statement = match parse_query(cmd) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            },
        };
        //println!("{:?}", statement);
        match execute_statement(statement, &mut global) {
            Ok(s) => {
                println!("{}", s);
                //println!("{:?}", global);
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
