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
        2.540279e-2,
        1.887260e-7,
    ];

    assert!((line.synch_integrals[0] - si_exp[0]) / si_exp[0] < 1e-6);
    assert!((line.synch_integrals[1] - si_exp[1]) / si_exp[1] < 1e-6);
    assert!((line.synch_integrals[2] - si_exp[2]) / si_exp[2] < 1e-6);
    assert!((line.synch_integrals[3] - si_exp[3]) / si_exp[3] < 1e-6);
    assert!((line.synch_integrals[4] - si_exp[4]) / si_exp[4] < 1e-6);
}
