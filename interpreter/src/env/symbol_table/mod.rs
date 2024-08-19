pub mod process_table;
pub mod symbol;
pub mod symbol_table_stack;

use std::collections::HashMap;

use symbol::Symbol;

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, symbol: Symbol) {
        self.symbols.insert(name, symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        self.symbols.get_mut(name)
    }

    pub fn update(&mut self, name: String, symbol: Symbol) {
        self.symbols.insert(name, symbol);
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }
}
