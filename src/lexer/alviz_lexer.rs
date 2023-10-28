use crate::lexer::Token;
use std::{
    iter::{Enumerate, Peekable},
    str::{Chars, FromStr},
};

pub struct Tokens<'a> {
    source: &'a str,
    chars: Peekable<Enumerate<Chars<'a>>>,
    has_next: bool,
}

impl<'a> Tokens<'a> {
    fn new(source: &'a str) -> Tokens<'a> {
        Tokens {
            source,
            chars: source.chars().enumerate().peekable(),
            has_next: true,
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next {
            return None;
        }
        while let Some(&(_, ch)) = self.chars.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.chars.next();
        }
        match self.chars.next() {
            Some((_, '<')) => Some(Token::LfAngle),
            Some((_, '>')) => Some(Token::RtAngle),
            Some((_, '[')) => Some(Token::LfBracket),
            Some((_, ']')) => Some(Token::RtBracket),
            Some((_, ':')) => Some(Token::Colon),
            Some((_, ';')) => Some(Token::Semicolon),
            Some((_, ',')) => Some(Token::Comma),
            Some((_, '/')) => Some(Token::FwdSlash),
            Some((_, '#')) => Some(Token::Hashtag),
            Some((i, ch_i)) if ch_i.is_alphabetic() => {
                let mut cursor = i;
                while let Some(&(j, ch_j)) = self.chars.peek() {
                    if !ch_j.is_alphanumeric() {
                        break;
                    }
                    cursor = j;
                    self.chars.next();
                }
                Some(Token::Ident(&self.source[i..=cursor]))
            },
            Some((i, ch_i)) if ch_i.is_digit(10) => {
                let mut cursor = i;
                while let Some(&(j, ch_j)) = self.chars.peek() {
                    if !ch_j.is_digit(10) {
                        break;
                    }
                    cursor = j;
                    self.chars.next();
                }
                match u32::from_str(&self.source[i..=cursor]) {
                    Ok(n) => Some(Token::Nat(n)),
                    Err(e) => panic!("{}", e),
                }
            }
            Some((_, ch_i)) => Some(Token::Char(ch_i)),
            None => {
                self.has_next = false;
                Some(Token::EoF)
            }
        }
    }
}

pub trait Lexer {
    fn tokens(&self) -> Tokens;
}

impl<'a> Lexer for &'a str {
    fn tokens(&self) -> Tokens {
        Tokens::new(self)
    }
}

impl Lexer for String {
    fn tokens(&self) -> Tokens {
        Tokens::new(self)
    }
}
