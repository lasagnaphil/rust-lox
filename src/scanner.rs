use super::token::Token;
use super::token::TokenType;
use super::token::Literal;
use super::lox;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0, current: 0, line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, lox::Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token::new( 
            TokenType::Eof,
            String::from(""),
            Literal::Nil,
            self.line
        ));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), lox::Error> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' | '=' | '<' | '>' => {
                let matched = self.match_char('=');
                self.add_token(
                    match c {
                        '!' => if matched { TokenType::BangEqual } else { TokenType::Bang },
                        '=' => if matched { TokenType::EqualEqual } else { TokenType::Equal },
                        '<' => if matched { TokenType::LessEqual } else { TokenType::Less },
                        '>' => if matched { TokenType::GreaterEqual } else { TokenType::Greater },
                        _ => panic!()
                    }
                );
            },
            '/' => {
                let matched = self.match_char('/');
                if matched {
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                }
                else {
                    self.add_token(TokenType::Slash);
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => { self.line += 1; },
            '"' => self.string()?,
            '0'...'9' => self.number(),
            'a'...'z' | 'A'...'Z' | '_' => self.identifier(),
            _ => {
                return Err(lox::Error::from_line(self.line, 
                                           &format!("Unexpected character: {}", c).to_owned()));
            }
        }
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source[self.current] != expected { return false; }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0' }
        self.source[self.current + 1]
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::Nil);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text.into_iter().collect(), literal, self.line));
    }

    fn parse_token(token_str: &str) -> Option<TokenType> {
        match token_str {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None
        }
    }

    fn string(&mut self) -> Result<(), lox::Error> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            return Err(lox::Error::from_line(self.line, "Unterminated string."));
        }

        self.advance();

        let value = self.source[(self.start + 1)..(self.current - 1)].into_iter().collect();
        self.add_token_with_literal(TokenType::Str, Literal::Str(value));
        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) { self.advance(); }

        if self.peek() == '.' {
            if self.peek_next().is_digit(10) {
                self.advance();
                while self.peek().is_digit(10) { self.advance(); }
            }
        }

        let value: f64 = self.source[self.start..self.current].into_iter().collect::<String>().parse().unwrap();
        self.add_token_with_literal(TokenType::Number, Literal::Number(value));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' { self.advance(); }
        let text = self.source[self.start..self.current].into_iter().collect::<String>();

        match Scanner::parse_token(&text) {
            Some(t) => self.add_token(t),
            None => self.add_token(TokenType::Identifier)
        }
    }

}
