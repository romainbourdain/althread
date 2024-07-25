use super::Ast;
use crate::parser::Rule;
use pest::iterators::Pairs;
use std::fmt;

impl<'a> Ast<'a> {
    fn draw_pairs(
        &self,
        pairs: Pairs<'a, Rule>,
        level: usize,
        prefix: &str,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let mut pairs_iter = pairs.peekable();
        while let Some(pair) = pairs_iter.next() {
            let has_next = pairs_iter.peek().is_some();
            let new_prefix = if has_next {
                format!("{}│   ", prefix)
            } else {
                format!("{}    ", prefix)
            };

            write!(
                f,
                "{}{}── {:?}",
                prefix,
                if has_next { "├" } else { "└" },
                pair.as_rule()
            )?;

            let mut inner_pairs = pair.clone().into_inner();
            if inner_pairs.clone().count() == 0 {
                writeln!(f, ": {}", pair.as_str())?;
            } else {
                while inner_pairs.clone().count() == 1 {
                    let inner_pair = inner_pairs.next().unwrap();
                    write!(f, " > {:?}", inner_pair.as_rule())?;
                    let inner_value = inner_pair.as_str();
                    inner_pairs = inner_pair.into_inner();
                    if inner_pairs.clone().count() == 0 {
                        writeln!(f, ": {}", inner_value)?;
                        break;
                    }
                }
                if inner_pairs.clone().count() > 0 {
                    writeln!(f)?;
                    self.draw_pairs(inner_pairs, level + 1, &new_prefix, f)?;
                }
            }
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Ast<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Process Bricks:")?;
        for (name, pairs) in &self.process_bricks {
            writeln!(f, "\n{}", name)?;
            self.draw_pairs(pairs.clone(), 0, "", f)?;
        }

        writeln!(f, "\nCondition Bricks:")?;
        for pairs in &self.condition_bricks {
            self.draw_pairs(pairs.clone(), 0, "", f)?;
        }

        writeln!(f, "\nGlobal Bricks:")?;
        for pairs in &self.global_bricks {
            self.draw_pairs(pairs.clone(), 0, "", f)?;
        }
        Ok(())
    }
}
