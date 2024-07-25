use std::fmt;

use super::{Environment, Symbol, SymbolValue};

impl<'a> fmt::Display for Environment<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display the global table
        writeln!(f, "-------------------- Environment --------------------\n")?;
        writeln!(f, "Global:\n+------+------------+----------+------------+")?;
        writeln!(f, "| Name |  Datatype  | Mutable  | Value      |")?;
        writeln!(f, "+------+------------+----------+------------+")?;
        for (name, symbol) in self.global_table.iter() {
            writeln!(f, "| {:<4} | {} |", name, symbol)?;
        }
        writeln!(f, "+------+------------+----------+------------+")?;

        // Display each symbol table
        writeln!(f, "\nSymbol_tables:")?;
        for (index, table) in self.symbol_tables.iter().enumerate() {
            writeln!(f, "Table {}:", index)?;
            writeln!(f, "+------+------------+----------+------------+")?;
            writeln!(f, "| Name |  Datatype  | Mutable  | Value      |")?;
            writeln!(f, "+------+------------+----------+------------+")?;
            for (name, symbol) in table.iter() {
                writeln!(f, "| {:<4} | {} |", name, symbol)?;
            }
            writeln!(f, "+------+------------+----------+------------+")?;
        }
        Ok(())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<10} | {:<8} | {:<10}",
            self.datatype,
            self.mutable,
            match &self.value {
                Some(value) => value.to_string(),
                None => "None".to_string(),
            }
        )
    }
}

impl fmt::Display for SymbolValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SymbolValue::Null => "Null".to_string(),
                SymbolValue::Bool(b) => b.to_string(),
                SymbolValue::Int(i) => i.to_string(),
                SymbolValue::Float(fl) => fl.to_string(),
                SymbolValue::String(s) => s.clone(),
            }
        )
    }
}
