use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[command(version, about = "searches for patterns")]
struct Args {
    #[arg(short, long)]
    pattern: String,
    #[arg(short, long)]
    filepath: String,
}

fn main() {
    let args = Args::parse();
    let needle = args.pattern;
    let filepath = args.filepath;
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let re = Regex::new(&needle).unwrap();
    let mut count = 1;
    for line_ in reader.lines() {
        let line = line_.unwrap();
        let line = line.trim();
        match re.find(&line) {
            Some(_) => println!("{}: {}", count, line),
            None => {}
        }
        count += 1;
    }
}
