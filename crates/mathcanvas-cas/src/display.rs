use mathcanvas_core::ast::Expression;

pub fn format_expression(expr: &Expression) -> String {
    match expr {
        Expression::Number(n) => n.to_string(),
        Expression::Variable(v) => v.clone(),
        Expression::Add(a, b) => format!("{} + {}", format_expression(a), format_expression(b)),
        Expression::Subtract(a, b) => {
            format!("{} - {}", format_expression(a), format_expression(b))
        }
        Expression::Multiply(a, b) => {
            let left = match **a {
                Expression::Add(..) | Expression::Subtract(..) => {
                    format!("({})", format_expression(a))
                }
                _ => format_expression(a),
            };
            let right = match **b {
                Expression::Add(..) | Expression::Subtract(..) => {
                    format!("({})", format_expression(b))
                }
                _ => format_expression(b),
            };
            format!("{} * {}", left, right)
        }
        Expression::Divide(a, b) => {
            let left = match **a {
                Expression::Add(..) | Expression::Subtract(..) => {
                    format!("({})", format_expression(a))
                }
                _ => format_expression(a),
            };
            let right = match **b {
                Expression::Add(..)
                | Expression::Subtract(..)
                | Expression::Multiply(..)
                | Expression::Divide(..) => format!("({})", format_expression(b)),
                _ => format_expression(b),
            };
            format!("{} / {}", left, right)
        }
        Expression::Power(a, b) => {
            let left = match **a {
                Expression::Number(..)
                | Expression::Variable(..)
                | Expression::FunctionCall(..) => format_expression(a),
                _ => format!("({})", format_expression(a)),
            };
            format!("{}^{}", left, format_expression(b))
        }
        Expression::FunctionCall(name, args) => {
            let args_str: Vec<String> = args.iter().map(format_expression).collect();
            format!("{}({})", name, args_str.join(", "))
        }
    }
}
