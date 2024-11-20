use crate::grammar::Declaration;
use crate::grammar::Grammar;
use crate::lexer::Lexer;
use crate::token::Spanned;
use crate::token::Token;

pub struct Parser<'a> {
    input: &'a str,
    lexer: std::iter::Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, lexer: Lexer<'a>) -> Self {
        Parser {
            input,
            lexer: lexer.peekable(),
        }
    }

    fn next(&mut self) -> Spanned<Token> {
        self.lexer.next().unwrap()
    }

    fn peek(&mut self) -> &Spanned<Token> {
        self.lexer.peek().unwrap()
    }

    fn text(&self, span: std::ops::Range<usize>) -> &str {
        &self.input[span]
    }

    fn expect(&mut self, token: Token) -> Spanned<Token> {
        match self.next() {
            spanned if spanned.data == token => spanned,
            spanned => panic!(
                "Expected {:?}, found {:?} at byte {:?}",
                token, spanned.data, spanned.span.start
            ),
        }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut prelude = Vec::new();
        loop {
            let t = self.peek();
            match t.data {
                Token::Directive => {
                    let directive = self.expect(Token::Directive);
                    let mut arguments = Vec::new();
                    // Parse until next directive
                    loop {
                        let t = self.peek();
                        match t.data {
                            Token::Directive => break,
                            Token::Ident => {
                                let ident = self.expect(Token::Ident);
                                arguments.push(self.text(ident.span.clone()).to_string());
                            }
                            _ => panic!("Expected % or ident, found {:?}", t.data),
                        }
                    }
                    let directive = Declaration {
                        name: self.text(directive.span.clone()).to_string(),
                        arguments,
                    };
                    prelude.push(directive);
                }
                _ => break,
            }
        }
        prelude
    }

    pub fn parse_grammar(&mut self) -> Grammar {
        let declarations = self.parse_declarations();
        self.expect(Token::PercentPercent);
        let rules = Vec::new();
        let programs = String::new();
        Grammar {
            declarations,
            rules,
            programs,
        }
    }
}
