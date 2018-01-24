mod scanner;
mod parser;
mod token;
mod expr;

pub mod lox {

    use std::fs::File;
    use std::io::prelude::*;
    use std::io;
    use std::fmt;

    use scanner::Scanner;
    use parser::Parser;
    use token::TokenType;
    use token::Token;

    pub struct Error {
        pub line: usize,
        pub at: String,
        pub message: String
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[line {line}] Error{at}: {message}", 
                   line=self.line, at=self.at, message=self.message)
        }
    }

    impl Error {
        pub fn from_token(token: Token, message: &str) -> Self {
            match token.token_type {
                TokenType::Eof => Error {
                    line: token.line, 
                    at: " at end".to_string(), 
                    message: message.to_string()
                },
                _ => Error {
                    line: token.line, 
                    at: format!(" at '{}'", token.lexeme),
                    message: message.to_string()
                }
            }
        }

        pub fn from_line(line: usize, message: &str) -> Self {
           Error {
               line: line,
               at: "".to_string(),
               message: message.to_string()
           }
        }
    }

    pub fn run_file(path: &str) -> Result<(), Error> {
        let mut f = File::open(path)
            .expect(&format!("Path not found: {}", path));
        
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Failed reading contents from file!");
        
        run(&contents)
    }

    pub fn run_prompt() -> Result<(), Error> {
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line)
                .expect("Failed to read line");
            if let Err(e) = run(&line) {
                println!("{}", e);
                return Err(e);
            }
        }
    }

    pub fn run(source: &str) -> Result<(), Error> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let expression = parser.parse()?;
        println!("{}", expression);
        /*
        for token in tokens {
            println!("{}", token);
        }
        */
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::expr::Expr;
    use super::token::Token;
    use super::token::TokenType;
    use super::token::Literal;
    #[test]
    fn print_ast() {
        let expr = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, String::from("-"), Literal::Nil, 1),
                Box::new(Expr::Literal(Literal::Number(123.0))))),
            Token::new(TokenType::Star, String::from("*"), Literal::Nil, 1),
            Box::new(Expr::Grouping(
                Box::new(Expr::Literal(Literal::Number(45.67))))));

        let expr_str = format!("{}", expr).to_owned();
        assert_eq!(expr_str, "(* (- 123) (group 45.67))");
    }
}
