use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct Automaton<S, T> {
    pub external_states: HashSet<S>,
    pub internal_states: HashSet<S>,
    pub initial_states: HashSet<S>,
    pub final_states: HashSet<S>,
    pub external_transitions: HashMap<S, HashMap<T, S>>,
    pub internal_transitions: HashMap<S, HashMap<T, HashSet<S>>>,
}

impl<S, T> Automaton<S, T>
where
    S: Eq + Hash,
    T: Eq + Hash,
{
    /// Construct a new [`doa::Automaton`] instance.
    pub fn new() -> Self {
        Self {
            external_states: HashSet::new(),
            internal_states: HashSet::new(),
            initial_states: HashSet::new(),
            final_states: HashSet::new(),
            external_transitions: HashMap::new(),
            internal_transitions: HashMap::new(),
        }
    }

    /// Insert a new external state into the automata.
    pub fn insert_external_state(&mut self, state: S) {
        // TODO check wether the state exists
        self.external_states.insert(state);
    }

    /// Insert a new internal state into the automata.
    pub fn insert_internal_state(&mut self, state: S) {
        // TODO check wether the state exists
        self.internal_states.insert(state);
    }

    /// Insert a new initial state into the automata.
    pub fn insert_initial_state(&mut self, state: S) {
        // TODO check wether the state exists
        self.initial_states.insert(state);
    }

    /// Insert a new final state into the automata.
    pub fn insert_final_state(&mut self, state: S) {
        // TODO check wether the state exists
        self.final_states.insert(state);
    }

    /// Insert a new external transition into the automata.
    ///
    /// This function does not check if the states have been previously added.
    pub fn insert_external_transition(&mut self, source: S, transition: T, destination: S) {
        if let Some(deltas) = self.external_transitions.get_mut(&source) {
            deltas.insert(transition, destination);
        } else {
            let mut deltas = HashMap::new();
            deltas.insert(transition, destination);
            self.external_transitions.insert(source, deltas);
        };
    }

    /// Insert a new internal transition into the automata.
    ///
    /// This function does not check if the states have been previously added.
    pub fn insert_internal_transition(&mut self, source: S, transition: T, destination: S) {
        if let Some(deltas) = self.internal_transitions.get_mut(&source) {
            if let Some(destinations) = deltas.get_mut(&transition) {
                destinations.insert(destination);
            } else {
                let mut destinations = HashSet::new();
                destinations.insert(destination);
                deltas.insert(transition, destinations);
            }
        } else {
            let mut deltas = HashMap::new();
            let mut destinations = HashSet::new();
            destinations.insert(destination);
            deltas.insert(transition, destinations);
            self.internal_transitions.insert(source, deltas);
        }
    }

    pub fn validate<P: Property<S, T>>(&self) -> Option<P::Out> {
        P::validate(self)
    }
}

/// Trait for property validation over [`doa::Automaton`].
pub trait Property<S, T> {
    type Out;

    fn validate(input: &Automaton<S, T>) -> Option<Self::Out>;
}

/// A wrapper structure for the set of productive states.
///
/// Construction is done by validation of a [`doa::Automaton`].
///
/// For example:
/// ```rust
/// # use doa::{Automaton, Property, Productive};
/// # let automaton = Automaton::new();
/// let productive_states = automaton.validate::<Productive>();
/// ```
pub struct Productive<S>(HashSet<S>)
where
    S: Eq + Hash;

impl<S, T> Property<S, T> for Productive<S>
where
    S: Eq + Hash,
    T: Eq + Hash,
{
    type Out = S;

    fn validate(input: &Automaton<S, T>) -> Option<Self::Out> {
        // TODO implement the productive property validation
        todo!()
    }
}

/// A wrapper structure for the set of useful states.
///
/// Construction is done by validation of a [`doa::Automaton`].
///
/// For example:
/// ```rust
/// # use doa::{Automaton, Property, Useful};
/// # let automaton = Automaton::new();
/// let useful_states = automaton.validate::<Useful>();
/// ```
pub struct Useful<S>(HashSet<S>)
where
    S: Eq + Hash;

impl<S, T> Property<S, T> for Useful<S>
where
    S: Eq + Hash,
    T: Eq + Hash,
{
    type Out = S;

    fn validate(input: &Automaton<S, T>) -> Option<Self::Out> {
        // TODO implement the useful property validation
        todo!()
    }
}
