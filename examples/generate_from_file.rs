extern crate markov;

use std::fs::File;
use std::env;
use std::io::prelude::*;

use markov::*;

fn load_text() -> String {
    let mut args = env::args();
    args.next();
    let filename = args.next().expect("Please pass filename as first arg");

    println!("Loading from file '{}'", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    contents
}

fn main() {
    let text = load_text();

    let mut model: Model<String> = Model::new();
    model.ingest(text.split_whitespace());

    for word in model.generator().take(1000) {
        print!("{} ", word);
    }
}
