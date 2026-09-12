use crate::ast::Expression;
use std::collections::HashMap;

#[derive(Default)]
pub struct Environment {
    pub variables: HashMap<String, f64>,
}

pub fn evaluate(expr: &Expression, env: &Environment) -> f64 {
    match expr {
        Expression::Number(n) => *n,
        Expression::Variable(name) => {
            if name == "pi" || name == "π" {
                std::f64::consts::PI
            } else if name == "e" {
                std::f64::consts::E
            } else {
                *env.variables.get(name).unwrap_or(&0.0)
            }
        }
        Expression::Add(a, b) => evaluate(a, env) + evaluate(b, env),
        Expression::Subtract(a, b) => evaluate(a, env) - evaluate(b, env),
        Expression::Multiply(a, b) => evaluate(a, env) * evaluate(b, env),
        Expression::Divide(a, b) => evaluate(a, env) / evaluate(b, env),
        Expression::Power(base, exponent) => evaluate(base, env).powf(evaluate(exponent, env)),
        Expression::FunctionCall(name, args) => {
            let eval_args: Vec<f64> = args.iter().map(|arg| evaluate(arg, env)).collect();
            match name.as_str() {
                "sin" => eval_args.first().copied().unwrap_or(0.0).sin(),
                "cos" => eval_args.first().copied().unwrap_or(0.0).cos(),
                "tan" => eval_args.first().copied().unwrap_or(0.0).tan(),
                "sqrt" | "√" => eval_args.first().copied().unwrap_or(0.0).sqrt(),
                "ln" => eval_args.first().copied().unwrap_or(0.0).ln(),
                "log" => eval_args.first().copied().unwrap_or(0.0).log10(),
                _ => 0.0, // Fallback for unknown functions
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_evaluation() {
        let env = Environment::default();
        let expr = Expression::Add(
            Box::new(Expression::Number(2.0)),
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(3.0)),
                Box::new(Expression::Number(4.0)),
            )),
        );
        assert_eq!(evaluate(&expr, &env), 14.0);
    }
}
