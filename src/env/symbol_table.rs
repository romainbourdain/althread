use pest::iterators::Pair;

use crate::{
    error::{AlthreadError, AlthreadResult, ErrorType},
    parser::Rule,
};

use super::{datatype::DataType, value::Value, Environment, Symbol, SymbolTable};

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
        mutable: bool,
        identifier: &Pair<Rule>,
        datatype: Option<DataType>,
        value: Option<Value>,
    ) -> AlthreadResult<()> {
        let (line, column) = identifier.line_col();
        let identifier = identifier.as_str().to_string();
        let current_symbol_table = self
            .symbol_tables
            .last_mut()
            .unwrap_or(&mut self.global_table);

        if current_symbol_table.contains_key(&identifier) {
            return Err(AlthreadError::new(
                ErrorType::VariableError,
                line,
                column,
                format!("Symbol {} already exists in current scope", identifier),
            ));
        }

        let (datatype, value) = match (datatype, value) {
            (Some(datatype), Some(value)) => (datatype, value),
            (Some(datatype), None) => {
                let value = Value::from_datatype(&datatype);
                (datatype, value)
            }
            (None, Some(value)) => {
                let datatype = DataType::from_value(&value);
                (datatype, value)
            }
            (None, None) => (DataType::Void, Value::Null),
        };
        let value_type = DataType::from_value(&value);
        if datatype != value_type {
            return Err(AlthreadError::new(
                ErrorType::TypeError,
                line,
                column,
                format!(
                    "Wrong type for variable: expected {:?}, found {:?}",
                    datatype, value_type
                ),
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

    pub fn get_symbol(&self, identifier: &Pair<Rule>) -> AlthreadResult<&Symbol> {
        for table in self.symbol_tables.iter().rev() {
            if let Some(symbol) = table.get(identifier.as_str()) {
                return Ok(symbol);
            }
        }

        if let Some(symbol) = self.global_table.get(identifier.as_str()) {
            return Ok(symbol);
        }

        Err(AlthreadError::new(
            ErrorType::VariableError,
            0,
            0,
            format!("Symbol {} not found", identifier.as_str()),
        ))
    }

    pub fn update_symbol(&mut self, identifier: &String, value: Value) -> Result<(), String> {
        for table in self.symbol_tables.iter_mut().rev() {
            if let Some(symbol) = table.get_mut(identifier.as_str()) {
                symbol.value = value;
                return Ok(());
            }
        }

        if let Some(symbol) = self.global_table.get_mut(identifier.as_str()) {
            symbol.value = value;
            return Ok(());
        }

        Err(format!("Symbol {} not found", identifier.as_str()))
    }
}
