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
    fn ast_fmt(&self, f: &mut Formatter, prefix: Prefix) -> fmt::Result;
}

pub type Prefix = Vec<TreeMarker>;

#[derive(Clone)]
pub enum TreeMarker {
    Leaf,
    Branch,
}

impl fmt::Display for TreeMarker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TreeMarker::Leaf => write!(f, "└── "),
            TreeMarker::Branch => write!(f, "├── "),
        }
    }
}

pub fn display_prefix(prefix: Prefix, f: &mut Formatter<'_>) -> fmt::Result {
    for (i, p) in prefix.iter().enumerate() {
        // Si ce n'est pas le dernier élément, on ajoute des espaces
        if i < prefix.len() - 1 {
            match p {
                TreeMarker::Leaf => write!(f, "    ")?,
                TreeMarker::Branch => write!(f, "│   ")?,
            }
        } else {
            // Dernier élément (feuille ou branche finale)
            write!(f, "{}", p)?;
        }
    }
    Ok(())
}

pub fn concat_prefix(prefix: Prefix, marker: TreeMarker) -> Prefix {
    let mut new_prefix = prefix.clone();
    new_prefix.push(marker);
    new_prefix
}
