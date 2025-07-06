use ndarray::{arr1, Array1, Array2};
use rust_lattice_analysis::*;
use itertools::izip;

fn main() {
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

    let r11 = line_matrix[[0, 0]];
    let r12 = line_matrix[[0, 1]];
    let r22 = line_matrix[[1, 1]];
    let r33 = line_matrix[[2, 2]];
    let r34 = line_matrix[[2, 3]];
    let r44 = line_matrix[[3, 3]];
    let r16 = line_matrix[[0, 5]];

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
    let mut eta_x_vec: Vec<f64>  = vec![];
    for ele in line.iter() {
        beta_x_vec.push(twiss_mat[[0,0]]);
        beta_y_vec.push(twiss_mat[[2,2]]);
        eta_x_vec.push(eta_vec[[0]]);

        twiss_mat = ele.r_matrix.dot(&twiss_mat.dot(&ele.r_matrix.t()));
        eta_vec = ele.eta_prop_matrix.dot(&eta_vec);
    }
    beta_x_vec.push(twiss_mat[[0,0]]);
    beta_y_vec.push(twiss_mat[[2,2]]);
    eta_x_vec.push(eta_vec[[0]]);

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

    for (bx, by, ex) in izip!(beta_x_vec, beta_y_vec, eta_x_vec) {
        println!("{}, {}, {}", bx, by, ex);
    }
}
