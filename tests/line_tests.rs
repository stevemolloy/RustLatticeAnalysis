use rust_lattice_analysis::*;

#[test]
fn max_lattice_test() {
    let kinetic_energy: f64 = 3.0e9;
    let periodicity = 20;
    let file_path = "lattices/max_4u_sp_jb_5.lat";
    let line: Line = Line::new(file_path, periodicity, kinetic_energy).unwrap();

    let si_exp = [
        1.563961e-3,
        2.092111e-2,
        1.061231e-3,
        -2.540279e-2,
        1.887260e-7,
    ];
    let emit_exp = 53.808e-12;
    let espread_exp = 9.234e-4;

    assert!(((line.synch_integrals[0] - si_exp[0]) / si_exp[0]).abs() < 1e-6);
    assert!(((line.synch_integrals[1] - si_exp[1]) / si_exp[1]).abs() < 1e-6);
    assert!(((line.synch_integrals[2] - si_exp[2]) / si_exp[2]).abs() < 1e-6);
    assert!(((line.synch_integrals[3] - si_exp[3]) / si_exp[3]).abs() < 1e-6);
    assert!(((line.synch_integrals[4] - si_exp[4]) / si_exp[4]).abs() < 1e-6);

    assert!(((emit_exp - line.nat_emitt_x) / emit_exp).abs() < 1e-3);
    assert!(((espread_exp - line.e_spread) / espread_exp).abs() < 1e-3);
}
