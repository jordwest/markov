extern crate markov;

use markov::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Location {
    Home,
    Work,
    Shops,
    Airport,
    Tokyo,
    Osaka,
    NewYork,
}

impl<'a> From<&'a Location> for Location {
    fn from(l: &'a Location) -> Location {
        l.clone()
    }
}

fn main() {
    let mut model: Model<Location> = Model::new();
    use Location::*;
    let history = vec![
        Home, Work, Home, Work, Shops,
        Home, Airport, NewYork, Airport,
        Tokyo, Osaka, Airport, Home,
        Work, Airport, Tokyo, Airport,
        Home, Work, Shops, Home,
        Shops, Home,
    ];
    model.ingest(history.iter());

    for location in model.generator().take(50) {
        print!("{:?} > ", location);
    }
}
