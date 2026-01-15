use crate::frontend::parser::parse::Statement;
use crate::frontend::parser::ast::{
    CreateType,
    Identifier,
};
use std::time::Instant;
use crate::backend::engine::bitcask::{
    execute_create_database,
};

pub fn execute_statement(statement: Statement) -> Result<String, String> {
    let mut success_string = String::new();
    let now = Instant::now();


    match statement {
        Statement::Create(data) => {
            //execute statement
            match data.core.create_type {
                CreateType::Database => {
                    let name = match data.core.name {
                        Identifier::Name(n) => n,
                    };
                    success_string.push_str(execute_create_database(name)?.as_str());
                },
                CreateType::Table => {
                    todo!()
                },
            }
        },
    }

    let elapsed = now.elapsed();
    success_string.push_str(format!(" ({:.2?})", elapsed).as_str());

    Ok(success_string)
}
