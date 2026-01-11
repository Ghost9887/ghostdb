use std::{
    process,
    io::{self, Write},
};
use ghostdb::engine::bitcask::{
    Bitcask, open_file, 
};
use ghostdb::repl::actions::{
    Action,
    parse_repl_cmd,
    execute_action,
};

fn main() -> Result<(), io::Error> {
    let path = "data/data.log";
    let file = match open_file(path){
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {e}");
            process::exit(1);
        },
    };

    let engine = Bitcask::new(file);     

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
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd)?;

        let action: Action = parse_repl_cmd(cmd.trim());
        if action == Action::Quit {
            return Ok(());
        }
        
        match execute_action(action, &mut engine) {
            Ok(user) => eprintln!("{:?}", user),
            Err(e) => eprintln!("{e}"),
        }
    }
}
