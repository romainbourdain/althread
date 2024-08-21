use std::{cmp, fmt};

#[derive(Debug, PartialEq, cmp::Eq, Hash)]
pub enum ConditionKeyword {
    Always,
    Never,
}

impl fmt::Display for ConditionKeyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConditionKeyword::Always => write!(f, "always"),
            ConditionKeyword::Never => write!(f, "never"),
        }
    }
}
