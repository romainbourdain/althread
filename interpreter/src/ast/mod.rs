pub mod block;
pub mod display;
pub mod node;
pub mod statement;
pub mod token;

use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use block::Block;
use display::{AstDisplay, Prefix};
use node::Node;
use pest::iterators::Pairs;
use token::{condition_keyword::ConditionKeyword, literal::Literal};

use crate::{
    env::{process_env::ProcessEnv, Env},
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

#[derive(Debug)]
pub struct Ast {
    pub process_blocks: HashMap<String, Node<Block>>,
    pub condition_blocks: HashMap<ConditionKeyword, Node<Block>>,
    pub global_block: Option<Node<Block>>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            process_blocks: HashMap::new(),
            condition_blocks: HashMap::new(),
            global_block: None,
        }
    }

    pub fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let mut ast = Self::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::main_block => {
                    let mut pairs = pair.into_inner();

                    let main_block = Node::build(pairs.next().unwrap())?;
                    ast.process_blocks.insert("main".to_string(), main_block);
                }
                Rule::global_block => {
                    let mut pairs = pair.into_inner();

                    let global_block = Node::build(pairs.next().unwrap())?;
                    ast.global_block = Some(global_block);
                }
                Rule::condition_block => {
                    let mut pairs = pair.into_inner();

                    let keyword_pair = pairs.next().unwrap();
                    let condition_keyword = match keyword_pair.as_rule() {
                        Rule::ALWAYS_KW => ConditionKeyword::Always,
                        Rule::NEVER_KW => ConditionKeyword::Never,
                        _ => return Err(no_rule!(keyword_pair)),
                    };
                    let condition_block = Node::build(pairs.next().unwrap())?;
                    ast.condition_blocks
                        .insert(condition_keyword, condition_block);
                }
                Rule::process_block => {
                    let mut pairs = pair.into_inner();

                    let process_identifier = pairs.next().unwrap().as_str().to_string();
                    let process_block = Node::build(pairs.next().unwrap())?;
                    ast.process_blocks.insert(process_identifier, process_block);
                }
                Rule::EOI => (),
                _ => return Err(no_rule!(pair)),
            }
        }

        Ok(ast)
    }

    pub fn eval_process(
        &self,
        identifier: String,
        process: &mut ProcessEnv,
    ) -> AlthreadResult<Option<Literal>> {
        let block = self.process_blocks.get(&identifier).unwrap();
        block.eval(process)
    }

    pub fn eval_globals(&self, env: &Env) -> AlthreadResult<()> {
        if let Some(global) = &self.global_block {
            let mut global_env = ProcessEnv::new_global(env);
            while global.eval(&mut global_env)?.is_none() {}
        }

        Ok(())
    }

    pub fn eval_conditions(&self, env: &Env) -> AlthreadResult<()> {
        for (keyword, condition_block) in &self.condition_blocks {
            let mut condition_env = ProcessEnv::new_global(env);

            let condition = Self::eval_condition_block(condition_block, &mut condition_env)?;

            match keyword {
                ConditionKeyword::Always if !condition => {
                    Err("Always condition not met".to_string())
                }
                ConditionKeyword::Never if condition => Err("Never condition not met".to_string()),
                _ => Ok(()),
            }
            .map_err(|e| {
                AlthreadError::new(
                    ErrorType::SyntaxError,
                    condition_block.line,
                    condition_block.column,
                    e,
                )
            })?;
        }

        Ok(())
    }

    fn eval_condition_block(
        condition_block: &Node<Block>,
        process_env: &mut ProcessEnv,
    ) -> AlthreadResult<bool> {
        for condition in &condition_block.value.children {
            if !condition.eval(process_env)?.unwrap().is_true() {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.ast_fmt(f, &Prefix::new())
    }
}

impl AstDisplay for Ast {
    fn ast_fmt(&self, f: &mut Formatter, prefix: &Prefix) -> fmt::Result {
        if let Some(global_node) = &self.global_block {
            writeln!(f, "{}shared", prefix)?;
            global_node.ast_fmt(f, &prefix.add_branch())?;
        }

        writeln!(f, "")?;

        for (condition_name, condition_node) in &self.condition_blocks {
            writeln!(f, "{}{}", prefix, condition_name)?;
            condition_node.ast_fmt(f, &prefix.add_branch())?;
            writeln!(f, "")?;
        }

        for (process_name, process_node) in &self.process_blocks {
            writeln!(f, "{}{}", prefix, process_name)?;
            process_node.ast_fmt(f, &prefix.add_branch())?;
            writeln!(f, "")?;
        }

        Ok(())
    }
}
