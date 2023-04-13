use std::iter::Peekable;

#[derive(Debug)]
enum Token {
    IDENT(String),
    NUMBER(String),
    COMMENT(String),

    PLUS(String),
    MINUS(String),
    ASTERISK(String),
    SLASH(String), 
    PERCENT(String),
    HASH(String),
    COLON(String),
    COMMA(String),
    ASSIGN(String),
    LPAREN(String), 
    RPAREN(String),
    LBRACE(String),
    RBRACE(String),
    
    EOL(String),
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
}


fn nextchar<'a>(it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> char {
    *it.next().unwrap_or(&('$' as u8)) as char 
}

fn peekchar<'a>(it: &mut Peekable<impl Iterator<Item = &'a u8>>) -> char {
    **it.peek().unwrap_or(&&('$' as u8)) as char
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
            '/' => Token::COMMENT(format!("{}{}", c, nextchar(it))),
            _ => Token::SLASH(String::from(c))
        }
        '%' => Token::PERCENT(String::from(c)),
        '#' => Token::HASH(String::from(c)),
        '(' => Token::LPAREN(String::from(c)),
        ')' => Token::RPAREN(String::from(c)),
        '{' => Token::LBRACE(String::from(c)),
        '}' => Token::RBRACE(String::from(c)),
        ':' => Token::COLON(String::from(c)),
        ',' => Token::COMMA(String::from(c)),
        '=' => Token::ASSIGN(String::from(c)),
        '$' => Token::EOL(String::from(c)),
        _ => Token::UNDEFINED(String::from(c))
    }
}

pub fn parse(line: String) {
    let mut it = line.as_bytes().iter().peekable();

    loop {
        let token = nexttoken(&mut it);
        println!("{:?}", token);

        if let Token::EOL(_) = token {
            break;
        }
    }
}
