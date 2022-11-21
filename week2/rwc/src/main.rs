use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead};
use std::process; // For read_file_lines()

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];

    // Your code here :)
    let file = File::open(filename).unwrap();
    let mut words_num = 0;
    let mut lines_num = 0;
    let mut chars_num = 0;

    for line in io::BufReader::new(file).lines() {
        let line_str = line.unwrap();
        lines_num += 1;
        words_num += line_str.split_whitespace().count();
        chars_num += line_str.len();
    }

    println!("\tlines\twords\tcharacters");
    println!("counts\t{}\t{}\t{}", lines_num, words_num, chars_num);
}
