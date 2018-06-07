extern crate markov;

use markov::*;

fn main() {
    let mut model: Model<char> = Model::new();
    model.ingest("Woop woohoo yes woo yeeaaaahhhh yaaay".chars());

    for character in model.generator().take(1000) {
        print!("{}", character);
    }
}
