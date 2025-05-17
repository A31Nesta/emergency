use std::{collections::HashMap, sync::Arc};

use grand::Gex;
use rand::Rng;
use tokio::sync::Mutex;

use crate::dto::{EquationDto, Generator};
use generator_functions::*;

mod generator_functions;

pub struct StateStruct {
    easy_generators_sorted: Vec<Generator>,
    mid_generators_sorted: Vec<Generator>,
    saved_generators: HashMap<String, Gex>
}

impl StateStruct {
    pub fn new() -> Self {
        StateStruct {
            easy_generators_sorted: vec![
                Box::new(easy_integer_addition),
                Box::new(easy_integer_subtraction),
                Box::new(easy_three_simple_ops)
            ],
            mid_generators_sorted: vec![
                Box::new(mid_simple_multiplication),
                Box::new(mid_simple_division),
                Box::new(mid_pemdas1),
                Box::new(mid_pemdas2)
            ],
            saved_generators: HashMap::new(),
        }
    }
    pub fn new_state() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::new()))
    }

    pub fn random_easy(&mut self) -> EquationDto {
        let index: usize = rand::rng().random_range(0..self.easy_generators_sorted.len());
        let function: &mut Generator = &mut self.easy_generators_sorted[index];
        function(&mut self.saved_generators)
    }
    pub fn random_mid(&mut self) -> EquationDto {
        let index: usize = rand::rng().random_range(0..self.mid_generators_sorted.len());
        let function: &mut Generator = &mut self.mid_generators_sorted[index];
        function(&mut self.saved_generators)
    }
}