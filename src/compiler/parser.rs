use std::iter::Peekable;
use std::slice::Iter;
use crate::ast::node::{self, Expression};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    IDENT,
    NUMBER,

    PLUS,
    MINUS,
    ASTERISK,
    SLASH, 
    PERCENT,
    HASH,
    DOLLAR,
    COLON,
    COMMA,
    ASSIGN,
    LPAREN, 
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOLON,

    EOF,
    UNDEFINED,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub tokentype: TokenType,
    pub literal: String,
} 

impl Token {
    fn new(tokentype: TokenType, literal: String) -> Token {
        Token {
            tokentype,
            literal
        }
    }

    fn number<'a>(c: char, it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> Self {
        let mut s = String::from(c);
        while let '0'..='9' = peekchar(it) {
            s.push(nextchar(it));
        }

        if peekchar(it) == '.' {
            s.push(nextchar(it));
        }

        while let '0'..='9' = peekchar(it) {
            s.push(nextchar(it));
        }

        Token::new(TokenType::NUMBER, s)
    }

    fn ident<'a>(c: char, it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> Self {
        let mut s = String::from(c);
        while let 'a'..='z' | 'A'..='Z' |'0'..='9' | '_' = peekchar(it) {
            s.push(nextchar(it));
        }
        Token::new(TokenType::IDENT, s)   
    }

}

fn nextchar<'a>(it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> char {
    *it.next().unwrap_or(&('@' as u8)) as char 
}

fn peekchar<'a>(it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> char {
    **it.peek().unwrap_or(&&('@' as u8)) as char
}

fn nexttoken<'a>(it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> Token {
    let mut c = nextchar(it);
    while c.is_whitespace() { c = nextchar(it); }
    
    match c {
        'a'..='z' | 'A'..='Z' | '_' => Token::ident(c, it),
        '0'..='9' => Token::number(c, it),
        '+' => Token::new(TokenType::PLUS, String::from(c)),
        '-' => Token::new(TokenType::MINUS, String::from(c)),
        '*' => Token::new(TokenType::ASTERISK, String::from(c)),
        '/' => match peekchar(it) {
            '/' => Token::new(TokenType::SEMICOLON, String::from(';')),
            _ => Token::new(TokenType::SLASH, String::from(c))
        }
        '%' => Token::new(TokenType::PERCENT, String::from(c)),
        '#' => Token::new(TokenType::HASH, String::from(c)),
        '$' => Token::new(TokenType::DOLLAR, String::from(c)),
        '(' => Token::new(TokenType::LPAREN, String::from(c)),
        ')' => Token::new(TokenType::RPAREN, String::from(c)),
        '{' => Token::new(TokenType::LBRACE, String::from(c)),
        '}' => Token::new(TokenType::RBRACE, String::from(c)),
        ':' => Token::new(TokenType::COLON, String::from(c)),
        ',' => Token::new(TokenType::COMMA, String::from(c)),
        '=' => Token::new(TokenType::ASSIGN, String::from(c)),
        ';' => Token::new(TokenType::SEMICOLON, String::from(c)),
        '@' => Token::new(TokenType::EOF, String::from(c)),
        _ => Token::new(TokenType::UNDEFINED, String::from(c))
    }
}

pub struct Parser<'a> {
    it : Peekable<Iter<'a, u8>>,
    curtoken: Token, 
    peektoken: Token,
}

impl<'a> Parser<'a> {
    fn from(it: Peekable<Iter<'a, u8>>) -> Self {
        let mut p = Self {
            it,
            curtoken: Token::new(TokenType::UNDEFINED, " ".to_owned()),
            peektoken: Token::new(TokenType::UNDEFINED, " ".to_owned())
        };
        p.advance();
        p.advance();
        p
    }

    fn advance(&mut self) {     
           
        self.curtoken = self.peektoken.clone();
        self.peektoken = nexttoken(&mut self.it);
    }

    fn parse_block(&mut self) -> node::Block {

        let mut program = node::Block::new();
        loop {
            if let TokenType::EOF = self.curtoken.tokentype {break;}
            
            // todo: it shouldn't just shutdown the program, we need to collect errors and output them together
            let stmt = self.parse_stmt().expect("Something wrong");
            program.stmts.push(stmt);
            self.advance();
        }   
        program
    }

    fn parse_stmt(&mut self) -> Result<Box<dyn node::Stmt>, &str> {
        match self.curtoken.tokentype {
            TokenType::HASH => self.parse_band(),
            _ => Err("missing '#'!")
        }
    }

    fn parse_band(&mut self) -> Result<Box<dyn node::Stmt>, &str> {
        let hash_sign = self.curtoken.clone();
        
        
        let name = if let TokenType::IDENT = &self.peektoken.tokentype {
            self.advance();
            self.parse_ident()
        } else {
            return Err("missing Identifier!")
        };
        loop {
            match self.curtoken.tokentype {
                TokenType::ASSIGN => break,
                TokenType::SEMICOLON | TokenType::EOF => return Err("missing '='!"),
                _ => self.advance()
            }
        }
        
        loop {
            match self.curtoken.tokentype {
                TokenType::SEMICOLON => break,
                TokenType::EOF => return Err("missing ';'!"),
                _ => self.advance()
            }
        }
        Ok(Box::new(node::Band {
            hash_sign,
            name,
            args: vec![],
            value: Box::new(Expression {})
        }))
    }

    fn parse_ident(&mut self) -> node::Identifier {
        node::Identifier {
            literal: self.curtoken.clone(),
            value: self.curtoken.literal.clone(),
        }
    }

}

pub fn parse(sentence: String) {
    let it = sentence.as_bytes().iter().peekable();

    let mut p = Parser::from(it);

    let program = p.parse_block();

    println!("{}", program);
    
    // loop {
    //     if let Token::SEMICOLON(_) = p.curtoken {break;}
    //     println!("{:?}", p.curtoken);
    //     p.advance();
    // }    
}
