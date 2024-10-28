use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead};
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut v: Vec<String> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        v.push(line_str);
    }
    Ok(v)
}

fn handle(str: &Vec<String>) -> (usize, usize, usize) {
    let length: usize = str.len();
    let mut words : usize = 0;
    let mut chars : usize = 0;

    for val in str {
        chars += val.len();
        words += val.split_whitespace().count();
    }  
    (length, words, chars)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let str = read_file_lines(filename).unwrap();
    let (lines,words,chars) = handle(&str);
    println!("{} {} {}", lines, words, chars);
}
