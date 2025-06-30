use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;
use lexical::parse_partial;

use crate::element::Element;

pub enum Token {
  SYMBOL(String),
  NUMBER(f64),
  ASSIGNMENT,
  ADD,
  MULT,
  SUB,
  DIV,
  OPAREN,
  CPAREN,
  SEMICOLON,
  COLON,
  COMMA,
}

pub struct Lexer<'a> {
    chars: &'a[u8]
}

impl Lexer<'_> {
    pub fn get_next_token(&mut self) -> Option<Token> {
        while !self.chars.is_empty() && self.chars[0].is_ascii_whitespace() {
            self.chars = &self.chars[1..];
        }
        if self.chars.is_empty() {
            return None;
        }

        if self.chars[0] == b'=' {
            self.chars = &self.chars[1..];
            return Some(Token::ASSIGNMENT);
        } else if self.chars[0] == b'+' {
            self.chars = &self.chars[1..];
            return Some(Token::ADD);
        } else if self.chars[0] == b'*' {
            self.chars = &self.chars[1..];
            return Some(Token::MULT);
        } else if self.chars[0] == b'-' {
            self.chars = &self.chars[1..];
            return Some(Token::SUB);
        } else if self.chars[0] == b'/' {
            self.chars = &self.chars[1..];
            return Some(Token::DIV);
        } else if self.chars[0] == b'(' {
            self.chars = &self.chars[1..];
            return Some(Token::OPAREN);
        } else if self.chars[0] == b')' {
            self.chars = &self.chars[1..];
            return Some(Token::CPAREN);
        } else if self.chars[0] == b';' {
            self.chars = &self.chars[1..];
            return Some(Token::SEMICOLON);
        } else if self.chars[0] == b':' {
            self.chars = &self.chars[1..];
            return Some(Token::COLON);
        } else if self.chars[0] == b',' {
            self.chars = &self.chars[1..];
            return Some(Token::COMMA);
        }

        todo!();
    }
}

pub fn parse_lattice_from_tracy_file(file_path: &str) -> Result<Vec<Element>, ()> {
    let f = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: Could not open {file_path}: {err}");
        exit(1);
    });
    let mut reader = BufReader::new(f);

    let mut file_contents: String = String::new();
    reader
        .read_to_string(&mut file_contents)
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Could not open {file_path}: {err}");
            exit(1);
        });

    let lexer: Lexer = Lexer {chars: file_contents.as_bytes()};

    if let Ok((partial_match, n)) = parse_partial::<f64,_>(lexer.chars) {
        println!("{partial_match}, {n} characters");
    } else {
        println!("No match found.  No float here.");
    }

    todo!();
}
