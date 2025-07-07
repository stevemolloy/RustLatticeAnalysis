use std::f64::consts::PI;

// use itertools::izip;
use ndarray::{Array1, Array2, arr1};
use rust_lattice_analysis::*;

const ELECTRON_MASS: f64 = 510998.9499961642f64;
const C: f64 = 299792458.0f64;

fn main() {
    let kinetic_energy: f64 = 3.0e9;
    let gamma0 = kinetic_energy / ELECTRON_MASS;
    let periodicity = 20;
    let file_path = "lattices/max_4u_sp_jb_5.lat";
    let line = parse_lattice_from_tracy_file(file_path).unwrap();

    // for (i, ele) in line.iter().enumerate() {
    //     println!(
    //         "{index}: {ele}: L = {len:0.6}",
    //         index = i + 1,
    //         len = ele.length
    //     );
    //     print_matrix(&ele.r_matrix);
    //     println!();
    // }

    let line_len = get_line_length(&line);
    let line_angle = radians_to_degrees(get_bending_angle(&line));
    let line_matrix = get_line_matrix(&line);
    let total_matrix = apply_matrix_n_times(&line_matrix, periodicity);

    let x_trace = total_matrix[[0, 0]] + total_matrix[[1, 1]];
    let y_trace = total_matrix[[2, 2]] + total_matrix[[3, 3]];
    let x_frac_tune = (x_trace / 2.0).acos() / (2.0 * PI);
    let y_frac_tune = (y_trace / 2.0).acos() / (2.0 * PI);

    let r11 = line_matrix[[0, 0]];
    let r12 = line_matrix[[0, 1]];
    let r22 = line_matrix[[1, 1]];
    let r33 = line_matrix[[2, 2]];
    let r34 = line_matrix[[2, 3]];
    let r44 = line_matrix[[3, 3]];
    let r16 = line_matrix[[0, 5]];
    let r56 = line_matrix[[4, 5]];

    let mut synch_integrals: [f64; 5] = [
        r56,
        synch_rad_integral_2(&line),
        synch_rad_integral_3(&line),
        0.0,
        0.0,
    ];

    let eta_x = r16 / (1.0 - r11);
    let phi_x = ((r11 + r22) / 2.0).acos();
    let phi_y = ((r33 + r44) / 2.0).acos();
    let beta_x = (r12 / phi_x.sin()).abs();
    let beta_y = (r34 / phi_y.sin()).abs();

    let mut twiss_mat = Array2::<f64>::from_diag(&arr1(&[
        beta_x,
        1.0 / beta_x,
        beta_y,
        1.0 / beta_y,
        0.0,
        0.0,
    ]));

    let mut eta_vec: Array1<f64> = arr1(&[eta_x, 0.0, 1.0]);

    let mut beta_x_vec: Vec<f64> = vec![];
    let mut beta_y_vec: Vec<f64> = vec![];
    let mut eta_x_vec: Vec<f64> = vec![];
    for ele in line.iter() {
        beta_x_vec.push(twiss_mat[[0, 0]]);
        beta_y_vec.push(twiss_mat[[2, 2]]);
        eta_x_vec.push(eta_vec[[0]]);

        if ele.k[0] != 0.0 && ele.length != 0.0 {
            let h = ele.k[0] / ele.length;
            let omega_sqr = h.powi(2) + ele.k[1];
            let omega = omega_sqr.abs().sqrt();
            let omega_l = omega * ele.length;
            let mean_eta = if omega_sqr > 0.0 {
                eta_vec[0] * omega_l.sin() / omega_l
                    + eta_vec[1] * (1.0 - omega_l.cos()) / (omega * omega_l)
                    + h * (omega_l - omega_l.sin()) / (omega.powi(3) * ele.length)
            } else {
                eta_vec[0] * omega_l.sinh() / omega_l
                    - eta_vec[1] * (1.0 - omega_l.cosh()) / (omega * omega_l)
                    - h * (omega_l - omega_l.sinh()) / (omega.powi(3) * ele.length)
            };
            synch_integrals[3] += mean_eta * h * ele.length * (2.0 * ele.k[1] + h * h);
            synch_integrals[4] += ele.length
                * h.abs().powi(3)
                * get_curly_h(
                    ele,
                    eta_vec[0],
                    eta_vec[1],
                    twiss_mat[[0, 0]],
                    -twiss_mat[[0, 1]],
                );
        }

        twiss_mat = ele.r_matrix.dot(&twiss_mat.dot(&ele.r_matrix.t()));
        eta_vec = ele.eta_prop_matrix.dot(&eta_vec);
    }
    beta_x_vec.push(twiss_mat[[0, 0]]);
    beta_y_vec.push(twiss_mat[[2, 2]]);
    eta_x_vec.push(eta_vec[[0]]);

    let j_x = 1.0 - synch_integrals[3] / synch_integrals[1];
    let t_0 = (line_len * periodicity as f64) / C;

    println!();
    println!("Summary of the lattice defined in {file_path}");
    println!();
    println!("Periodicity: {periodicity}");
    println!("Number of elements in the line: {len}", len = line.len());
    println!(
        "Total length of the lattice: {len:0.3} m ({line_len:0.3} m for the line)",
        len = periodicity as f64 * line_len
    );
    println!(
        "Total bending angle of the lattice {tot_angle:0.3} deg ({line_angle:0.3} deg for the line)",
        tot_angle = line_angle * periodicity as f64
    );
    println!();
    println!("Total matrix, R, for the line is:");
    print_matrix(&line_matrix);
    println!();
    println!("Total matrix, R, for the full system:");
    print_matrix(&total_matrix);
    println!();

    println!("Synchrotron radiation integrals:");
    println!(
        "\tI_1 = {:+0.3e} ({:+0.3e} for the line)",
        periodicity as f64 * synch_integrals[0],
        synch_integrals[0]
    );
    println!(
        "\tI_2 = {:+0.3e} ({:+0.3e} for the line)",
        periodicity as f64 * synch_integrals[1],
        synch_integrals[1]
    );
    println!(
        "\tI_3 = {:+0.3e} ({:+0.3e} for the line)",
        periodicity as f64 * synch_integrals[2],
        synch_integrals[2]
    );
    println!(
        "\tI_4 = {:+0.3e} ({:+0.3e} for the line)",
        periodicity as f64 * synch_integrals[3],
        synch_integrals[3]
    );
    println!(
        "\tI_5 = {:+0.3e} ({:+0.3e} for the line)",
        periodicity as f64 * synch_integrals[4],
        synch_integrals[4]
    );

    println!();
    println!("x fractional tune:    {x_frac_tune}");
    println!("y fractional tune:    {y_frac_tune}");
    println!(
        "Energy loss per turn: {:0.3} keV",
        e_loss_per_turn(periodicity as f64 * synch_integrals[1], gamma0) / 1e3
    );
    println!(
        "Momentum compaction:  {:+0.3e}",
        total_matrix[[4, 5]] / (line_len * periodicity as f64)
    );
    println!("j_x:                  {j_x:+0.3e}");
    println!(
        "tau_x:                {:0.3} ms",
        1e3 * 2.0 * kinetic_energy * t_0
            / (j_x * e_loss_per_turn(periodicity as f64 * synch_integrals[1], gamma0))
    );
    println!(
        "Natural x emittance:  {:0.3} pm.rad",
        1e12 * natural_emittance_x(
            synch_integrals[1],
            synch_integrals[3],
            synch_integrals[4],
            gamma0
        )
    );
    println!(
        "Energy spread:        {:0.3e}",
        energy_spread(
            synch_integrals[1],
            synch_integrals[2],
            synch_integrals[3],
            gamma0
        )
        .sqrt()
    );
    // for (bx, by, ex) in izip!(beta_x_vec, beta_y_vec, eta_x_vec) {
    //     println!("{}, {}, {}", bx, by, ex);
    // }
}
