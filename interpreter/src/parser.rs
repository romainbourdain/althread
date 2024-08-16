use pest::{
    error::{ErrorVariant, LineColLocation},
    iterators::Pairs,
    Parser,
};
use pest_derive::Parser;

use crate::error::{AlthreadError, ErrorType};

#[derive(Parser)]
#[grammar = "althread.pest"]
struct AlthreadParser;

pub fn parse(source: &str) -> Result<Pairs<Rule>, AlthreadError> {
    AlthreadParser::parse(Rule::program, source).map_err(|e| {
        let (line, col) = match e.line_col {
            LineColLocation::Pos(pos) | LineColLocation::Span(pos, _) => pos,
        };
        let error_message = match e.variant {
            ErrorVariant::ParsingError { positives, .. } => {
                format!("Expected one of {:?}", positives)
            }
            ErrorVariant::CustomError { message } => message,
        };
        AlthreadError::new(ErrorType::SyntaxError, line, col, error_message)
    })
}
