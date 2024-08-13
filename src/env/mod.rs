// pub mod process_table;
// pub mod symbol_table;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct SymbolTable {
    pub table: HashMap<String, ()>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolTableStack {
    pub stack: Vec<SymbolTable>,
}

impl SymbolTableStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    pub fn push(&mut self) {
        self.stack.push(SymbolTable::new());
    }
}

#[derive(Debug)]
pub struct Env {
    pub symbol_table: Rc<RefCell<SymbolTableStack>>,
    pub position: usize,
    pub child: Option<Box<Env>>,
}

impl Env {
    pub fn new(symbol_table: &Rc<RefCell<SymbolTableStack>>) -> Self {
        Self {
            position: 0,
            child: None,
            symbol_table: Rc::clone(symbol_table),
        }
    }

    pub fn consume(&mut self) {
        self.child = None;
        self.position += 1;
    }

    pub fn get_child(&mut self) -> &mut Env {
        if self.child.is_none() {
            self.child = Some(Box::new(Self::new(&self.symbol_table)));
        }

        self.child.as_mut().unwrap()
    }
}

// #[derive(Debug)]
// pub struct Env {
//     pub process_table: Rc<RefCell<ProcessTable>>,
//     pub global_table: Rc<RefCell<SymbolTable>>,
//     pub running_process: Rc<RefCell<RunningProcess>>,
// }

// impl Env {
//     pub fn new() -> Self {
//         Self {
//             process_table: Rc::new(RefCell::new(ProcessTable::new())),
//             global_table: Rc::new(RefCell::new(SymbolTable::new())),
//             running_process: Rc::new(RefCell::new(RunningProcess::new())),
//         }
//     }

//     pub fn run(&mut self, ast: &Ast) {
//         if let Some(_global_block) = &ast.global_block {
//             println!("Run global block");
//         }

//         for (name, _block) in &ast.condition_blocks {
//             println!("Run condition block {}", name);
//         }

//         for (name, _block) in &ast.process_blocks {
//             let process = Process::new(
//                 &self.global_table,
//                 &self.process_table,
//                 &self.running_process,
//             );

//             self.process_table
//                 .borrow_mut()
//                 .insert(name.clone(), process);
//         }

//         self.running_process
//             .borrow_mut()
//             .push("main".to_string(), &self.process_table);

//         println!("{}", self.process_table.borrow());
//         println!("{:?}", self.running_process.borrow());

//         // TODO : Boucle principale
//     }
// }
