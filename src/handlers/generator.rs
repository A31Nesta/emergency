use crate::{dto::EquationDto, state::StateStruct};

pub enum Difficulty {
    Easy,
    Mid,
    Hard
}

// Ported from JavaScript
// Generator functions are in state/generator_functions.rs
pub fn generate_equation(state: &mut StateStruct, difficulty: Difficulty) -> EquationDto {
    // Gambling twice increases our chances of success ofc
    match difficulty {
        Difficulty::Easy => {
            state.random_easy()
        }
        Difficulty::Mid => {
            state.random_mid()
        }
        Difficulty::Hard => {
            state.random_mid()
        }
    }
}