use kalkulator::Expression;

/// Tests basic arithmetic operation.
#[test]
fn test_basic_arithmetic() {
    let mut expr = Expression::new("3+2");
    expr.infix_to_postfix()
        .expect("Failed to convert to postfix");
    expr.compute_expression()
        .expect("Failed to compute expression!");
    assert_eq!(*expr.get_result().as_ref().unwrap(), 5.0);
}

/// Tests division and ensures correct handling of division by zero.
#[test]
fn test_division_by_zero() {
    let mut expr = Expression::new("10/0");
    expr.infix_to_postfix().unwrap(); // Conversion to postfix should succeed.
    let compute_result = expr.compute_expression();
    assert!(compute_result.is_err()); // Expect an error during computation.
}

/// Tests expression involving the factorial operation.
#[test]
fn test_factorial_operation() {
    let mut expr = Expression::new("5!");
    expr.infix_to_postfix()
        .expect("Failed to convert to postfix");
    expr.compute_expression()
        .expect("Failed to compute expression");
    assert_eq!(*expr.get_result().as_ref().unwrap(), 120.0);
}

/// Tests complex expression combining various operations.
#[test]
fn test_complex_expression() {
    let mut expr = Expression::new("3+4*2/(1-5)+2*3");
    expr.infix_to_postfix()
        .expect("Failed to convert to postfix");
    expr.compute_expression()
        .expect("Failed to compute expression");

    assert_eq!(*expr.get_result().as_ref().unwrap(), 7.0);
}

/// Tests exponentiation and factorial combined
#[test]
fn test_exponentiation_factorial() {
    let mut expr = Expression::new("(2*3)! + 2! + 2^3");
    expr.infix_to_postfix()
        .expect("Failed to convert to postfix");
    expr.compute_expression()
        .expect("Failed to compute expression");
    assert_eq!(*expr.get_result().as_ref().unwrap(), 730.0);
}
