use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Commands
    Define,
    Having,
    Assigned,
    Extern,
    Block,
    EndLine,
    // Identifier contains the identifier as a String.
    Identifier(String),
    // Simple version, all num in Kaleidoscope are 64 bit floats
    // We store number in the variant instead of in a global variable.
    Number(f64),
    String(String),
    // UnknownChar corresponds to returning a positive integer from gettok.
    UnknownChar(char),
}

// Lexer is implemented as a struct that holds its state instead of a function that works on
// global state
#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    source: &'a str,
}

impl<'a> Iterator for Lexer<'a> {
    // We allow iterating over Tokens
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // next is first char that not equal whitespace
        let next = self.chars
            .find(|&c| c != ' ');

        match next {
            Some(c) => self.get_token(c),
            None => None,
        }
    }
}

impl <'a> Lexer<'a> {
    fn get_token(&mut self, c: char) -> Option<Token> {
        if c.is_alphabetic() {
            let mut iden = String::new();
            iden.push(c);

            loop {
                // We create nested block so xp will be out of scope
                // when self.chars.next() is called
                {
                    let xp = self.chars.peek();
                    match xp {
                        Some(c) if c.is_alphanumeric() => iden.push(*c),
                        _ => break,
                    }
                };
                self.chars.next();
            }

            match iden.as_str() {
                "create" => Some(Token::Define),
                "extern" => Some(Token::Extern),
                "in" => Some(Token::Block),
                _ => Some(Token::Identifier(iden)),
            }
        } else if c == '\'' {
            let xp = self.chars.next();
            match xp {
                Some(c) if c == 's' => Some(Token::Having),
                _ => Some(Token::UnknownChar(c)),
            }
        } else if c.is_digit(10) || c == '.' {
            let mut num = String::new();
            num.push(c);

            loop {
                // We create nested block so xp will be out of scope
                // when self.chars.next() is called
                {
                    let xp = self.chars.peek();
                    match xp {
                        Some(c) if c.is_digit(10) || *c == '.' => num.push(*c),
                        _ => break,
                    }
                };
                self.chars.next();
            }
            Some(Token::Number(num.parse().expect("Can't parse number")))
        } else if c == '"' {
            let mut iden = String::new();
            loop {
                let xp = self.chars.peek();
                match xp {
                    Some(c) if *c != '"' => iden.push(*c),
                    _ => break,
                };
                self.chars.next();
            }
            // after loop, we next again to bypass the end '"'
            self.chars.next();
            Some(Token::String(iden))
        } else if c == '=' {
            Some(Token::Assigned)
        } else if c == '\n' {
            Some(Token::EndLine)
        } else if c == '#' {
            loop {
                // We create nested block so xp will be out of scope
                // when self.chars.next() is called
                {
                    let xp = self.chars.peek();
                    match xp {
                        // just eat the chars
                        Some(c) if *c != '\r' && *c != '\n' => {},
                        _ => break,
                    }
                };
                self.chars.next();
            }
            self.next()
        } else {
            Some(Token::UnknownChar(c))
        }
    }

    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            chars: source.chars().peekable(),
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_tokens_and_value() {
        let mut lexer = Lexer::new("1 + 1 - foo");
        assert_eq!(lexer.next().unwrap(), Token::Number(1.0));
        assert_eq!(lexer.next().unwrap(), Token::UnknownChar('+'));
        assert_eq!(lexer.next().unwrap(), Token::Number(1.0));
        assert_eq!(lexer.next().unwrap(), Token::UnknownChar('-'));
        assert_eq!(lexer.next().unwrap(), Token::Identifier(String::from("foo")));
        assert_eq!(lexer.next(), None);
    }
    #[test]
    fn test_simple_tokens_and_value_no_whitespace() {
        let mut lexer = Lexer::new("1+1-foo");
        assert_eq!(lexer.next().unwrap(), Token::Number(1.0));
        assert_eq!(lexer.next().unwrap(), Token::UnknownChar('+'));
        assert_eq!(lexer.next().unwrap(), Token::Number(1.0));
        assert_eq!(lexer.next().unwrap(), Token::UnknownChar('-'));
        assert_eq!(lexer.next().unwrap(), Token::Identifier(String::from("foo")));
        assert_eq!(lexer.next(), None);
    }
    #[test]
    fn test_comments() {
        let code = "# This is a comment 1+1
        1 + 2 # <- is code
        # this is not";
        let mut lexer = Lexer::new(code);
        assert_eq!(lexer.next(), Some(Token::Number(1.0)));
        assert_eq!(lexer.next(), Some(Token::UnknownChar('+')));
        assert_eq!(lexer.next(), Some(Token::Number(2.0)));
        assert_eq!(lexer.next(), None);
    }
}