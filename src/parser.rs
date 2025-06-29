use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;

use crate::element::Element;

pub fn parse_lattice_from_tracy_file(file_path: &str) -> Result<Vec<Element>, ()> {
    let f = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: Could not open {file_path}: {err}");
        exit(1);
    });
    let mut reader = BufReader::new(f);

    let mut file_contents: String = String::new();
    reader
        .read_to_string(&mut file_contents)
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Could not open {file_path}: {err}");
            exit(1);
        });

    todo!();
}
