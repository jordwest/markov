extern crate rand;
use std::collections::*;
use std::hash::Hash;
use std::cmp::Eq;

#[derive(Clone)]
pub struct FutureStates<State: Hash + Eq> {
    // Total number of words ingested
    count: u32,

    // Next word and the number of occurrences
    states: HashMap<State, u32>,
}

impl <State: Hash + Eq> FutureStates<State> {
    fn new() -> Self {
        FutureStates {
            count: 0,
            states: HashMap::new(),
        }
    }

    pub fn chance_of<Q: Into<State>>(&self, next: Q) -> f32 {
        match self.states.get(&next.into()) {
            Some(v) => (*v as f32) / (self.count as f32),
            None => return 0.0,
        }
    }

    pub fn add_future_state<Q: Into<State>>(&mut self, state: Q) {
        *self.states.entry(state.into()).or_insert(0) += 1;
        self.count += 1;
    }

    pub fn choose_random(&self) -> Option<&State> {
        if self.count == 0 {
            return None;
        }

        let mut cumulative_count = 0;
        let v = (rand::random::<f64>() * (self.count as f64)) as u32;
        for (next_state, count) in self.states.iter() {
            cumulative_count += count;
            if cumulative_count >= v {
                return Some(&next_state);
            }
        }

        panic!("This should be impossible");
    }
}

#[derive(Clone)]
pub struct Model<State: Hash + Eq + Clone> {
    states: HashMap<State, FutureStates<State>>,
}

impl<State: Hash + Eq + Clone> Model<State> {
    pub fn new() -> Self {
        Model {
            states: HashMap::new(),
        }
    }

    pub fn ingest<Q, I>(&mut self, sequence: I)
        where Q: Into<State>,
              I: Iterator<Item=Q>,
    {
        let mut prev_state = None;

        for state in sequence {
            let state = state.into();
            if let Some(prev_state) = prev_state {
                self.given(prev_state).add_future_state(state.clone());
            }

            prev_state = Some(state);
        }
    }

    pub fn given<Q: Into<State>>(&mut self, given: Q) -> &mut FutureStates<State> {
        self.states
            .entry(given.into())
            .or_insert(FutureStates::new())
    }

    pub fn generator(&self) -> Generator<State> {
        let word = self.states.keys().next().unwrap();
        Generator {
            current_state: word.clone(),
            model: self.clone()
        }
    }
}

pub struct Generator<State: Hash + Eq + Clone> {
    current_state: State,
    model: Model<State>,
}

impl<State: Hash + Eq + Clone> std::iter::Iterator for Generator<State> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let targets = self.model.given(self.current_state.clone());
        let next_state = targets.choose_random()?;
        self.current_state = next_state.clone();
        Some(next_state.clone())
    }
}