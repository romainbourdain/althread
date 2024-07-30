use std::fmt;

use super::{Environment, Symbol};

impl<'a> fmt::Display for Environment<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display the global table
        writeln!(f, "Global:\n+------+------------+----------+------------+")?;
        writeln!(f, "| Name |  Datatype  | Mutable  | Value      |")?;
        writeln!(f, "+------+------------+----------+------------+")?;
        for (name, symbol) in self.global_table.iter() {
            writeln!(f, "| {:<4} | {} |", name, symbol)?;
        }
        writeln!(f, "+------+------------+----------+------------+")?;

        // Display each symbol table
        for (index, table) in self.symbol_tables.iter().enumerate() {
            writeln!(f, "\nTable {}:", index)?;
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
            self.datatype.as_str(),
            self.mutable,
            self.value.to_string(),
        )
    }
}
