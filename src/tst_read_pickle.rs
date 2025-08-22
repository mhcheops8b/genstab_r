use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};
//use serde;





fn main() {
    let args_len = std::env::args().len();

    if args_len < 2 {
        println!("Usage: {} <pickle_file>", std::env::args().next().unwrap());
        return;
    }

    let pickle_filename = std::env::args().nth(1).unwrap();

    let mut pickle_file = BufReader::new(File::open(pickle_filename).expect("Cannot open {pickle_filename} for reading"));

    let mut buffer = Vec::<u8>::new();

    pickle_file.read_to_end(&mut buffer).expect("Cannot read from pickle file {pickle_filename}");

    let de_stable_perm_vec:Vec<HashSet<Vec<usize>>> = serde_pickle::from_slice(&buffer, Default::default()).unwrap();

    for (i, sp) in de_stable_perm_vec.iter().enumerate() {
        eprintln!("{}\t{}", i+1, sp.len());
    }
    eprintln!("\t{}", de_stable_perm_vec.len());
}
