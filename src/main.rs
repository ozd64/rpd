use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Display;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OperationType {
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION
}

impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::ADDITION => write!(f, "+"),
            OperationType::SUBTRACTION => write!(f, "-"),
            OperationType::MULTIPLICATION => write!(f, "*"),
            OperationType::DIVISION => write!(f, "/")
        }
    }
}

enum PolishNotationToken {
    Operation(OperationType),
    Number(u32),
    Space
}

#[derive(Debug, PartialEq, Eq)]
enum TokenError {
    InvalidCharacter(usize, char),
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::InvalidCharacter(index, ch) => write!(f, "Invalid character at position {}, \"{}\"", index + 1, ch),
        }
    }
}

impl Error for TokenError {}

#[derive(Debug, PartialEq, Eq)]
enum CalculationError {
   NoNumberFoundForOperation(usize, OperationType),
   NoResultAvailable(&'static str),
   IncompleteExpression(usize)
}

impl Display for CalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculationError::NoNumberFoundForOperation(pos, op) => write!(f, "No number found before the operation {} at position {}", op, pos + 1),
            CalculationError::NoResultAvailable(error_msg) => write!(f, "{}", error_msg),
            CalculationError::IncompleteExpression(stack_size) => write!(f, "Incomplete expression. {} tokens unprocessed.", stack_size - 1)
        }
    }
}

impl Error for CalculationError {}


fn main() {
    let mut input = String::new();

    print!("Enter the reversed polish notation: ");

    if let Err(err) = io::stdout().flush() {
        eprintln!("An error occurred while flushing standard output!\n{}", err);
    }

    if let Err(err) = io::stdin().read_line(&mut input) {
        eprintln!("An error occurred while reading the input!\n{}", err);
    }


    let token_result: Result<Vec<(usize, PolishNotationToken)>, TokenError> = input.trim().chars().enumerate()
        .map(|(pos, ch)| parse_rpd_token(pos, ch))
        .collect();

    match token_result {
        Ok(tokens) => {
            match calculate_rpd(tokens) {
                Ok(result) => println!("Result: {}", result),
                Err(err) => eprintln!("An error occurred while calculating reversed polish notation. {}", err)
            }
        },
        Err(token_error) =>
            eprintln!("An error occurred while evaluating reversed polish notation. {}", token_error)
    }

}

fn calculate_rpd(tokens: Vec<(usize, PolishNotationToken)>) -> Result<u32, CalculationError> {

    let mut stack = VecDeque::<u32>::new();

    for token in tokens {
        match token.1 {
            PolishNotationToken::Operation(op) => apply_op(token.0, op, &mut stack)?,
            PolishNotationToken::Number(num) => stack.push_back(num),
            PolishNotationToken::Space => continue,
        } 
    }

    if stack.len() > 1 {
        return Err(CalculationError::IncompleteExpression(stack.len()));
    }

    stack
        .pop_back()
        .ok_or_else(|| CalculationError::NoResultAvailable("No result can be generated."))
}

fn apply_op(op_pos: usize, op_type: OperationType, stack: &mut VecDeque<u32>) -> Result<(), CalculationError> {
    match op_type {
        OperationType::ADDITION => {
            let y = stack.pop_back().ok_or_else(|| CalculationError::NoNumberFoundForOperation(op_pos, op_type))?;
            let x = stack.pop_back().ok_or_else(|| CalculationError::NoNumberFoundForOperation(op_pos, op_type))?;

            Ok(stack.push_back(x + y))
        },
        OperationType::SUBTRACTION => {
            let y = stack.pop_back().ok_or_else(|| CalculationError::NoNumberFoundForOperation(op_pos, op_type))?;
            let x = stack.pop_back().ok_or_else(|| CalculationError::NoNumberFoundForOperation(op_pos, op_type))?;

            Ok(stack.push_back(x - y))
        },
        OperationType::MULTIPLICATION => {
            let y = stack.pop_back().ok_or_else(|| CalculationError::NoNumberFoundForOperation(op_pos, op_type))?;
            let x = stack.pop_back().ok_or_else(|| CalculationError::NoNumberFoundForOperation(op_pos, op_type))?;

            Ok(stack.push_back(x * y))
        },
        OperationType::DIVISION => {
            let y = stack.pop_back().ok_or_else(|| CalculationError::NoNumberFoundForOperation(op_pos, op_type))?;
            let x = stack.pop_back().ok_or_else(|| CalculationError::NoNumberFoundForOperation(op_pos, op_type))?;

            Ok(stack.push_back(x / y))
        }
    }
}

fn parse_rpd_token(index: usize, ch: char) -> Result<(usize, PolishNotationToken), TokenError> {
    match ch {
        '+' => Ok((index, PolishNotationToken::Operation(OperationType::ADDITION))),
        '-' => Ok((index, PolishNotationToken::Operation(OperationType::SUBTRACTION))),
        '*' => Ok((index, PolishNotationToken::Operation(OperationType::MULTIPLICATION))),
        'x' => Ok((index, PolishNotationToken::Operation(OperationType::MULTIPLICATION))),
        '/' => Ok((index, PolishNotationToken::Operation(OperationType::DIVISION))),
        '0' ..= '9' => Ok((index, PolishNotationToken::Number(ch.to_digit(10).unwrap()))),
        ' ' => Ok((index, PolishNotationToken::Space)),
        _ => Err(TokenError::InvalidCharacter(index, ch))

    }
}
