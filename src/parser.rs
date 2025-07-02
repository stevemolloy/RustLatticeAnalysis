use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

use evalexpr::*;
use winnow::combinator::{alt, delimited, opt, separated};
use winnow::token::literal;
use winnow::token::{take_till, take_while};
use winnow::{Parser, Result};

use crate::{make_drift, make_marker, make_quad, make_sbend};

// use crate::element::Element;

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

pub fn parse_lattice_from_tracy_file(file_path: &str) {
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
    for line in parsed_data.iter() {
        match line {
            Assignment(var, expr) => {
                match evaluate_expr(expr, &vars) {
                    Ok(result) => vars.insert(var, result),
                    Err(e) => {
                        eprintln!("ERROR: {}", e);
                        exit(1);
                    }
                };
            }
            _ => {}
        }
    }
    println!("{vars:?}");

    let mut eles = HashMap::new();
    for line in parsed_data.iter() {
        match line {
            Element(name, typ, params) => {
                match *typ {
                    "Drift" => {
                        let length = evaluate_expr(params["L"], &vars).unwrap();
                        eles.insert(name, make_drift((*name).to_string(), length));
                    }
                    "Cavity" => {
                        // let harnum = evaluate_expr(params["HarNum"], &vars).unwrap();
                        // let voltage = evaluate_expr(params["Voltage"], &vars).unwrap();
                        // let freq = evaluate_expr(params["Frequency"], &vars).unwrap();
                        // let phi = evaluate_expr(params["Phi"], &vars).unwrap();
                        eles.insert(name, make_drift((*name).to_string(), 0.0));
                    }
                    "Quadrupole" => {
                        let length = evaluate_expr(params["L"], &vars).unwrap();
                        let phi = evaluate_expr(params["Phi"], &vars).unwrap();
                        if phi != 0.0 {
                            eprintln!("ERROR: Cannot yet deal with skew quads");
                            exit(1);
                        }
                        let b_2 = evaluate_expr(params["B_2"], &vars).unwrap();
                        eles.insert(name, make_quad((*name).to_string(), length, b_2));
                    }
                    "Bending" => {
                        let length =
                            evaluate_expr(params.get("L").unwrap_or(&"0.0"), &vars).unwrap();
                        let b_2 =
                            evaluate_expr(params.get("B_2").unwrap_or(&"0.0"), &vars).unwrap();
                        let angle =
                            evaluate_expr(params.get("Phi").unwrap_or(&"0.0"), &vars).unwrap();
                        eles.insert(name, make_sbend((*name).to_string(), length, angle, b_2));
                    }
                    "Sextupole" => {
                        let length = evaluate_expr(params["L"], &vars).unwrap();
                        // let b_3 = evaluate_expr(params["B_3"], &vars).unwrap();
                        eles.insert(name, make_drift((*name).to_string(), length));
                    }
                    "Octupole" => {
                        let length = evaluate_expr(params["L"], &vars).unwrap();
                        // let b_4 = evaluate_expr(params["B_4"], &vars).unwrap();
                        eles.insert(name, make_drift((*name).to_string(), length));
                    }
                    "Marker" => {
                        eles.insert(name, make_marker((*name).to_string()));
                    }
                    &_ => todo!(),
                }
            }
            _ => {}
        }
    }
    for line in parsed_data.iter() {
        match line {
            Line(name, eles) => {
                println!("{name} --> {eles:?}");
            }
            _ => {}
        }
    }
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
