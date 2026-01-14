use std::{
    io::{self, Write},
};
use ghostdb::frontend::parser::parse::parse_cmd;

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
        
        match parse_cmd(cmd) {
            Ok(statement) => println!("{:?}", statement),
            Err(e) => eprintln!("{}", e),
        }
    }
}
