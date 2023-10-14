#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut result: Option<i32> = None;
    let mut stack: Vec<CalculatorInput> = Vec::new();
    let mut is_valid_calculation: bool = true;
    for input in inputs {
        let value: Option<i32> = match input {
            CalculatorInput::Add => calculation_add(&mut stack),
            CalculatorInput::Subtract => calculation_subtract(&mut stack),
            CalculatorInput::Multiply => calculation_multiply(&mut stack),
            CalculatorInput::Divide => calculation_divide(&mut stack),
            CalculatorInput::Value(number) => Some(number.clone()),
        };
        if value.is_some() {
            stack.push(CalculatorInput::Value(value.unwrap()));
        } else {
            is_valid_calculation = false;
            break;
        }
    }
    if stack.len() != 1 {
        is_valid_calculation = false;
    }
    if is_valid_calculation {
        match stack.pop() {
            Some(CalculatorInput::Value(number)) => result = Some(number),
            _ => result = None,
        }
    }
    return result;
}

pub fn calculation_add(stack: &mut Vec<CalculatorInput>) -> Option<i32> {
    let mut result: Option<i32> = None;
    let right_hand_side = match stack.pop() {
        Some(CalculatorInput::Value(number)) => Some(number),
        _ => None,
    };
    let left_hand_side = match stack.pop() {
        Some(CalculatorInput::Value(number)) => Some(number),
        _ => None,
    };
    if left_hand_side.is_some() && right_hand_side.is_some() {
        result = Some(left_hand_side.unwrap() + right_hand_side.unwrap());
    }
    return result;
}

pub fn calculation_subtract(stack: &mut Vec<CalculatorInput>) -> Option<i32> {
    let mut result: Option<i32> = None;
    let right_hand_side = match stack.pop() {
        Some(CalculatorInput::Value(number)) => Some(number),
        _ => None,
    };
    let left_hand_side = match stack.pop() {
        Some(CalculatorInput::Value(number)) => Some(number),
        _ => None,
    };
    if left_hand_side.is_some() && right_hand_side.is_some() {
        result = Some(left_hand_side.unwrap() - right_hand_side.unwrap());
    }
    return result;
}

pub fn calculation_multiply(stack: &mut Vec<CalculatorInput>) -> Option<i32> {
    let mut result: Option<i32> = None;
    let right_hand_side = match stack.pop() {
        Some(CalculatorInput::Value(number)) => Some(number),
        _ => None,
    };
    let left_hand_side = match stack.pop() {
        Some(CalculatorInput::Value(number)) => Some(number),
        _ => None,
    };
    if left_hand_side.is_some() && right_hand_side.is_some() {
        result = Some(left_hand_side.unwrap() * right_hand_side.unwrap());
    }
    return result;
}

pub fn calculation_divide(stack: &mut Vec<CalculatorInput>) -> Option<i32> {
    let mut result: Option<i32> = None;
    let right_hand_side = match stack.pop() {
        Some(CalculatorInput::Value(number)) => Some(number),
        _ => None,
    };
    let left_hand_side = match stack.pop() {
        Some(CalculatorInput::Value(number)) => Some(number),
        _ => None,
    };
    if left_hand_side.is_some() && right_hand_side.is_some() {
        result = Some(left_hand_side.unwrap() / right_hand_side.unwrap());
    }
    return result;
}

#[cfg(test)]
fn calculator_input(s: &str) -> Vec<CalculatorInput> {
    s.split_whitespace()
        .map(|s| match s {
            "+" => CalculatorInput::Add,
            "-" => CalculatorInput::Subtract,
            "*" => CalculatorInput::Multiply,
            "/" => CalculatorInput::Divide,
            n => CalculatorInput::Value(n.parse().unwrap()),
        })
        .collect()
}

#[test]
fn test_empty_input_returns_none() {
    let input = calculator_input("");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_simple_value() {
    let input = calculator_input("10");
    assert_eq!(evaluate(&input), Some(10));
}

#[test]
fn test_simple_addition() {
    let input = calculator_input("2 2 +");
    assert_eq!(evaluate(&input), Some(4));
}

#[test]
fn test_simple_subtraction() {
    let input = calculator_input("7 11 -");
    assert_eq!(evaluate(&input), Some(-4));
}

#[test]
fn test_simple_multiplication() {
    let input = calculator_input("6 9 *");
    assert_eq!(evaluate(&input), Some(54));
}

#[test]
fn test_simple_division() {
    let input = calculator_input("57 19 /");
    assert_eq!(evaluate(&input), Some(3));
}

#[test]
fn test_complex_operation() {
    let input = calculator_input("4 8 + 7 5 - /");
    assert_eq!(evaluate(&input), Some(6));
}

#[test]
fn test_too_few_operands_returns_none() {
    let input = calculator_input("2 +");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_too_many_operands_returns_none() {
    let input = calculator_input("2 2");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_zero_operands_returns_none() {
    let input = calculator_input("+");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_intermediate_error_returns_none() {
    let input = calculator_input("+ 2 2 *");
    assert_eq!(evaluate(&input), None);
}
