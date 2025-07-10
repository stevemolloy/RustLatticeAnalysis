use crate::*;
use ndarray::{Array1, Array2, arr1};
use std::f64::consts::PI;

const ELECTRON_MASS: f64 = 510998.9499961642f64;
const C: f64 = 299792458.0f64;

pub struct Line {
    pub line: Vec<Element>,
    pub periodicity: usize,
    pub energy: f64,
    pub gamma0: f64,
    pub line_length: f64,
    pub total_length: f64,
    pub line_matrix: Array2<f64>,
    pub total_matrix: Array2<f64>,
    pub line_angle: f64,
    pub total_angle: f64,
    pub x_frac_tune: f64,
    pub y_frac_tune: f64,
    pub beta_x_vec: Vec<f64>,
    pub beta_y_vec: Vec<f64>,
    pub eta_x_vec: Vec<f64>,
    pub synch_integrals: [f64; 5],
    pub j_x: f64,
    pub tau_x: f64,
    pub e_loss_per_turn: f64,
    pub mom_compact: f64,
    pub nat_emitt_x: f64,
    pub e_spread: f64,
}

impl Line {
    pub fn new(file_path: &str, periodicity: usize, energy: f64) -> Result<Self, ParseError> {
        let line = parse_lattice_from_tracy_file(file_path)?;
        let line_length = get_line_length(&line);
        let line_matrix = get_line_matrix(&line);
        let total_matrix = apply_matrix_n_times(&line_matrix, periodicity);
        let line_angle = radians_to_degrees(get_bending_angle(&line));
        let total_angle = line_angle * periodicity as f64;

        let phi_x = ((total_matrix[[0, 0]] + total_matrix[[1, 1]]) / 2.0).acos();
        let phi_y = ((total_matrix[[2, 2]] + total_matrix[[3, 3]]) / 2.0).acos();
        let beta_x = (total_matrix[[0, 1]] / phi_x.sin()).abs();
        let beta_y = (total_matrix[[2, 3]] / phi_y.sin()).abs();
        let eta_x = total_matrix[[0, 5]] / (1.0 - total_matrix[[0, 0]]);

        let x_frac_tune = phi_x / (2.0 * PI);
        let y_frac_tune = phi_y / (2.0 * PI);

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

        let mut synch_integrals: [f64; 5] = [
            line_matrix[[4, 5]],
            synch_rad_integral_2(&line),
            synch_rad_integral_3(&line),
            0.0,
            0.0,
        ];

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
        let t_0 = (line_length * periodicity as f64) / C;
        let tau_x = 2.0 * energy * t_0
            / (j_x
                * e_loss_per_turn(
                    periodicity as f64 * synch_integrals[1],
                    energy / ELECTRON_MASS,
                ));
        let e_loss_per_turn = e_loss_per_turn(
            periodicity as f64 * synch_integrals[1],
            energy / ELECTRON_MASS,
        );

        let mom_compact = total_matrix[[4, 5]] / (line_length * periodicity as f64);

        let nat_emitt_x = natural_emittance_x(
            synch_integrals[1],
            synch_integrals[3],
            synch_integrals[4],
            energy / ELECTRON_MASS,
        );

        let e_spread = energy_spread(
            synch_integrals[1],
            synch_integrals[2],
            synch_integrals[3],
            energy / ELECTRON_MASS,
        );

        let retval = Line {
            line,
            periodicity,
            energy,
            gamma0: energy / ELECTRON_MASS,
            line_length,
            total_length: line_length * periodicity as f64,
            line_matrix,
            total_matrix,
            line_angle,
            total_angle,
            x_frac_tune,
            y_frac_tune,
            beta_x_vec,
            beta_y_vec,
            eta_x_vec,
            synch_integrals,
            j_x,
            tau_x,
            e_loss_per_turn,
            mom_compact,
            nat_emitt_x,
            e_spread,
        };

        Ok(retval)
    }
}
