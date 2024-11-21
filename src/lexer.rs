use crate::token::Spanned;
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input[self.pos..].chars();
        let start = self.pos;
        let token = loop {
            let c = chars.next()?;
            match c {
                '\n' | ' ' | '\t' => {
                    self.pos += 1;
                    continue;
                }
                '%' => {
                    self.pos += 1;
                    let c = chars.next()?;
                    match c {
                        '%' => {
                            self.pos += 1;
                            Token::PercentPercent
                        }
                        '{' => {
                            self.pos += 1;
                            loop {
                                let c = chars.next()?;
                                self.pos += 1;
                                if c == '%' {
                                    let c = chars.next()?;
                                    self.pos += 1;
                                    if c == '}' {
                                        break;
                                    }
                                }
                            }
                            break Token::Prologue;
                        }
                        'a'..='z' | 'A'..='Z' => {
                            self.pos += 1;
                            while let Some(c) = chars.next() {
                                if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                                    self.pos += 1;
                                    continue;
                                }
                                break;
                            }
                            break Token::Directive;
                        }
                        _ => panic!("unexpected character: {:?} at byte {:?}", c, self.pos),
                    }
                }
                '>' => {
                    self.pos += 1;
                    break Token::GreaterThan;
                }
                '<' => {
                    self.pos += 1;
                    break Token::LessThan;
                }
                '|' => {
                    self.pos += 1;
                    break Token::Bar;
                }
                ':' => {
                    self.pos += 1;
                    break Token::Colon;
                }
                ';' => {
                    self.pos += 1;
                    break Token::SemiColon;
                }
                // {...}
                '{' => {
                    self.pos += 1;
                    let mut depth = 1;
                    loop {
                        let c = chars.next().expect("unexpected end of input");
                        self.pos += 1;
                        match c {
                            '{' => depth += 1,
                            '}' => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    break Token::Code;
                }
                'a'..='z' | 'A'..='Z' => {
                    self.pos += 1;
                    while let Some(c) = chars.next() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            self.pos += 1;
                            continue;
                        }
                        break;
                    }
                    break Token::Ident;
                }
                _ => panic!("unexpected character: {:?} at byte {:?}", c, self.pos),
            };
        };
        Some(Spanned::new(token, start..self.pos))
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }
}
