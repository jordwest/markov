extern crate markov;

use std::fs::File;
use std::env;
use std::io::prelude::*;

use markov::*;

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("Please pass filename as first arg");

    println!("Loading from file '{}'", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut model = Model::new();
    model.ingest(&contents);

    let mut max_words = 1000;
    for word in model.random_state() {
        print!("{} ", word);
        max_words -= 1;
        if max_words == 0 {
            break;
        }
    }
}
