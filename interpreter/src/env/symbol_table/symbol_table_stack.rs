use std::{cell::RefCell, rc::Rc};

use crate::ast::token::{datatype::DataType, identifier::Identifier, literal::Literal};

use super::{symbol::Symbol, SymbolTable};

#[derive(Debug)]
pub struct SymbolTableStack {
    pub tables: Vec<Rc<RefCell<SymbolTable>>>, // each item is a scope symbol table
}

impl SymbolTableStack {
    pub fn new(global: &Rc<RefCell<SymbolTable>>) -> Self {
        Self {
            tables: vec![Rc::clone(global)],
        }
    }

    pub fn insert(
        &mut self,
        mutable: bool,
        identifier: &Identifier,
        datatype: Option<DataType>,
        value: Option<Literal>,
    ) -> Result<(), String> {
        let current_symbol_table = self
            .tables
            .last()
            .ok_or_else(|| "No symbol table found".to_string())?;

        if current_symbol_table
            .borrow()
            .contains_key(&identifier.value)
        {
            return Err(format!(
                "Symbol {} already exists in current scope",
                identifier
            ));
        }

        let (datatype, value) = match (datatype, value) {
            (Some(datatype), Some(value)) => (datatype, value),
            (Some(datatype), None) => {
                let value = datatype.get_literal();
                (datatype, value)
            }
            (None, Some(value)) => (value.get_datatype(), value),
            (None, None) => (DataType::Void, Literal::Null),
        };

        current_symbol_table.borrow_mut().insert(
            identifier.value.to_string(),
            Symbol::new(mutable, datatype.clone(), value.clone())?,
        );

        Ok(())
    }

    pub fn update(&self, identifier: &Identifier, value: Literal) -> Result<(), String> {
        for table in self.tables.iter().rev() {
            if let Some(symbol) = table.borrow_mut().get_mut(&identifier.value) {
                if !symbol.mutable {
                    return Err(format!("Symbol {} is not mutable", identifier));
                }

                if symbol.datatype != value.get_datatype() {
                    return Err(format!(
                        "Cannot assign {} to {}",
                        value.get_datatype(),
                        symbol.datatype
                    ));
                }

                symbol.value = value.clone();
                break;
            }
        }

        Ok(())
    }

    pub fn get(&self, identifier: &Identifier) -> Result<Symbol, String> {
        self.tables
            .iter()
            .rev()
            .find_map(|table| table.borrow().get(&identifier.value).cloned())
            .ok_or_else(|| format!("Symbol {} not found", identifier))
    }

    pub fn get_global_table(&self) -> Rc<RefCell<SymbolTable>> {
        self.tables[0].clone()
    }
}
