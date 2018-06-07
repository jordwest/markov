extern crate markov;

use markov::*;

fn main() {
    let text = "sometimes I like eating I also \
        like dancing and sometimes I even go \
        to the beach but when I go to the beach it's not always \
        a nice day but when it's a nice day I sometimes take my dog
        my dog likes eating too but doesn't always like the weather
        even when it's a nice day";

    let mut model: Model<String> = Model::new();
    model.ingest(text.split_whitespace());

    for word in model.generator().take(100) {
        print!("{} ", word);
    }
}
