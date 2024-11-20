#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Equal,          // =
    PercentPercent, // %%
    GreaterThan,    // >
    LessThan,       // <
    Bar,            // |
    Colon,          // :
    SemiColon,      // ;
    Code,           // { ... }
    Ident,          // [a-zA-Z_][a-zA-Z0-9_]*
    String,         // '...'
    Eof,            // End of file
    Directive,      // %ident ...
}

pub struct Spanned<T> {
    pub data: T,
    pub span: std::ops::Range<usize>,
}

impl<T> Spanned<T> {
    pub fn new(data: T, span: std::ops::Range<usize>) -> Self {
        Spanned { data, span }
    }
}
