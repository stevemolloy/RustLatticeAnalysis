use rust_lattice_analysis::*;

fn main() {
    let line = parse_lattice_from_tracy_file("lattices/max_4u_sp_jb_5.lat").unwrap();
    for ele in line.iter() {
        println!("{ele}");
    }
}
