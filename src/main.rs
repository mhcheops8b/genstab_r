use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use serde::{Serialize, Deserialize};

// use std::time::{Duration, Instant};

use falglib;

fn main() {
    let args_len = std::env::args().len();

    if args_len < 4 {
        println!("Usage: {} <size> <rel_file> <pickle_file>", std::env::args().next().unwrap());
        return;
    }

    let mut cursize = 0usize;
    match std::env::args().nth(1).unwrap().parse() {
        Ok(val) => {cursize = val},
        Err(_e) => println!("Must be a number")
    }

    //let filename = String::from("C:/Users/mhycko/Documents/rust_genqord2/results/qord3_ord2canmax.txt");
    let filename = std::env::args().nth(2).unwrap();
    let pickle_filename = std::env::args().nth(3).unwrap();

    let mut stab_size = 0usize;
    let mut line_no = 0usize;
    let mut stable_perm_vec = Vec::<HashSet<Vec<usize>>>::new();
    if let Ok(lines_qord) = read_lines(&filename) {
        for line_qord in lines_qord.map_while(Result::ok) {
            line_no += 1;
            let parsed_ord = falglib::parse_vector(cursize, &line_qord);
            let cur_stable =falglib::rel_get_stabilizer_perms(&parsed_ord); 
            
            let cur_size = cur_stable.len();
            eprintln!("{line_no}\t{cur_size}");
            stab_size += cur_size;
            stable_perm_vec.push(cur_stable);
        }
        eprintln!("\t{stab_size}");
        // serialize
        let mut file_buf = BufWriter::new(File::create(&pickle_filename).expect("Cannot {pickle_filename} open for writing"));
	
	    let stable_perm_vec_serialized = serde_pickle::to_vec(&stable_perm_vec, Default::default()).unwrap();
        // if let Ok(_status)  = file_buf.write(&stable_perm_vec_serialized) {
        // }
        // else {
        //     
        // }
        if let Err(status)  = file_buf.write(&stable_perm_vec_serialized) {
            eprintln!("Error writing a pickle file '{pickle_filename}' returning {status:?}");
        }
    }
    else {
        eprintln!("Error opening file '{}'.", &filename);
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
