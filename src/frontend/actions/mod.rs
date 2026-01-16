use crate::frontend::parser::parse::Statement;
use crate::frontend::parser::ast::{
    Type,
    Identifier,
};
use std::time::Instant;
use crate::backend::engine::bitcask::{
    execute_create_database,
    execute_drop_database,
    execute_change_active_database,
};
use crate::backend::global::Global;

pub fn execute_statement(statement: Statement, global: &mut Global) -> Result<String, String> {
    let mut success_string = String::new();
    let now = Instant::now();


    match statement {
        Statement::Create(data) => {
            //execute statement
            match data.core.create_type {
                Type::Database => {
                    let name = match data.core.name {
                        Identifier::Name(n) => n,
                    };
                    success_string.push_str(execute_create_database(name)?.as_str());
                },
                Type::Table => {
                    todo!()
                },
            }
        },
        Statement::Drop(data) => {
            match data.core.drop_type {
                Type::Database => {
                    let name = match data.core.name {
                        Identifier::Name(n) => n,
                    };
                    success_string.push_str(execute_drop_database(name, global)?.as_str());
                },            
                Type::Table => {
                    todo!()
                },
            }
        },
        Statement::Use(data) => {
            let name = match data.core.name {
                Identifier::Name(n) => n,
            };
            success_string.push_str(execute_change_active_database(name, global)?.as_str());
        },
    }

    let elapsed = now.elapsed();
    success_string.push_str(format!(" ({:.2?})", elapsed).as_str());

    Ok(success_string)
}
