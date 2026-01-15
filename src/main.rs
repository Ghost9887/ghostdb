use std::{
    io::{self, Write},
};
use ghostdb::frontend::parser::parse::parse_query;
use ghostdb::frontend::actions::execute_statement;

fn main() -> Result<(), io::Error> {

    repl()?;

    Ok(())
}

fn repl() -> Result<(), io::Error> {
    //clear the screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    loop {
        print!("\u{1F47B} > ");
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

        println!("{:?}", statement);

        match execute_statement(statement) {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e),
        }
    }
}
