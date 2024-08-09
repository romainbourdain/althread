use std::fmt;

use pest::iterators::Pairs;

use crate::parser::Rule;

use super::{node::Node, Ast};

impl<'a> Ast<'a> {
    fn draw_nodes(
        &self,
        nodes: &Vec<Node<'a>>,
        level: usize,
        prefix: &str,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let mut nodes_iter = nodes.iter().peekable();
        while let Some(node) = nodes_iter.next() {
            let has_next = nodes_iter.peek().is_some();
            let new_prefix = if has_next {
                format!("{}│   ", prefix)
            } else {
                format!("{}    ", prefix)
            };

            match node {
                Node::Atomic(atomic) => {
                    write!(
                        f,
                        "{}{}── Atomic({:?})",
                        prefix,
                        if has_next { "├" } else { "└" },
                        atomic.kind
                    )?;
                    let pair = &atomic.pair;
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
                Node::Block(block) => {
                    writeln!(
                        f,
                        "{}{}── Block({:?})",
                        prefix,
                        if has_next { "├" } else { "└" },
                        block.kind
                    )?;
                    self.draw_nodes(&block.children, level + 1, &new_prefix, f)?;
                }
            }
        }
        Ok(())
    }

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

impl fmt::Display for Ast<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Process Bricks:")?;
        for (name, brick) in &self.process_bricks {
            writeln!(f, "\n{}", name)?;
            self.draw_nodes(&brick.nodes, 0, "", f)?;
        }

        writeln!(f, "\nCondition Bricks:")?;
        for brick in &self.condition_bricks {
            self.draw_nodes(&brick.nodes, 0, "", f)?;
        }

        writeln!(f, "\nGlobal Bricks:")?;
        for brick in &self.global_bricks {
            self.draw_nodes(&brick.nodes, 0, "", f)?;
        }
        Ok(())
    }
}
