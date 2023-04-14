use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub enum Token {
    IDENT(String),
    NUMBER(String),

    PLUS(String),
    MINUS(String),
    ASTERISK(String),
    SLASH(String), 
    PERCENT(String),
    HASH(String),
    DOLLAR(String),
    COLON(String),
    COMMA(String),
    ASSIGN(String),
    LPAREN(String), 
    RPAREN(String),
    LBRACE(String),
    RBRACE(String),
    
    EOS(String),
    UNDEFINED(String),
}

impl Token {
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

        Token::NUMBER(s)
    }

    fn ident<'a>(c: char, it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> Self {
        let mut s = String::from(c);
        while let 'a'..='z' | 'A'..='Z' |'0'..='9' | '_' = peekchar(it) {
            s.push(nextchar(it));
        }
        Token::IDENT(s)   
    }

    pub fn unwrap(&self) -> String {
        match self {
            Token::IDENT(s) => s.to_owned(),
            Token::NUMBER(s) => s.to_owned(),

            Token::PLUS(s) => s.to_owned(),
            Token::MINUS(s) => s.to_owned(),
            Token::ASTERISK(s) => s.to_owned(),
            Token::SLASH(s) => s.to_owned(), 
            Token::PERCENT(s) => s.to_owned(),
            Token::HASH(s) => s.to_owned(),
            Token::DOLLAR(s) => s.to_owned(),
            Token::COLON(s) => s.to_owned(),
            Token::COMMA(s) => s.to_owned(),
            Token::ASSIGN(s) => s.to_owned(),
            Token::LPAREN(s) => s.to_owned(), 
            Token::RPAREN(s) => s.to_owned(),
            Token::LBRACE(s) => s.to_owned(),
            Token::RBRACE(s) => s.to_owned(),
            
            Token::EOS(s) => s.to_owned(),
            Token::UNDEFINED(s) => s.to_owned(),
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::IDENT(s) => Self::IDENT(s.to_owned()),
            Token::NUMBER(s) => Self::NUMBER(s.to_owned()),

            Token::PLUS(s) => Self::PLUS(s.to_owned()),
            Token::MINUS(s) => Self::MINUS(s.to_owned()),
            Token::ASTERISK(s) => Self::ASTERISK(s.to_owned()),
            Token::SLASH(s) => Self::SLASH(s.to_owned()), 
            Token::PERCENT(s) => Self::PERCENT(s.to_owned()),
            Token::HASH(s) => Self::HASH(s.to_owned()),
            Token::DOLLAR(s) => Self::DOLLAR(s.to_owned()),
            Token::COLON(s) => Self::COLON(s.to_owned()),
            Token::COMMA(s) => Self::COMMA(s.to_owned()),
            Token::ASSIGN(s) => Self::ASSIGN(s.to_owned()),
            Token::LPAREN(s) => Self::LPAREN(s.to_owned()),
            Token::RPAREN(s) => Self::RPAREN(s.to_owned()),
            Token::LBRACE(s) => Self::LBRACE(s.to_owned()),
            Token::RBRACE(s) => Self::RBRACE(s.to_owned()),
            
            Token::EOS(s) => Self::EOS(s.to_owned()),
            Token::UNDEFINED(s) => Self::UNDEFINED(s.to_owned()),
        }
    }
}

fn nextchar<'a>(it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> char {
    *it.next().unwrap_or(&(';' as u8)) as char 
}

fn peekchar<'a>(it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> char {
    **it.peek().unwrap_or(&&(';' as u8)) as char
}

fn nexttoken<'a>(it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> Token {
    let mut c = nextchar(it);
    while c.is_whitespace() { c = nextchar(it); }
    
    match c {
        'a'..='z' | 'A'..='Z' | '_' => Token::ident(c, it),
        '0'..='9' => Token::number(c, it),
        '+' => Token::PLUS(String::from(c)),
        '-' => Token::MINUS(String::from(c)),
        '*' => Token::ASTERISK(String::from(c)),
        '/' => match peekchar(it) {
            '/' => Token::EOS(String::from(';')),
            _ => Token::SLASH(String::from(c))
        }
        '%' => Token::PERCENT(String::from(c)),
        '#' => Token::HASH(String::from(c)),
        '$' => Token::DOLLAR(String::from(c)),
        '(' => Token::LPAREN(String::from(c)),
        ')' => Token::RPAREN(String::from(c)),
        '{' => Token::LBRACE(String::from(c)),
        '}' => Token::RBRACE(String::from(c)),
        ':' => Token::COLON(String::from(c)),
        ',' => Token::COMMA(String::from(c)),
        '=' => Token::ASSIGN(String::from(c)),
        ';' => Token::EOS(String::from(c)),
        _ => Token::UNDEFINED(String::from(c))
    }
}

pub struct Parser<'a> {
    it : Peekable<Iter<'a, u8>>,
    curtoken: Token, 
    peektoken: Token
}

impl<'a> Parser<'a> {
    fn from(it: Peekable<Iter<'a, u8>>) -> Self {
        let mut p = Self {
            it,
            curtoken: Token::UNDEFINED(" ".to_owned()),
            peektoken: Token::UNDEFINED(" ".to_owned())
        };
        p.advance();
        p.advance();
        p
    }

    fn advance(&mut self) {     
           
        self.curtoken = self.peektoken.clone();
        self.peektoken = nexttoken(&mut self.it);
    }
}

pub fn parse(sentence: String) {
    let it = sentence.as_bytes().iter().peekable();

    let mut p = Parser::from(it);
    
    println!("{:?} {:?}", p.curtoken, p.peektoken);

}
