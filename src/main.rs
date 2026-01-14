use std::{
    io::{self, Write},
};
use ghostdb::backend::engine::bitcask::Bitcask; 
use ghostdb::frontend::parser::parse::parse_repl_cmd;
use ghostdb::frontend::actions::{execute_actions, Action};
use ghostdb::frontend::codes::Code;

fn main() -> Result<(), io::Error> {
    let engine: Bitcask = Bitcask::new();     

    repl(engine)?;

    Ok(())
}

fn repl(mut engine: Bitcask) -> Result<(), io::Error> {
    //clear the screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    loop {
        print!("\u{1F47B} > ");
        io::stdout().flush()?;
        
        //get user command
        let mut raw_cmd: String = String::new();
        io::stdin().read_line(&mut raw_cmd)?;
        let cmd = raw_cmd.trim_end().trim_start();
        
        let actions: Vec<Action> = match parse_repl_cmd(cmd.to_string()) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            },
        };

        let code: Code = execute_actions(&actions, &mut engine); 

        match code {
            Code::Exit => break,
            Code::Uknown => println!("Uknown Failure"),
            Code::Error(err) => eprintln!("Error: {err}"),
            Code::Success(suc) => println!("Success: {suc}"),
        }
        println!("Actions: {:?}", actions);
    }

    Ok(())
}
