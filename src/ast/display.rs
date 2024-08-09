use std::fmt;

use super::{
    stmt::{assign::Assign, expr::Expr, Stmt},
    Ast,
};

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Process Bricks:")?;
        for (name, brick) in &self.process_bricks {
            writeln!(f, "\nBrick: {}", name)?;
            self.draw_nodes(&brick.children, 0, "", f)?;
        }

        writeln!(f, "\nCondition Bricks:")?;
        for (name, brick) in &self.condition_bricks {
            writeln!(f, "\nBrick: {}", name)?;
            self.draw_nodes(&brick.children, 0, "", f)?;
        }

        if let Some(global_brick) = &self.global_brick {
            writeln!(f, "\nGlobal Brick:")?;
            self.draw_nodes(&global_brick.children, 0, "", f)?;
        }

        Ok(())
    }
}

impl Ast {
    fn draw_nodes(
        &self,
        nodes: &Vec<Stmt>,
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
                Stmt::Assign(assign) => {
                    writeln!(
                        f,
                        "{}{}── Assign: {}",
                        prefix,
                        if has_next { "├" } else { "└" },
                        match assign {
                            Assign::Binary(binary) => format!(
                                "Binary Assign - {} {:?}",
                                binary.identifier.value, binary.operator.value
                            ),
                            Assign::Unary(unary) => format!(
                                "Unary Assign - {} {:?}",
                                unary.identifier.value, unary.operator.value
                            ),
                        }
                    )?;
                }
                Stmt::Decl(decl) => {
                    writeln!(
                        f,
                        "{}{}── Decl: {:?} {}",
                        prefix,
                        if has_next { "├" } else { "└" },
                        decl.keyword.value,
                        decl.identifier.value,
                    )?;
                }
                Stmt::Expr(expr) => {
                    writeln!(
                        f,
                        "{}{}── Expr: {}",
                        prefix,
                        if has_next { "├" } else { "└" },
                        self.display_expr(expr),
                    )?;
                }
                Stmt::Run(run) => {
                    writeln!(
                        f,
                        "{}{}── Run: {}",
                        prefix,
                        if has_next { "├" } else { "└" },
                        run.identifier.value,
                    )?;
                }
                Stmt::Print(print) => {
                    writeln!(
                        f,
                        "{}{}── Print: {}",
                        prefix,
                        if has_next { "├" } else { "└" },
                        self.display_expr(&print.value),
                    )?;
                }
                Stmt::If(if_block) => {
                    writeln!(
                        f,
                        "{}{}── If Block:",
                        prefix,
                        if has_next { "├" } else { "└" },
                    )?;
                    writeln!(
                        f,
                        "{}    ├── Condition: {}",
                        prefix,
                        self.display_expr(&if_block.condition),
                    )?;
                    writeln!(f, "{}    ├── Then Block:", prefix)?;
                    self.draw_nodes(&if_block.then_block.children, level + 1, &new_prefix, f)?;
                    if let Some(else_block) = &if_block.else_block {
                        writeln!(f, "{}    └── Else Block:", prefix)?;
                        self.draw_nodes(&else_block.children, level + 1, &new_prefix, f)?;
                    }
                }
                Stmt::While(while_block) => {
                    writeln!(
                        f,
                        "{}{}── While Block:",
                        prefix,
                        if has_next { "├" } else { "└" },
                    )?;
                    writeln!(
                        f,
                        "{}    ├── Condition: {}",
                        prefix,
                        self.display_expr(&while_block.condition),
                    )?;
                    writeln!(f, "{}    └── Body:", prefix)?;
                    self.draw_nodes(&while_block.then_block.children, level + 1, &new_prefix, f)?;
                }
                Stmt::Scope(scope) => {
                    writeln!(f, "{}{}── Scope:", prefix, if has_next { "├" } else { "└" },)?;
                    self.draw_nodes(&scope.children, level + 1, &new_prefix, f)?;
                }
            }
        }
        Ok(())
    }

    fn display_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(binary) => format!(
                "Binary Expression - {} {:?} {}",
                self.display_expr(&binary.left),
                binary.operator.value,
                self.display_expr(&binary.right),
            ),
            Expr::Unary(unary) => format!(
                "Unary Expression - {:?}{}",
                unary.operator.value,
                self.display_expr(&*unary.operand)
            ),
            Expr::Primary(primary) => format!("Primary Expression - {:?}", primary.value),
        }
    }
}
