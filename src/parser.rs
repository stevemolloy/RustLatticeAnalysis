use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

use winnow::combinator::{alt, delimited, separated};
use winnow::token::literal;
use winnow::token::{take_till, take_while};
use winnow::{Parser, Result};

use crate::element::Element;

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Assignment(&'a str, &'a str),
    Element(&'a str, &'a str, Vec<(&'a str, &'a str)>),
    Line(&'a str, Vec<&'a str>),
    Use(&'a str),
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

    todo!();
}

pub fn do_nothing_parser<'a>(_input: &mut &'a str) -> Result<&'a str> {
    Ok("")
}

pub fn optional_whitespace<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(0.., char::is_whitespace).parse_next(input)
}

pub fn symbol<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)
}

pub fn use_instruction<'a>(input: &mut &'a str) -> Result<&'a str> {
    (literal("USE:"), optional_whitespace, symbol, literal(";"))
        .map(|(_, _, sym, _)| sym)
        .parse_next(input)
}

pub fn expr_til_semicolon_or_comma<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_till(1.., |c| c == ';' || c == ',').parse_next(input)
}

pub fn variable_assignment<'a>(input: &mut &'a str) -> Result<(&'a str, &'a str)> {
    (
        symbol,
        optional_whitespace,
        literal("="),
        optional_whitespace,
        expr_til_semicolon_or_comma,
    )
        .map(|(sym, _, _, _, expr)| (sym, expr))
        .parse_next(input)
}

pub fn variable_assignment_statement<'a>(input: &mut &'a str) -> Result<(&'a str, &'a str)> {
    (
        variable_assignment,
        optional_whitespace,
        literal(";")
    )
        .map(|(assign, _, _)| assign)
        .parse_next(input)
}

pub fn element_creation<'a>(
    input: &mut &'a str,
) -> Result<(&'a str, &'a str, Vec<(&'a str, &'a str)>)> {
    (
        symbol,
        optional_whitespace,
        literal(":"),
        optional_whitespace,
        symbol,
        optional_whitespace,
        literal(","),
        optional_whitespace,
        separated(
            0..,
            variable_assignment,
            delimited(optional_whitespace, literal(","), optional_whitespace),
        ),
        literal(";"),
    )
        .map(|(sym, _, _, _, typ, _, _, _, fields, _)| (sym, typ, fields))
        .parse_next(input)
}

pub fn line_creation<'a>(input: &mut &'a str) -> Result<(&'a str, Vec<&'a str>)> {
    (
        symbol,
        optional_whitespace,
        literal(":"),
        optional_whitespace,
        literal("LINE"),
        optional_whitespace,
        literal("="),
        optional_whitespace,
        delimited(
            "(",
            separated(
                0..,
                symbol,
                delimited(optional_whitespace, literal(","), optional_whitespace),
            ),
            ")",
        ),
        literal(";"),
    )
        .map(|(sym, _, _, _, _, _, _, _, defn, _)| (sym, defn))
        .parse_next(input)
}

pub fn parse_statement<'a>(input: &mut &'a str) -> Result<Statement<'a>> {
    alt((
        use_instruction.map(Statement::Use),
        element_creation.map(|(name, typ, fields)| Statement::Element(name, typ, fields)),
        line_creation.map(|(name, defn)| Statement::Line(name, defn)),
        variable_assignment_statement.map(|(var, expr)| Statement::Assignment(var, expr)),
    ))
    .parse_next(input)
}
