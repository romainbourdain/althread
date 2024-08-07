pub mod assign;
pub mod blocks;
pub mod call;
pub mod decl;
pub mod expr;

use std::io::{self, Write};

use assign::eval_assign;
use blocks::{eval_if, eval_scope, eval_while};
use call::eval_call;
use decl::eval_decl;
use expr::eval_expr;

use crate::{args::Config, env::Environment, error::AlthreadResult};

use super::{
    node::{Atomic, AtomicKind, Block, BlockKind, Node},
    Ast, Brick,
};

impl<'a> Ast<'a> {
    pub fn eval(&mut self, env: &mut Environment, config: &Config) -> AlthreadResult<()> {
        for brick in &mut self.global_bricks {
            brick.consume(env, config)?;
        }
        for brick in &mut self.condition_bricks {
            brick.consume(env, config)?;
        }
        for (_, brick) in &mut self.process_bricks {
            loop {
                io::stdout().flush().expect("Erreur de flush");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Erreur de lecture");

                if !brick.consume(env, config)? {
                    break;
                }
            }
        }
        Ok(())
    }
}

impl<'a> Brick<'a> {
    pub fn consume(&mut self, env: &mut Environment, config: &Config) -> AlthreadResult<bool> {
        if self.nodes.is_empty() {
            return Ok(false);
        }

        if self.current == 0 {
            env.push_table();
        }

        if !self.nodes[self.current].consume(env, config)? {
            self.current += 1;
        }

        if self.current >= self.nodes.len() {
            env.pop_table();
            return Ok(false);
        }

        Ok(true)
    }
}

impl<'a> Node<'a> {
    pub fn consume(&mut self, env: &mut Environment, config: &Config) -> AlthreadResult<bool> {
        match self {
            Node::Atomic(atomic) => {
                atomic.consume(env)?;
                Ok(false)
            }
            Node::Block(block) => Ok(block.consume(env, config)?),
        }
    }

    pub fn reset(&mut self) {
        match self {
            Node::Block(block) => block.reset(),
            _ => (),
        }
    }
}

impl<'a> Atomic<'a> {
    pub fn consume(&self, env: &mut Environment) -> AlthreadResult<()> {
        let pair = self.pair.clone();
        println!("{:?}", pair.as_str());
        match self.kind {
            AtomicKind::Expr => {
                eval_expr(pair, env)?;
            }
            AtomicKind::Print => {
                eval_call(pair, env)?;
            }
            AtomicKind::Decl => {
                eval_decl(pair, env)?;
            }
            AtomicKind::Assignment => {
                eval_assign(pair, env)?;
            }
            AtomicKind::Run => {
                unimplemented!();
            }
        }
        Ok(())
    }
}

impl<'a> Block<'a> {
    pub fn consume(&mut self, env: &mut Environment, config: &Config) -> AlthreadResult<bool> {
        match self.kind {
            BlockKind::Scope => Ok(eval_scope(self, env, config)?),
            BlockKind::If => Ok(eval_if(self, env, config)?),
            BlockKind::While => Ok(eval_while(self, env, config)?),
        }
    }

    pub fn reset(&mut self) {
        self.current = 0;
        for child in &mut self.children {
            child.reset();
        }
    }
}

// impl<'a> Ast<'a> {
//     pub fn eval(&self, env: &mut Environment, config: &Config) -> AlthreadResult<()> {
//         for (_, pairs) in &self.process_bricks {
//             env.push_table();
//             eval_pairs(pairs.clone(), env, config)?;
//             env.pop_table();
//         }

//         Ok(())
//     }
// }

// fn eval_pairs<'a>(
//     mut pairs: Pairs<'a, Rule>,
//     env: &mut Environment,
//     config: &Config,
// ) -> AlthreadResult<()> {
//     loop {
//         if !consume_pair(&mut pairs, env, config)? {
//             break;
//         };
//     }

//     Ok(())
// }

// fn consume_pair(
//     pairs: &mut Pairs<Rule>,
//     env: &mut Environment,
//     config: &Config,
// ) -> AlthreadResult<bool> {
//     match pairs.next() {
//         None => Ok(false),
//         Some(pair) => {
//             match pair.as_rule() {
//                 Rule::expr => {
//                     eval_expr(pair, env)?;
//                 }
//                 Rule::print_stmt => eval_call(pair, env)?,
//                 Rule::decl => eval_decl(pair, env)?,
//                 Rule::assignment => eval_assign(pair, env)?,
//                 Rule::scope | Rule::run_stmt | Rule::while_stmt | Rule::if_stmt => {
//                     unimplemented!()
//                 }
//                 _ => return Err(no_rule!(pair)),
//             }
//             Ok(true)
//         }
//     }
// }
