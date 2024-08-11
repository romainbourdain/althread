// pub mod datatype;
// // pub mod display;
// pub mod symbol;
// pub mod symbol_table;
// pub mod value;

// use std::collections::HashMap;

// use pest::iterators::Pair;
// use symbol::Symbol;
// use symbol_table::SingleSymbolTable;

// use crate::{
//     ast::brick::Brick,
//     error::{AlthreadError, AlthreadResult, ErrorType},
//     parser::Rule,
// };

// #[derive(Debug)]
// pub struct Environment<'a> {
//     pub global_table: SingleSymbolTable,
//     pub running_process: HashMap<String, Brick<'a>>,
// }

// impl<'a> Environment<'a> {
//     pub fn new() -> Self {
//         Self {
//             global_table: SingleSymbolTable::new(),
//             running_process: HashMap::new(),
//         }
//     }

//     pub fn run_process(
//         &mut self,
//         identifier: String,
//         process_table: &HashMap<String, Brick<'a>>,
//     ) -> AlthreadResult<()> {
//         if let Some(process) = process_table.get(&identifier) {
//             self.running_process.insert(identifier, process.clone());
//             Ok(())
//         } else {
//             Err(AlthreadError::new(
//                 ErrorType::ProcessError,
//                 0,
//                 1,
//                 format!("Process {} doesn't exist", identifier),
//             ))
//         }
//     }
// }
