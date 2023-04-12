use core::slice;

#[derive(Debug)]
enum Token {
    
    PLUS(String),
    MINUS(String),
    ASTERISK(String),
    SLASH(String), 
    PERCENT(String),

    EOF(String),
    UNDEFINED(String),
}


fn nextchar(it: &mut slice::Iter<u8>) -> char {
    *it.next().unwrap_or(&('$' as u8)) as char 
}

fn nexttoken(it: &mut slice::Iter<u8>) -> Token {
    let c = nextchar(it);
    Token::UNDEFINED(String::from(c))
}

pub fn parse(line: String) {
    let mut it = line.as_bytes().iter();
    loop {
        let token = nexttoken(&mut it);
        println!("{:?}", token);

        match token {
            Token::UNDEFINED(a) => if a == "$" {
                break;
            }
            _=> {}
        }
    }
}

