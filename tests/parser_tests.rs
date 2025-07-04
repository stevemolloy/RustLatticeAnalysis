use std::collections::HashMap;

use rust_lattice_analysis::*;

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
    let mut input = "begin: Marker;";
    let output = element_creation(&mut input);
    assert_eq!(input, "");
    assert_eq!(output, Ok(("begin", "Marker", HashMap::new())));

    let mut input = "d8:  Drift, L = 0.125 - 0.1;";
    let output = element_creation(&mut input);
    assert_eq!(input, "");
    assert_eq!(
        output,
        Ok(("d8", "Drift", HashMap::from([("L", "0.125 - 0.1")])))
    );

    let mut input = "q1   : Quadrupole, L = 0.25000, Phi =  0.00000, B_2 =  4.79596, N = n_quad;";
    let output = element_creation(&mut input);
    assert_eq!(input, "");
    assert_eq!(
        output,
        Ok((
            "q1",
            "Quadrupole",
            HashMap::from([
                ("L", "0.25000"),
                ("Phi", "0.00000"),
                ("B_2", "4.79596"),
                ("N", "n_quad")
            ])
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
            HashMap::from([
                ("Frequency", "c0/C*h_rf"),
                ("Voltage", "2*1.50e6"),
                ("HarNum", "h_rf"),
                ("Phi", "0.0"),
            ])
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
            HashMap::from([("L", "0.1"), ("B_3", "-1.24426e+02"), ("N", "n_sext")])
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

#[test]
fn test_parse_tracy_file() {
    use Statement::*;

    let mut input = r#"h_rf = 176;
    C    = 528.0/20.0;
    
    
    cav: Cavity, Frequency = c0/C*h_rf, Voltage = 2*1.50e6, HarNum = h_rf,
         Phi = 0.0;
    
    d1:  Drift, L = 0.01;
    d2:  Drift, L = 0.30311 - 0.1;
    d3:  Drift, L = 0.40311 - 0.30311;

         sp: LINE = (begin, sup_per, cav);
    
    USE: sp;
    "#;
    let output = parse_tracy_file(&mut input);
    assert_eq!(
        output,
        vec![
            Assignment("h_rf", "176"),
            Assignment("C", "528.0/20.0"),
            Element(
                "cav",
                "Cavity",
                HashMap::from([
                    ("Frequency", "c0/C*h_rf"),
                    ("Voltage", "2*1.50e6"),
                    ("HarNum", "h_rf"),
                    ("Phi", "0.0")
                ])
            ),
            Element("d1", "Drift", HashMap::from([("L", "0.01")])),
            Element("d2", "Drift", HashMap::from([("L", "0.30311 - 0.1")])),
            Element("d3", "Drift", HashMap::from([("L", "0.40311 - 0.30311")])),
            Line("sp", vec!["begin", "sup_per", "cav"]),
            Use("sp"),
        ]
    );
}

#[test]
fn test_line_parsing_weirdness() {
    let mut input = "m_cell: LINE = (
	  s2, d5, d4, q3, twk, ge, d6, gs, s1, d7, bpm, d8, ch, cv, d9, o3,
	  -b_mc, d10, q2, d11, o2, d12, q1, d12, o1, d13, ch, cv, d14, bpm, ge,
	  d15);";
    let output = line_creation(&mut input);
    assert_eq!(input, "");
    assert_eq!(
        output,
        Ok((
            "m_cell",
            vec![
                "s2", "d5", "d4", "q3", "twk", "ge", "d6", "gs", "s1", "d7", "bpm", "d8", "ch",
                "cv", "d9", "o3", "-b_mc", "d10", "q2", "d11", "o2", "d12", "q1", "d12", "o1",
                "d13", "ch", "cv", "d14", "bpm", "ge", "d15"
            ]
        ))
    );
}

#[test]
fn test_parse_lattice_from_tracy_file() {
    let filename = "lattices/max_4u_sp_jb_5.lat";
    let ele_names = vec![
        "begin", "d15", "ge", "bpm", "d14", "cv", "ch", "d13", "o1", "d12", "q1", "d12", "o2",
        "d11", "q2", "d10", "d1_u6", "d1_u5", "d1_u4", "d1_u3", "d1_u2", "d1_u1", "d1_0", "d1_d1",
        "d1_d2", "d1_d3", "d1_d4", "d1_d5", "o3", "d9", "cv", "ch", "d8", "bpm", "d7", "s1", "gs",
        "d6", "ge", "twk", "q3", "d4", "d5", "s2", "s3", "d5", "bpm", "d4", "r1", "d3", "cv", "ch",
        "d2", "s4", "d1", "d2_5", "d2_4", "d2_3", "d2_2", "d2_1", "d2_0", "d2_0", "d2_1", "d2_2",
        "d2_3", "d2_4", "d2_5", "d1", "s4", "ge", "d2", "gs", "d_corr", "d_corr", "d3", "r1", "d4",
        "d5", "s3", "s3", "d5", "bpm", "d4", "r1", "d3", "cv", "ch", "d2", "s4", "d1", "d2_5",
        "d2_4", "d2_3", "d2_2", "d2_1", "d2_0", "d2_0", "d2_1", "d2_2", "d2_3", "d2_4", "d2_5",
        "d1", "s4", "ge", "d2", "gs", "d_corr", "d_corr", "d3", "r1", "d4", "d5", "s3", "s3", "d5",
        "bpm", "d4", "r1", "d3", "cv", "ch", "d2", "s4", "d1", "d2_5", "d2_4", "d2_3", "d2_2",
        "d2_1", "d2_0", "d2_0", "d2_1", "d2_2", "d2_3", "d2_4", "d2_5", "d1", "s4", "d2", "ch",
        "cv", "d3", "r1", "d4", "bpm", "d5", "s3", "s3", "d5", "d4", "r1", "d3", "d_corr",
        "d_corr", "gs", "d2", "ge", "s4", "d1", "d2_5", "d2_4", "d2_3", "d2_2", "d2_1", "d2_0",
        "d2_0", "d2_1", "d2_2", "d2_3", "d2_4", "d2_5", "d1", "s4", "d2", "ch", "cv", "d3", "r1",
        "d4", "bpm", "d5", "s3", "s3", "d5", "d4", "r1", "d3", "d_corr", "d_corr", "gs", "d2",
        "ge", "s4", "d1", "d2_5", "d2_4", "d2_3", "d2_2", "d2_1", "d2_0", "d2_0", "d2_1", "d2_2",
        "d2_3", "d2_4", "d2_5", "d1", "s4", "d2", "ch", "cv", "d3", "r1", "d4", "bpm", "d5", "s3",
        "s2", "d5", "d4", "q3", "twk", "ge", "d6", "gs", "s1", "d7", "bpm", "d8", "ch", "cv", "d9",
        "o3", "d1_d5", "d1_d4", "d1_d3", "d1_d2", "d1_d1", "d1_0", "d1_u1", "d1_u2", "d1_u3",
        "d1_u4", "d1_u5", "d1_u6", "d10", "q2", "d11", "o2", "d12", "q1", "d12", "o1", "d13", "ch",
        "cv", "d14", "bpm", "ge", "d15", "cav",
    ];
    let line = parse_lattice_from_tracy_file(filename).unwrap();
    for (ele, name) in line.iter().zip(ele_names) {
        assert_eq!(name, ele.name);
    }

    let total_angle_expected = 18.0;
    let total_length_expected = 26.4;
    let total_angle_actual = get_bending_angle(&line);
    let total_length_actual = get_line_length(&line);
    assert!((total_length_actual - total_length_expected).abs() < 1e-3);
    assert!((total_angle_actual - total_angle_expected).abs() < 1e-3);
}
