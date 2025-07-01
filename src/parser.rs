use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

use winnow::combinator::{delimited, separated};
use winnow::token::literal;
use winnow::token::{take_till, take_while};
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

pub fn parse_optional_whitespace<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(0.., char::is_whitespace).parse_next(input)
}

pub fn parse_symbol<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)
}

pub fn use_line_parser<'a>(input: &mut &'a str) -> Result<&'a str> {
    (
        literal("USE:"),
        parse_optional_whitespace,
        parse_symbol,
        literal(";"),
    )
        .map(|(_, _, sym, _)| sym)
        .parse_next(input)
}

pub fn expr_til_semicolon_or_comma<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_till(1.., |c| c == ';' || c == ',').parse_next(input)
}

pub fn variable_assignment_parser<'a>(input: &mut &'a str) -> Result<(&'a str, &'a str)> {
    (
        parse_symbol,
        parse_optional_whitespace,
        literal("="),
        parse_optional_whitespace,
        expr_til_semicolon_or_comma,
    )
        .map(|(sym, _, _, _, expr)| (sym, expr))
        .parse_next(input)
}

pub fn element_creation_parser<'a>(
    input: &mut &'a str,
) -> Result<(&'a str, &'a str, Vec<(&'a str, &'a str)>)> {
    (
        parse_symbol,
        parse_optional_whitespace,
        literal(":"),
        parse_optional_whitespace,
        parse_symbol,
        parse_optional_whitespace,
        literal(","),
        parse_optional_whitespace,
        separated(
            0..,
            variable_assignment_parser,
            delimited(
                parse_optional_whitespace,
                literal(","),
                parse_optional_whitespace,
            ),
        ),
        literal(";"),
    )
        .map(|(sym, _, _, _, typ, _, _, _, fields, _)| (sym, typ, fields))
        .parse_next(input)
}
