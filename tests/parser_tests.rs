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
