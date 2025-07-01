use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

use winnow::token::literal;
use winnow::token::take_while;
use winnow::{Parser, Result};

use crate::element::Element;

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

    todo!();
}

pub fn do_nothing_parser<'a>(_input: &mut &'a str) -> Result<&'a str> {
    Ok("")
}

pub fn parse_symbol<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)
}

pub fn use_line_parser<'a>(input: &mut &'a str) -> Result<&'a str> {
    (
        literal("USE"),
        literal(":"),
        take_while(1.., char::is_whitespace),
        parse_symbol,
        literal(";"),
    )
        .map(|(_, _, _, sym, _)| sym)
        .parse_next(input)
}
