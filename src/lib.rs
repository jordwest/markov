extern crate rand;
use std::collections::*;

#[derive(Clone)]
pub struct FutureStates {
    // Total number of words ingested
    count: u32,

    // Next word and the number of occurrences
    states: HashMap<String, u32>,
}

impl FutureStates {
    fn new() -> Self {
        FutureStates {
            count: 0,
            states: HashMap::new(),
        }
    }

    pub fn chance_of(&self, next: &str) -> f32 {
        match self.states.get(next) {
            Some(v) => (*v as f32) / (self.count as f32),
            None => return 0.0,
        }
    }

    pub fn add_future_state(&mut self, word: &str) {
        *self.states.entry(String::from(word)).or_insert(0) += 1;
        self.count += 1;
    }

    pub fn choose_random(&self) -> Option<&str> {
        if self.count == 0 {
            return None;
        }

        let mut cumulative_count = 0;
        let v = (rand::random::<f64>() * (self.count as f64)) as u32;
        for (next_word, count) in self.states.iter() {
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
    states: HashMap<String, FutureStates>,
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
                self.given(prev_word).add_future_state(word);
            }

            prev_word = Some(word);
        }
    }

    pub fn given(&mut self, given: &str) -> &mut FutureStates {
        self.states
            .entry(String::from(given))
            .or_insert(FutureStates::new())
    }

    pub fn generator(&self) -> Generator {
        let word = self.states.keys().next().unwrap();
        Generator {
            current_state: word.clone(),
            model: self.clone()
        }
    }
}

pub struct Generator {
    current_state: String,
    model: Model,
}

impl std::iter::Iterator for Generator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let targets = self.model.given(&self.current_state);
        let next_state = targets.choose_random()?;
        self.current_state = String::from(next_state);
        Some(String::from(next_state))
    }
}