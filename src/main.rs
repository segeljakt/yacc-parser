pub mod display;
pub mod grammar;
pub mod lexer;
pub mod parser;
pub mod token;

use lexer::Lexer;

fn main() {
    let input_file = std::env::args().nth(1).expect("No input file");
    let input = std::fs::read_to_string(input_file).expect("Failed to read input file");

    // Testing lexer
    let lexer = Lexer::new(&input);
    for t in lexer {
        if t.data == token::Token::Err {
            let (line0, col0) = line_col(&input, t.span.start);
            let (line1, col1) = line_col(&input, t.span.end);
            panic!(
                r#"Un-scannable token at {}:{}..{}:{}: "{}""#,
                line0,
                col0,
                line1,
                col1,
                &input[t.span.clone()]
            );
        }
        println!("{:?}", t);
    }

    let lexer = Lexer::new(&input);
    let mut parser = parser::Parser::new(&input, lexer);
    let _grammar = parser.parse_grammar();
    // println!("{}", grammar);
}

fn line_col(input: &str, pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for c in input.chars().take(pos) {
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}
