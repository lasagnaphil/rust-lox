use super::token::*;
use super::expr::Expr;
use super::lox;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

/*
 * Simple recursive-descent parser.
 *  expression     → equality ;
 *  equality       → comparison ( ( "!=" | "==" ) comparison )* ;
 *  comparison     → addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
 *  addition       → multiplication ( ( "-" | "+" ) multiplication )* ;
 *  multiplication → unary ( ( "/" | "*" ) unary )* ;
 *  unary          → ( "!" | "-" ) unary ;
 *                 | primary ;
 *  primary        → NUMBER | STRING | "false" | "true" | "nil"
 *                 | "(" expression ")" ;
 */

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens, current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Expr, lox::Error> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, lox::Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, lox::Error> {
        let mut expr = self.comparison()?;

        while self.match_many(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, lox::Error> {
        let mut expr = self.addition()?;
        while self.match_many(&[TokenType::Greater, 
                          TokenType::GreaterEqual, 
                          TokenType::Less, 
                          TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.addition()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, lox::Error> {
        let mut expr = self.multiplication()?;
        while self.match_many(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.multiplication()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, lox::Error> {
        let mut expr = self.unary()?;
        while self.match_many(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, lox::Error> {
        if self.match_many(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Expr::Unary(operator, Box::new(right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, lox::Error> {
        if self.match_one(TokenType::False) { Ok(Expr::Literal(Literal::Bool(false))) }
        else if self.match_one(TokenType::True) { Ok(Expr::Literal(Literal::Bool(true))) }
        else if self.match_one(TokenType::Nil) { Ok(Expr::Literal(Literal::Nil)) }
        else if self.match_many(&[TokenType::Number, TokenType::Str]) {
            Ok(Expr::Literal(self.previous().literal.clone()))
        }
        else if self.match_one(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Expr::Grouping(Box::new(expr)))
        }
        else { Err(lox::Error::from_token(self.peek().clone(), "Expect expression.")) }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, lox::Error> {
        if self.check(token_type) { 
            Ok(self.advance()) 
        }
        else {
            Err(lox::Error::from_token(self.peek().clone(), message))
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if let TokenType::Semicolon = self.previous().token_type {
                return;
            }
            match self.peek().token_type {
                TokenType::Class => {},
                TokenType::Fun => {},
                TokenType::Var => {},
                TokenType::For => {},
                TokenType::If => {},
                TokenType::While => {},
                TokenType::Print => {},
                TokenType::Return => { return; },
                _ => {}
            }

            self.advance();
        }
    }

    fn match_one(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        return false;
    }

    fn match_many(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false; }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

}
