// use rust_lattice_analysis::*;

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
