use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

use crate::error::AlthreadError;

#[derive(Parser)]
#[grammar = "althread.pest"]
struct AlthreadParser;

pub fn parse(source: &str) -> Result<Pairs<Rule>, AlthreadError> {
    AlthreadParser::parse(Rule::program, source).map_err(|e| {
        let (line, col) = match e.line_col {
            pest::error::LineColLocation::Pos(pos) | pest::error::LineColLocation::Span(pos, _) => {
                pos
            }
        };
        AlthreadError::error(line, col, format!("{}", e.variant.to_string()))
    })
}
