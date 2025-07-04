use rust_lattice_analysis::*;

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

    let line_matrix = get_line_matrix(&line);
    println!("Total matrix, R, for the line is:");
    print_matrix(&line_matrix);
    println!();

    let total_matrix = apply_matrix_n_times(&line_matrix, periodicity);
    println!("Total matrix, R, for the full system:");
    print_matrix(&total_matrix);
    println!();
}
