extern crate markov;

use markov::*;

fn main() {
    let mut model: Model<char> = Model::new();
    model.ingest("Woop woohoo yes woo yeeaaaahhhh yaaay".chars());

    let mut max_words = 1000;
    for word in model.generator() {
        print!("{}", word);
        max_words -= 1;
        if max_words == 0 {
            break;
        }
    }
}
