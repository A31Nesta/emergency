use std::collections::HashMap;

use grand::Gex;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct EquationDto {
    raw: String,
    latex: String
}
impl EquationDto {
    pub fn new(raw: &str) -> Self {
        EquationDto {
            raw: raw.to_string(),
            latex: raw.to_string()
        }
    }
}

pub type Generator = Box<dyn FnMut(&mut HashMap<String, Gex>) -> EquationDto + Send + 'static>;