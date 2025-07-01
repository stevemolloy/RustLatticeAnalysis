use rust_lattice_analysis::*;

#[test]
fn test_nothing_parser() {
    let mut input = "Hello world";

    let output = do_nothing_parser(&mut input);

    assert_eq!(input, "Hello world");
    assert_eq!(output, Ok(""));
}

#[test]
fn test_use_line_parser() {
    let mut input = "USE: line_to_use;";

    let output = use_line_parser(&mut input);

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
fn test_variable_assignment_parser() {
    let mut input = "L =  20 + 2;";
    let output = variable_assignment_parser(&mut input);
    assert_eq!(input, ";");
    assert_eq!(output, Ok(("L", "20 + 2")));

    let mut input = "my_element =  20 - 14e-6 / 2,";
    let output = variable_assignment_parser(&mut input);
    assert_eq!(input, ",");
    assert_eq!(output, Ok(("my_element", "20 - 14e-6 / 2")));

    let mut input = "my_element=20,";
    let output = variable_assignment_parser(&mut input);
    assert_eq!(input, ",");
    assert_eq!(output, Ok(("my_element", "20")));
}

#[test]
fn test_element_creation_parser() {
    let mut input = "d8:  Drift, L = 0.125 - 0.1;";
    let output = element_creation_parser(&mut input);
    assert_eq!(input, "");
    assert_eq!(output, Ok(("d8", "Drift", vec![("L", "0.125 - 0.1")])));

    let mut input = "q1   : Quadrupole, L = 0.25000, Phi =  0.00000, B_2 =  4.79596, N = n_quad;";
    let output = element_creation_parser(&mut input);
    assert_eq!(input, "");
    assert_eq!(
        output,
        Ok((
            "q1",
            "Quadrupole",
            vec![
               ("L","0.25000"),
               ("Phi","0.00000"),
               ("B_2","4.79596"),
               ("N","n_quad")
            ]
        ))
    );

    let mut input = "cav: Cavity, Frequency = c0/C*h_rf, Voltage = 2*1.50e6, HarNum = h_rf,\n     Phi = 0.0;";
    let output = element_creation_parser(&mut input);
    assert_eq!(input, "");
    assert_eq!(output, Ok(("cav", "Cavity", vec![
                ("Frequency", "c0/C*h_rf"),
                ("Voltage", "2*1.50e6"),
                ("HarNum", "h_rf"),
                ("Phi", "0.0"),
    ])));
}
