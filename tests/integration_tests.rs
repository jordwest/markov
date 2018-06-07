extern crate markov;

use markov::*;

#[test]
fn it_creates_a_markov_chain_and_ingests_a_string() {
    let mut model: Model<String> = Model::new();
    model.ingest("sometimes I like eating I also like dancing".split_whitespace());

    assert_eq!(
        model.given("sometimes").chance_of("I"),
        1.0,
    );

    assert_eq!(
        model.given("like").chance_of("eating"),
        0.5,
    );

    assert_eq!(
        model.given("like").chance_of("puking"),
        0.0,
    );

    assert_eq!(
        model.given("sometimes").choose_random(),
        Some(&String::from("I")),
    );
}
