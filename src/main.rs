use rust_lattice_analysis::*;

fn main() {
    let kinetic_energy: f64 = 3.0e9;
    let periodicity = 20;
    let file_path = "lattices/max_4u_sp_jb_5.lat";
    let line: Line = Line::new(file_path, periodicity, kinetic_energy).unwrap();

    println!();
    println!("Summary of the lattice defined in {file_path}");
    println!();
    println!("Periodicity: {periodicity}");
    println!("Number of elements in the line: {}", line.line.len());
    println!(
        "Total length of the lattice: {:0.3} m ({:0.3} m for the line)",
        line.total_length, line.line_length
    );
    println!(
        "Total bending angle of the lattice {:0.3} deg ({:0.3} deg for the line)",
        line.line_angle, line.total_angle
    );
    println!();
    println!("Total matrix, R, for the line is:");
    print_matrix(&line.line_matrix);
    println!();
    println!("Total matrix, R, for the full system:");
    print_matrix(&line.total_matrix);
    println!();

    println!("Synchrotron radiation integrals:");
    println!(
        "\tI_1 = {:+0.3e} ({:+0.3e} for the line)",
        line.periodicity as f64 * line.synch_integrals[0],
        line.synch_integrals[0]
    );
    println!(
        "\tI_2 = {:+0.3e} ({:+0.3e} for the line)",
        line.periodicity as f64 * line.synch_integrals[1],
        line.synch_integrals[1]
    );
    println!(
        "\tI_3 = {:+0.3e} ({:+0.3e} for the line)",
        line.periodicity as f64 * line.synch_integrals[2],
        line.synch_integrals[2]
    );
    println!(
        "\tI_4 = {:+0.3e} ({:+0.3e} for the line)",
        line.periodicity as f64 * line.synch_integrals[3],
        line.synch_integrals[3]
    );
    println!(
        "\tI_5 = {:+0.3e} ({:+0.3e} for the line)",
        line.periodicity as f64 * line.synch_integrals[4],
        line.synch_integrals[4]
    );

    println!();
    println!("x fractional tune:    {}", line.x_frac_tune);
    println!("y fractional tune:    {}", line.y_frac_tune);
    println!(
        "Energy loss per turn: {:0.3} keV",
        line.e_loss_per_turn / 1e3
    );
    println!("Momentum compaction:  {:+0.3e}", line.mom_compact);
    println!("j_x:                  {:+0.3e}", line.j_x);
    println!("tau_x:                {:0.3} ms", 1e3 * line.tau_x);
    println!(
        "Natural x emittance:  {:0.3} pm.rad",
        1e12 * line.nat_emitt_x
    );
    println!("Energy spread:        {:0.3e}", line.e_spread);
}
