extern crate markov;

use markov::*;

fn main() {
    let text = "sometimes I like eating I also \
        like dancing and sometimes I even go \
        to the beach but when I go to the beach it's not always \
        a nice day but when it's a nice day I sometimes take my dog
        my dog likes eating too but doesn't always like the weather
        even when it's a nice day";

    let mut model = Model::new();
    model.ingest(text);

    let mut max_words = 100;
    for word in model.generator() {
        print!("{} ", word);
        max_words -= 1;
        if max_words == 0 {
            break;
        }
    }
}
