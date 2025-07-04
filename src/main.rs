use rust_lattice_analysis::*;

fn main() {
    let line = parse_lattice_from_tracy_file("lattices/max_4u_sp_jb_5.lat").unwrap();

    let line_matrix = get_line_matrix(&line);

    print_matrix(line_matrix);
}
