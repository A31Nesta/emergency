use std::collections::HashMap;

use grand::Gex;
use rust_decimal::Decimal;

use crate::dto::EquationDto;

pub fn generate(saved_generators: &mut HashMap<String, Gex>, expression: &str) -> Decimal {
    if saved_generators.contains_key(expression) {
        saved_generators.get(expression).unwrap().generate()
    } else {
        let generator = grand::compile_raw(expression);
        let number = generator.generate();
        saved_generators.insert(expression.to_string(), generator);
        number
    }
}

pub fn easy_integer_addition(rngs: &mut HashMap<String, Gex>) -> EquationDto {
    let a = generate(rngs, "1..10|*1");
    let b = generate(rngs, "1..10|*1");
    return EquationDto::new(&format!("x = {a} + {b}"));
}
pub fn easy_integer_subtraction(rngs: &mut HashMap<String, Gex>) -> EquationDto {
    let a = generate(rngs, "1..20|*1");
    let b = generate(rngs, &format!("1..{a}|*1"));
    return EquationDto::new(&format!("x = {a} - {b}"));
}
pub fn easy_three_simple_ops(rngs: &mut HashMap<String, Gex>) -> EquationDto {
    let a = generate(rngs, "1..10|*1");
    let b = generate(rngs, "1..10|*1");
    let c = generate(rngs, "1..10|*1");
    let sign_a = if b>a { '+' } else if generate(rngs, "[0,1]").is_zero() { '+' } else { '-' };
    let sign_b = if c > (if sign_a == '+' {a+b} else {a-b}) {'+'} else if generate(rngs, "[0,1]").is_zero() {'+'} else {'-'};
    return EquationDto::new(&format!("x = {a} {sign_a} {b} {sign_b} {c}"));
}

pub fn mid_simple_multiplication(rngs: &mut HashMap<String, Gex>) -> EquationDto {
    let a = generate(rngs, "1..10|*1");
    let b = generate(rngs, "1..10|*1");
    return EquationDto::new(&format!("x = {a} * {b}"));
}
pub fn mid_simple_division(rngs: &mut HashMap<String, Gex>) -> EquationDto {
    let b = generate(rngs, "1..10|*1");
    let a = generate(rngs, &format!("1..100|*{b}"));
    return EquationDto::new(&format!("x = {a} / {b}"));
}
pub fn mid_pemdas1(rngs: &mut HashMap<String, Gex>) -> EquationDto {
    let b = generate(rngs, "1..10|*1");
    let c = generate(rngs, "1..10|*1");
    let sign_a = if c>=b {'+'} else if generate(rngs, "[0,1]").is_zero() {'+'} else {'-'};
    
    let constraint = if sign_a == '+' {b+c} else {b-c};
    let a = generate(rngs, &format!("{constraint},.100|*{constraint}"));
    return EquationDto::new(&format!("x = {a} / ({b} {sign_a} {c})"));
}
pub fn mid_pemdas2(rngs: &mut HashMap<String, Gex>) -> EquationDto {
    let b = generate(rngs, "1..10|*1");
    let c = generate(rngs, "1..10|*1");
    let sign_a = if generate(rngs, "[0,1]").is_zero() {'+'} else {'-'};
    
    let a = generate(rngs, "-10..10|*1");
    return EquationDto::new(&format!("x = {a} * ({b} {sign_a} {c})"));
}