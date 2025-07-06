use rust_lattice_analysis::*;

#[test]
fn test_parse_lattice_from_tracy_file() {
    let filename = "lattices/max_4u_sp_jb_5.lat";
    let line = parse_lattice_from_tracy_file(filename).unwrap();

    let total_angle_expected = 18.0;
    let total_length_expected = 26.4;
    let total_angle_actual = radians_to_degrees(get_bending_angle(&line));
    let total_length_actual = get_line_length(&line);
    assert!((total_length_actual - total_length_expected).abs() < 1e-3);
    assert!((total_angle_actual - total_angle_expected).abs() < 1e-3);
}

#[test]
fn test_synch_rad_integrals() {
    let filename = "lattices/max_4u_sp_jb_5.lat";
    let line = parse_lattice_from_tracy_file(filename).unwrap();

    let line_matrix = get_line_matrix(&line);
    let r56 = line_matrix[[4, 5]];

    let i_1_expected = 1.563960764e-03;
    let i_2_expected = 2.092111245e-02;
    let i_3_expected = 1.061231140e-03;

    let i_1_calculated = r56;
    let i_2_calculated = synch_rad_integral_2(&line);
    let i_3_calculated = synch_rad_integral_3(&line);

    assert!((i_1_expected - i_1_calculated).abs() / i_1_expected.abs() < 1e-9);
    assert!((i_2_expected - i_2_calculated).abs() / i_2_expected.abs() < 1e-9);
    assert!((i_3_expected - i_3_calculated).abs() / i_3_expected.abs() < 1e-9);
}

// #[test]
// fn test_read_lattice_from_file() {
//     let file_path = "./lattices/max_4u_sp_jb_5.lat";
//     let line = parse_lattice_from_tracy_file(file_path);
// }

// #[test]
// fn test_line_matrix() {
//     let q1 = make_quad("quad".to_string(), 1.1, -0.5);
//     let d1 = make_drift("drift".to_string(), 2.0);
//     let m1 = make_marker("marker".to_string());
//     let s1 = make_sbend("sext".to_string(), 2.1, 0.1, 0.0);
//
//     let line = [d1, q1, m1, s1];
//     let line_matrix = get_line_matrix(&line);
//
//     println!("{line_matrix:?}");
// }
