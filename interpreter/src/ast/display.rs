use std::fmt::{self, Formatter};

#[macro_export]
macro_rules! write_indent {
    ($f:expr, $indent_level:expr, $($arg:tt)*) => {
        {
            let indent_str = " | ".repeat($indent_level);
            write!($f, "{} |─ {}\n", indent_str, format_args!($($arg)*))
        }
    };
}

pub trait AstDisplay {
    fn ast_fmt(&self, f: &mut Formatter, prefix: &Prefix) -> fmt::Result;
}

#[derive(Clone)]
pub struct Prefix {
    pub prefix: Vec<TreeMarker>,
}

#[derive(Clone)]
pub enum TreeMarker {
    Leaf,
    Branch,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut index = self.prefix.len();
        for marker in &self.prefix {
            index -= 1;
            match marker {
                TreeMarker::Leaf if index == 0 => write!(f, "└── ")?,
                TreeMarker::Branch if index == 0 => write!(f, "├── ")?,
                TreeMarker::Leaf => write!(f, "    ")?,
                TreeMarker::Branch => write!(f, "│   ")?,
            }
        }

        Ok(())
    }
}
impl Prefix {
    pub fn new() -> Self {
        Self { prefix: Vec::new() }
    }

    pub fn add_branch(&self) -> Self {
        let mut new = self.clone();
        new.prefix.push(TreeMarker::Branch);
        new
    }

    pub fn add_leaf(&self) -> Self {
        let mut new = self.clone();
        new.prefix.push(TreeMarker::Leaf);
        new
    }

    pub fn switch(&self) -> Self {
        let mut new = self.clone();
        let last = new.prefix.pop().unwrap();
        match last {
            TreeMarker::Leaf => new.prefix.push(TreeMarker::Branch),
            TreeMarker::Branch => new.prefix.push(TreeMarker::Leaf),
        }
        new
    }
}
