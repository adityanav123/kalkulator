//! Command-line interface for the `kalkulator` library.
//!
//! This interface allows users to either convert mathematical expressions to postfix notation or evaluate them directly.
//!
//! # Usage
//!
//! To convert an expression to postfix notation without evaluating:
//! ```
//! kalkulator --expr "2+3/4" -p
//! ```
//!
//! To evaluate the result of an expression:
//! ```
//! kalkulator --expr "2+3/4"
//! ```

//! # Examples
//!
//! Basic usage of the `Expression` struct to evaluate an arithmetic expression:
//!
//! ```
//! use kalkulator::Expression;
//!
//! let mut expr = Expression::new("3+4*2");
//! expr.infix_to_postfix().unwrap(); // Converts to postfix notation
//! expr.compute_expression().unwrap(); // Evaluates the expression
//! assert_eq!(expr.get_result().unwrap(), 11); // The result is 11
//! ```
//!
//! Using the `Expression` struct to evaluate an expression with factorial and division:
//!
//! ```
//! use kalkulator::Expression;
//!
//! let mut expr = Expression::new("4!/(2+3)");
//! expr.infix_to_postfix().unwrap(); // Converts to postfix notation
//! expr.compute_expression().unwrap(); // Evaluates the expression
//! assert_eq!(expr.get_result().unwrap(), 24); // The result is 24 (120 / 5)
//! ```
//!
//! Evaluating an expression involving all supported operations:
//!
//! ```
//! use kalkulator::Expression;
//!
//! let mut expr = Expression::new("2+3*4-5/2");
//! expr.infix_to_postfix().unwrap();
//! expr.compute_expression().unwrap();
//! assert_eq!(expr.get_result().unwrap(), 11); // The result is 11
//! ```

use std::collections::VecDeque;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

__lazy_static_internal!(@MAKE TY, ,(pub),FACTORIAL_CACHE);
__lazy_static_internal!(@TAIL,FACTORIAL_CACHE:Mutex<Vec<i64>>  = Mutex::new(vec![1,1]));
lazy_static!();

/// Represents an arithmetic expression, its postfix notation, and computation result.
pub struct Expression {
    pub expr: String,
    pub post_fix: String,
    pub result: Result<i64, ErrorKind>,
}

/// Enumerates possible errors that can occur during expression parsing and evaluation.
#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidExpression,
    InsufficientOperands,
    DivisionByZero,
    Overflow,
    InvalidToken,
    MalformedExpression,
}

impl ErrorKind {
    /// Returns a description of the error
    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::InvalidExpression => "Invalid expression",
            ErrorKind::InsufficientOperands => "Insufficient operands",
            ErrorKind::DivisionByZero => "Division by zero",
            ErrorKind::Overflow => "Overflow",
            ErrorKind::InvalidToken => "Invalid token",
            ErrorKind::MalformedExpression => "Malformed postfix expression",
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Expression {
    /// Constructs a new 'Expression' instance from a String Slice
    pub fn new(expr: &str) -> Expression {
        Self {
            expr: expr.to_string(),
            post_fix: String::new(),
            result: Err(ErrorKind::InvalidExpression),
        }
    }

    /// Determines the precedence of operators to aid in postfix conversion
    pub fn precedence(operator: char) -> u8 {
        match operator {
            '+' | '-' => 1,
            '*' | '/' => 2,
            _ => 0, // highest precedence
        }
    }

    /// Converts the stored infix expression to postfix notation
    pub fn infix_to_postfix(&mut self) -> Result<(), ErrorKind> {
        let mut output_queue = VecDeque::new();
        let mut stack = Vec::new();
        let mut number_buffer = Vec::new();

        let mut tokens = self.expr.chars().peekable();

        while let Some(&ch) = tokens.peek() {
            match ch {
                '0'..='9' => {
                    number_buffer.push(ch);
                    tokens.next();
                }
                '!' => {
                    Self::flush_num_buffer(&mut number_buffer, &mut output_queue);
                    output_queue.push_back(String::from("!"));
                    tokens.next();
                }
                ' ' => {
                    Self::flush_num_buffer(&mut number_buffer, &mut output_queue);
                    tokens.next();
                }
                '+' | '-' | '*' | '/' => {
                    Self::flush_num_buffer(&mut number_buffer, &mut output_queue);
                    while let Some(&operation) = stack.last() {
                        if operation == '(' || !Self::has_higher_precedence(operation, ch) {
                            break;
                        }
                        output_queue.push_back(stack.pop().unwrap().to_string());
                    }
                    stack.push(ch);
                    tokens.next();
                }
                '(' => {
                    stack.push(ch);
                    tokens.next();
                }
                ')' => {
                    Self::flush_num_buffer(&mut number_buffer, &mut output_queue);
                    while let Some(op) = stack.pop() {
                        if op == '(' {
                            break;
                        }
                        output_queue.push_back(op.to_string());
                    }
                    tokens.next();
                }
                _ => return Err(ErrorKind::InvalidExpression),
            }
        }

        Self::flush_num_buffer(&mut number_buffer, &mut output_queue);

        while let Some(op) = stack.pop() {
            if op == '(' {
                return Err(ErrorKind::MalformedExpression); // Handle unbalanced parentheses
            }
            output_queue.push_back(op.to_string());
        }

        self.post_fix = output_queue.into_iter().collect::<Vec<_>>().join(" ");
        Ok(())
    }

    fn flush_num_buffer(number_buffer: &mut Vec<char>, output: &mut VecDeque<String>) {
        if !number_buffer.is_empty() {
            let num = number_buffer.iter().collect::<String>();
            output.push_back(num);
            number_buffer.clear();
        }
    }

    /// Returns String slice to the Postfix conversion
    pub fn show_post_fix(&self) -> &str {
        &self.post_fix
    }

    /// Checks if the first operator has higher precendence over the second
    pub fn has_higher_precedence(op1: char, op2: char) -> bool {
        Self::precedence(op1) > Self::precedence(op2)
    }

    /// Evaluates the postfix expression and stores the 'result'
    pub fn compute_expression(&mut self) -> Result<(), ErrorKind> {
        let post_fix_vector: Vec<&str> = self.post_fix.split_whitespace().collect();
        let mut stack: Vec<i64> = Vec::new();

        for token in post_fix_vector.iter() {
            match *token {
                "+" | "-" | "*" | "/" => {
                    if stack.len() < 2 {
                        return Err(ErrorKind::InsufficientOperands);
                    }
                    let operand2 = stack.pop().unwrap();
                    let operand1 = stack.pop().unwrap();

                    let result = match *token {
                        "+" => operand1 + operand2,
                        "-" => operand1 - operand2,
                        "*" => operand1 * operand2,
                        "/" => {
                            if operand2 == 0 {
                                return Err(ErrorKind::DivisionByZero);
                            }
                            operand1 / operand2
                        }
                        _ => unreachable!(), // Already validated during infix to postfix conversion
                    };
                    stack.push(result);
                }
                "!" => {
                    if let Some(a) = stack.pop() {
                        let fact = Self::factorial(a as usize)?;
                        stack.push(fact);
                    } else {
                        return Err(ErrorKind::InsufficientOperands);
                    }
                }
                _ => {
                    if let Ok(num) = token.parse::<i64>() {
                        stack.push(num);
                    } else {
                        return Err(ErrorKind::InvalidToken);
                    }
                }
            }
        }

        if stack.len() == 1 {
            self.result = Ok(stack.pop().unwrap());
        } else {
            return Err(ErrorKind::MalformedExpression);
        }

        Ok(())
    }

    /// Computes factorial of a number, utilizing a cache to improve performance
    fn factorial(n: usize) -> Result<i64, ErrorKind> {
        let mut cache = FACTORIAL_CACHE.lock().map_err(|_| ErrorKind::Overflow)?;

        if n >= cache.len() {
            for i in cache.len()..=n {
                let last = *cache.last().unwrap();
                match last.checked_mul(i as i64) {
                    Some(result) => cache.push(result),
                    None => return Err(ErrorKind::Overflow),
                }
            }
        }

        cache.get(n).copied().ok_or(ErrorKind::InvalidExpression)
    }

    /// Returns reference to the computation 'Result<>'
    pub fn get_result(&self) -> &Result<i64, ErrorKind> {
        &self.result
    }

    /// Process the expression by converting it to postfix and optionally computing it
    pub fn process_expression(&mut self, compute: bool) -> Result<(), ErrorKind> {
        self.infix_to_postfix()?;

        if !compute {
            println!("Postfix: [{}]", self.show_post_fix());
        }
        if compute {
            self.compute_expression()?;
            match self.get_result() {
                Ok(result) => println!("Result = {}", result),
                Err(e) => println!("Error: {}", e),
            }
        }

        Ok(())
    }
}
