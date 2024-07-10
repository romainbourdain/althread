use symbol_table::{Symbol, SymbolTable};

use crate::nodes::{datatype::DataType, expr::primary::PrimaryExpr};

pub mod symbol_table;

#[derive(Debug)]
pub struct Environment<'a> {
    pub symbol_tables: Vec<SymbolTable>,
    pub global_table: &'a mut SymbolTable,
}

impl<'a> Environment<'a> {
    pub fn new(global_table: &'a mut SymbolTable) -> Self {
        Self {
            symbol_tables: Vec::new(),
            global_table,
        }
    }

    pub fn push_table(&mut self) {
        self.symbol_tables.push(SymbolTable::new());
    }

    pub fn pop_table(&mut self) {
        self.symbol_tables.pop();
    }

    pub fn insert_symbol(
        &mut self,
        identifier: String,
        datatype: DataType,
        mutable: bool,
        value: Option<PrimaryExpr>,
    ) -> Result<(), String> {
        let current_symbol_table = self
            .symbol_tables
            .last_mut()
            .unwrap_or(&mut self.global_table);

        if current_symbol_table.contains_key(&identifier) {
            return Err(format!(
                "Symbol {} already exists in current scope",
                identifier
            ));
        }

        let symbol = Symbol {
            datatype,
            mutable,
            value,
        };
        current_symbol_table.insert(identifier, symbol);
        Ok(())
    }

    pub fn get_symbol(&self, identifier: &String) -> Result<&Symbol, String> {
        for table in self.symbol_tables.iter().rev() {
            if let Some(symbol) = table.get(identifier) {
                return Ok(symbol);
            }
        }

        if let Some(symbol) = self.global_table.get(identifier) {
            return Ok(symbol);
        }

        Err(format!("Symbol {} not found", identifier))
    }

    pub fn clear_global(&mut self) {
        self.global_table.clear()
    }

    pub fn update_symbol(&mut self, identifier: &String, value: PrimaryExpr) -> Result<(), String> {
        for table in self.symbol_tables.iter_mut().rev() {
            if let Some(symbol) = table.get_mut(identifier) {
                symbol.value = Some(value);
                return Ok(());
            }
        }

        if let Some(symbol) = self.global_table.get_mut(identifier) {
            symbol.value = Some(value);
            return Ok(());
        }

        Err(format!("Symbol {} not found", identifier))
    }
}
