extern crate rand;
use std::collections::*;

// --> FutureStates?
#[derive(Clone)]
pub struct Targets {
    // Total number of words ingested
    count: u32,

    // Next word and the number of occurrences
    targets: HashMap<String, u32>,
}

impl Targets {
    fn new() -> Self {
        Targets {
            count: 0,
            targets: HashMap::new(),
        }
    }

    pub fn chance_of(&self, next: &str) -> f32 {
        match self.targets.get(next) {
            Some(v) => (*v as f32) / (self.count as f32),
            None => return 0.0,
        }
    }

    pub fn add(&mut self, next: &str) {
        let next_count = match self.targets.get(next) {
            Some(v) => v + 1,
            None => 1,
        };

        self.count += 1;
        self.targets.insert(String::from(next), next_count);
    }

    pub fn choose_random(&self) -> Option<&str> {
        if self.count == 0 {
            return None;
        }

        let mut cumulative_count = 0;
        let v = (rand::random::<f64>() * (self.count as f64)) as u32;
        for (next_word, count) in self.targets.iter() {
            cumulative_count += count;
            if cumulative_count >= v {
                return Some(next_word);
            }
        }

        panic!("This should be impossible");
    }
}

#[derive(Clone)]
pub struct Model {
    states: HashMap<String, Targets>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            states: HashMap::new(),
        }
    }

    pub fn ingest(&mut self, string: &str) {
        let mut prev_word = None;

        for word in string.split_whitespace() {
            if let Some(prev_word) = prev_word {
                let mut targets = self.given(prev_word);
                targets.add(word);
                self.states.insert(String::from(prev_word), targets);
            }

            prev_word = Some(word);
        }
    }

    pub fn given(&self, given: &str) -> Targets {
        match self.states.get(given) {
            Some(v) => v.clone(),
            None => Targets::new(),
        }
    }

    pub fn random_state(&self) -> ModelState {
        // Not really random, just chooses the first word
        let word = self.states.keys().next().unwrap();
        ModelState {
            state: word.clone(),
            model: self.clone()
        }
    }

    pub fn state(&self, state: &str) -> ModelState {
        ModelState {
            state: String::from(state),
            model: self.clone()
        }
    }
}

// --> Generator?
pub struct ModelState {
    state: String,
    model: Model,
}

impl std::iter::Iterator for ModelState {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let targets = self.model.given(&self.state);
        let next_word = targets.choose_random()?;
        self.state = String::from(next_word);
        Some(String::from(next_word))
    }
}