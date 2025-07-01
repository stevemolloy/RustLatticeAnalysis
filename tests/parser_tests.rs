use rust_lattice_analysis::*;

#[test]
fn test_nothing_parser() {
    let mut input = "Hello world";

    let output = do_nothing_parser(&mut input);

    assert_eq!(input, "Hello world");
    assert_eq!(output, Ok(""));
}

#[test]
fn test_use_instruction() {
    let mut input = "USE: line_to_use;";

    let output = use_instruction(&mut input);

    assert_eq!(input, "");
    assert_eq!(output, Ok("line_to_use"));
}

#[test]
fn test_parse_till_semicolon_or_comma() {
    let mut input = "12 + 15 / -42;";
    let output = expr_til_semicolon_or_comma(&mut input);
    assert_eq!(input, ";");
    assert_eq!(output, Ok("12 + 15 / -42"));

    let mut input = "12e+01 + 15e-5 / -42,";
    let output = expr_til_semicolon_or_comma(&mut input);
    assert_eq!(input, ",");
    assert_eq!(output, Ok("12e+01 + 15e-5 / -42"));
}

#[test]
fn test_variable_assignment() {
    let mut input = "L =  20 + 2;";
    let output = variable_assignment(&mut input);
    assert_eq!(input, ";");
    assert_eq!(output, Ok(("L", "20 + 2")));

    let mut input = "my_element =  20 - 14e-6 / 2,";
    let output = variable_assignment(&mut input);
    assert_eq!(input, ",");
    assert_eq!(output, Ok(("my_element", "20 - 14e-6 / 2")));

    let mut input = "my_element=20,";
    let output = variable_assignment(&mut input);
    assert_eq!(input, ",");
    assert_eq!(output, Ok(("my_element", "20")));
}

#[test]
fn test_element_creation() {
    let mut input = "d8:  Drift, L = 0.125 - 0.1;";
    let output = element_creation(&mut input);
    assert_eq!(input, "");
    assert_eq!(output, Ok(("d8", "Drift", vec![("L", "0.125 - 0.1")])));

    let mut input = "q1   : Quadrupole, L = 0.25000, Phi =  0.00000, B_2 =  4.79596, N = n_quad;";
    let output = element_creation(&mut input);
    assert_eq!(input, "");
    assert_eq!(
        output,
        Ok((
            "q1",
            "Quadrupole",
            vec![
                ("L", "0.25000"),
                ("Phi", "0.00000"),
                ("B_2", "4.79596"),
                ("N", "n_quad")
            ]
        ))
    );

    let mut input = r#"cav: Cavity, Frequency = c0/C*h_rf, Voltage = 2*1.50e6, HarNum = h_rf,

                                      Phi = 0.0;"#;
    let output = element_creation(&mut input);
    assert_eq!(input, "");
    assert_eq!(
        output,
        Ok((
            "cav",
            "Cavity",
            vec![
                ("Frequency", "c0/C*h_rf"),
                ("Voltage", "2*1.50e6"),
                ("HarNum", "h_rf"),
                ("Phi", "0.0"),
            ]
        ))
    );
}

#[test]
fn test_line_creation() {
    let mut input = "b_uc:   LINE = (d2_0, d2_1, d2_2, d2_3, d2_4, d2_5);";
    let output = line_creation(&mut input);
    assert_eq!(input, "");
    assert_eq!(
        output,
        Ok(("b_uc", vec!["d2_0", "d2_1", "d2_2", "d2_3", "d2_4", "d2_5"]))
    );
}

#[test]
fn test_parse_statement() {
    let mut input = "b_uc:   LINE = (d2_0, d2_1, d2_2, d2_3, d2_4, d2_5);";
    let output = parse_statement(&mut input);
    assert_eq!(input, "");
    assert_eq!(
        output,
        Ok(Statement::Line(
            "b_uc",
            vec!["d2_0", "d2_1", "d2_2", "d2_3", "d2_4", "d2_5"]
        ))
    );

    let mut input = "s1: Sextupole, L = 0.1,  B_3 = -1.24426e+02, N = n_sext;";
    let output = parse_statement(&mut input);
    assert_eq!(input, "");
    assert_eq!(
        output,
        Ok(Statement::Element(
            "s1",
            "Sextupole",
            vec![("L", "0.1"), ("B_3", "-1.24426e+02"), ("N", "n_sext")]
        ))
    );

    let mut input = "USE: sp;";
    let output = parse_statement(&mut input);
    assert_eq!(input, "");
    assert_eq!(output, Ok(Statement::Use("sp")));

    let mut input = "C    = 528.0/20.0;";
    let output = parse_statement(&mut input);
    assert_eq!(input, "");
    assert_eq!(output, Ok(Statement::Assignment("C", "528.0/20.0")));
}
