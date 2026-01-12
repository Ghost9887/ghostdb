use std::{
    process,
    io::{self, Write},
    fs::File,
};
use ghostdb::engine::bitcask::{
    Bitcask, open_file, execute_actions, 
};
use ghostdb::parser::parse::{
    Action,
    parse_repl_cmd,
};

fn main() -> Result<(), io::Error> {
    let path: &str = "data/data.log";
    let file: File = match open_file(path){
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {e}");
            process::exit(1);
        },
    };

    let engine: Bitcask = Bitcask::new(file);     

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

        let _code = match execute_actions(&actions, &mut engine) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("{}", e);
            },
        };

        println!("Actions: {:?}", actions);
    }
}
