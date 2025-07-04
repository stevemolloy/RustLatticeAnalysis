use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

use evalexpr::*;
use winnow::combinator::{alt, delimited, opt, separated};
use winnow::token::literal;
use winnow::token::{take_till, take_while};
use winnow::{Parser, Result};

use crate::{make_cavity, make_drift, make_marker, make_oct, make_quad, make_sbend, make_sext};

#[derive(Debug)]
pub struct ParseError;

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Assignment(&'a str, &'a str),
    Element(&'a str, &'a str, HashMap<&'a str, &'a str>),
    Line(&'a str, Vec<&'a str>),
    Use(&'a str),
}

fn evaluate_expr(expr: &str, vars: &HashMap<&str, f64>) -> Result<f64, EvalexprError> {
    let mut context = HashMapContext::new();

    for (&key, &val) in vars {
        context.set_value(key.into(), Value::Float(val))?;
    }

    match eval_with_context(expr, &context)? {
        Value::Int(i) => Ok(i as f64),
        Value::Float(f) => Ok(f),
        other => Err(EvalexprError::expected_number(other.clone())),
    }
}

pub fn parse_lattice_from_tracy_file(file_path: &str) -> Result<Vec<crate::Element>, ParseError> {
    use Statement::*;

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

    let parsed_data = parse_tracy_file(&file_contents);
    let mut vars = HashMap::new();
    let mut element_dictionary: HashMap<&str, crate::Element> = HashMap::new();
    let mut line_dictionary: HashMap<&str, Vec<crate::Element>> = HashMap::new();
    for line in parsed_data.iter() {
        match line {
            Assignment(var, expr) => {
                match evaluate_expr(expr, &vars) {
                    Ok(result) => vars.insert(var, result),
                    Err(e) => {
                        eprintln!("ERROR: {e}");
                        exit(1);
                    }
                };
            }
            Element(name, typ, params) => match *typ {
                "Drift" => {
                    let length = evaluate_expr(params.get("L").unwrap_or(&"0.0"), &vars).unwrap();
                    element_dictionary.insert(name, make_drift((*name).to_string(), length));
                }
                "Cavity" => {
                    let length = evaluate_expr(params.get("L").unwrap_or(&"0.0"), &vars).unwrap();
                    let harnum =
                        evaluate_expr(params.get("HarNum").unwrap_or(&"0.0"), &vars).unwrap();
                    let voltage =
                        evaluate_expr(params.get("Voltage").unwrap_or(&"0.0"), &vars).unwrap();
                    let freq =
                        evaluate_expr(params.get("Frequency").unwrap_or(&"0.0"), &vars).unwrap();
                    let phi = evaluate_expr(params.get("Phi").unwrap_or(&"0.0"), &vars).unwrap();
                    element_dictionary.insert(
                        name,
                        make_cavity((*name).to_string(), length, freq, voltage, phi, harnum),
                    );
                }
                "Quadrupole" => {
                    let length = evaluate_expr(params.get("L").unwrap_or(&"0.0"), &vars).unwrap();
                    let phi = evaluate_expr(params.get("Phi").unwrap_or(&"0.0"), &vars).unwrap();
                    if phi != 0.0 {
                        eprintln!("ERROR: Cannot yet deal with skew quads");
                        exit(1);
                    }
                    let b_2 = evaluate_expr(params.get("B_2").unwrap_or(&"0.0"), &vars).unwrap();
                    element_dictionary.insert(name, make_quad((*name).to_string(), length, b_2));
                }
                "Bending" => {
                    let length = evaluate_expr(params.get("L").unwrap_or(&"0.0"), &vars).unwrap();
                    let b_2 = evaluate_expr(params.get("B_2").unwrap_or(&"0.0"), &vars).unwrap();
                    let angle = evaluate_expr(params.get("Phi").unwrap_or(&"0.0"), &vars).unwrap();
                    element_dictionary
                        .insert(name, make_sbend((*name).to_string(), length, angle, b_2));
                }
                "Sextupole" => {
                    let length = evaluate_expr(params.get("L").unwrap_or(&"0.0"), &vars).unwrap();
                    let b_3 = evaluate_expr(params.get("B_3").unwrap_or(&"0.0"), &vars).unwrap();
                    element_dictionary.insert(name, make_sext((*name).to_string(), length, b_3));
                }
                "Octupole" => {
                    let length = evaluate_expr(params.get("L").unwrap_or(&"0.0"), &vars).unwrap();
                    let b_4 = evaluate_expr(params.get("B_4").unwrap_or(&"0.0"), &vars).unwrap();
                    element_dictionary.insert(name, make_oct((*name).to_string(), length, b_4));
                }
                "Marker" => {
                    element_dictionary.insert(name, make_marker((*name).to_string()));
                }
                &_ => todo!(),
            },
            Line(name, eles_in_line) => {
                let mut new_line: Vec<crate::Element> = Vec::new();
                let mut rev_line: bool;
                for item in eles_in_line.iter() {
                    let search_str = if let Some(stripped) = item.strip_prefix('-') {
                        rev_line = true;
                        stripped
                    } else {
                        rev_line = false;
                        item
                    };
                    if element_dictionary.contains_key(search_str) {
                        new_line.push(element_dictionary[search_str].clone());
                    } else if line_dictionary.contains_key(search_str) {
                        if rev_line {
                            for e in line_dictionary[search_str].iter().rev() {
                                new_line.push(e.clone());
                            }
                        } else {
                            for e in line_dictionary[search_str].iter() {
                                new_line.push(e.clone());
                            }
                        }
                    } else {
                        eprintln!("ERROR: Could not find the element {search_str}");
                        exit(1);
                    }
                }
                line_dictionary.insert(name, new_line);
            }
            Use(name) => {
                println!("Found a USE statement with the name: {name}");
                if !line_dictionary.contains_key(name) {
                    eprintln!("\tThis name does NOT exist in the dictionary");
                    return Err(ParseError);
                } else {
                    let retval = line_dictionary.remove(name).unwrap(); // line_dictionary[name];
                    return Ok(retval);
                }
            }
        }
    }
    eprintln!("ERROR: Input file does not contain a USE instruction");
    Err(ParseError)
}

pub fn optional_whitespace<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(0.., char::is_whitespace).parse_next(input)
}

pub fn symbol<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(1.., |c: char| c.is_alphanumeric() || c == '_' || c == '-').parse_next(input)
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
    (variable_assignment, optional_whitespace, literal(";"))
        .map(|(assign, _, _)| assign)
        .parse_next(input)
}

pub fn element_creation<'a>(
    input: &mut &'a str,
) -> Result<(&'a str, &'a str, HashMap<&'a str, &'a str>)> {
    (
        symbol,
        optional_whitespace,
        literal(":"),
        optional_whitespace,
        symbol,
        optional_whitespace,
        opt((
            literal(","),
            optional_whitespace,
            separated(
                0..,
                variable_assignment,
                delimited(optional_whitespace, literal(","), optional_whitespace),
            ),
        )),
        literal(";"),
    )
        .map(|(sym, _, _, _, typ, _, fields_opt, _)| {
            let fields = fields_opt
                .map(|(_, _, fields)| fields)
                .unwrap_or_else(HashMap::new);
            (sym, typ, fields)
        })
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
                delimited(optional_whitespace, symbol, optional_whitespace),
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

pub fn parse_tracy_file(input: &str) -> Vec<Statement> {
    let mut remaining = input;
    let mut statements = Vec::new();

    while !remaining.trim_start().is_empty() {
        remaining = remaining.trim_start();

        match parse_statement(&mut remaining) {
            Ok(statement) => {
                statements.push(statement);
            }
            Err(_err) => {
                eprintln!("ERROR: Could not parse.  Remaining = \"{remaining}\"");
                break;
            }
        }
    }

    statements
}
