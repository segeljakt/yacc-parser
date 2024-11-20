pub mod grammar;
pub mod lexer;
pub mod parser;
pub mod token;

use lexer::Lexer;

fn main() {
    let input_file = std::env::args().nth(1).expect("No input file");
    let input = std::fs::read_to_string(input_file).expect("Failed to read input file");
    let lexer = Lexer::new(&input);
    let mut parser = parser::Parser::new(&input, lexer);
    let grammar = parser.parse_grammar();
    println!("{:?}", grammar);
}
