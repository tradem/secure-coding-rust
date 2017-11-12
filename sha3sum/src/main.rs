extern crate sha3;
extern crate stopwatch;
extern crate pbr;

use sha3::{Sha3_512, Digest};
use stopwatch::{Stopwatch};
use pbr::ProgressBar;
// use std::time::Duration;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
// use std::io::Read;
// use number_prefix::{binary_prefix, Standalone, Prefixed};

const BUFFER_SIZE: usize = 1024 * 128 * 4; // 4KiB

/// Print digest result as hex string and name pair
fn print_result(sum: &[u8], name: &str) {
    for byte in sum {
        print!("{:02x}", byte);
    }
    println!("\t{}", name);
}

/// Compute digest value for given `Reader` and print it
/// On any error simply return without doing anything

fn process<R: BufRead>(reader: &mut R, name: &str, lenght: u64) {
    // let mut buffer = [0u8; BUFFER_SIZE];
    let mut sh = Sha3_512::default();
    let mut count : u64 = 1;
    let buffy : u64 = BUFFER_SIZE as u64;
    if lenght > buffy {
        count = lenght / buffy;
    }
    let mut pb = ProgressBar::new(count);
    pb.format("╢▌▌░╟");
    let sw = Stopwatch::start_new();
    loop {
        let length = {
            let buffer = reader.fill_buf().unwrap();
            sh.input(&buffer);
            buffer.len()
        };
        if length == 0 { break; }
        reader.consume(length);
        pb.inc();
    }
    pb.finish_print("done");
    let duration = sw.elapsed();
    //match binary_prefix(lenght) {
    //    Standalone(bytes)   => println!("Hashing took {} minutes for {}.", duration.as_secs(), bytes),
    //    Prefixed(prefix, n) => println!("Hashing took {} minutes for {:.0} {}B", duration.as_secs(), n, prefix),
    //}
    println!("Hashing took {} minutes.", duration.as_secs() / 60);
    print_result(&sh.result(), name);
}


fn main() {
    let args = env::args();
    // Process files listed in command line arguments one by one
    // If no files provided process input from stdin
    if args.len() > 1 {
        for path_arg in args.skip(1) {
            // Open the path in read-only mode, returns `io::Result<File>`
            // Create a path to the desired file
            let path = Path::new(&path_arg);
            let display = path.display();
            let file = match File::open(&path) {
                // The `description` method of `io::Error` returns a string that
                // describes the error
                Err(why) => panic!("couldn't open {}: {}", display,
                                                              why.description()),
                Ok(file) => file,
             };
             let raw_lengh= file.metadata().unwrap().len() as usize;
             let length = raw_lengh as u64;
             let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
             process(&mut reader, &path_arg, length);
        }
    }
}
