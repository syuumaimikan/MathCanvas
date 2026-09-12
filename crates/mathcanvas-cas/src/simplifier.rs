use mathcanvas_core::ast::Expression;

pub struct Simplifier;

impl Simplifier {
    pub fn simplify(expr: &Expression) -> Expression {
        match expr {
            Expression::Add(a, b) => {
                let sa = Self::simplify(a);
                let sb = Self::simplify(b);

                // x + 0 = x
                if let Expression::Number(n) = sb {
                    if n == 0.0 {
                        return sa;
                    }
                }
                if let Expression::Number(n) = sa {
                    if n == 0.0 {
                        return sb;
                    }
                    // Constant folding
                    if let Expression::Number(m) = sb {
                        return Expression::Number(n + m);
                    }
                }

                // x + x = 2*x
                if sa == sb {
                    return Expression::Multiply(Box::new(Expression::Number(2.0)), Box::new(sa));
                }

                Expression::Add(Box::new(sa), Box::new(sb))
            }
            Expression::Subtract(a, b) => {
                let sa = Self::simplify(a);
                let sb = Self::simplify(b);

                // x - 0 = x
                if let Expression::Number(n) = sb {
                    if n == 0.0 {
                        return sa;
                    }
                }

                // x - x = 0
                if sa == sb {
                    return Expression::Number(0.0);
                }

                // Constant folding
                if let (Expression::Number(n), Expression::Number(m)) = (&sa, &sb) {
                    return Expression::Number(n - m);
                }

                Expression::Subtract(Box::new(sa), Box::new(sb))
            }
            Expression::Multiply(a, b) => {
                let sa = Self::simplify(a);
                let sb = Self::simplify(b);

                // x * 0 = 0
                if let Expression::Number(n) = sa {
                    if n == 0.0 {
                        return Expression::Number(0.0);
                    }
                    if n == 1.0 {
                        return sb.clone();
                    }
                }
                if let Expression::Number(n) = sb {
                    if n == 0.0 {
                        return Expression::Number(0.0);
                    }
                    if n == 1.0 {
                        return sa.clone();
                    }
                }

                // Constant folding
                if let (Expression::Number(n), Expression::Number(m)) = (&sa, &sb) {
                    return Expression::Number(n * m);
                }

                // x * x = x^2
                if sa == sb {
                    return Expression::Power(Box::new(sa), Box::new(Expression::Number(2.0)));
                }

                Expression::Multiply(Box::new(sa), Box::new(sb))
            }
            Expression::Divide(a, b) => {
                let sa = Self::simplify(a);
                let sb = Self::simplify(b);

                // 0 / x = 0
                if let Expression::Number(n) = sa {
                    if n == 0.0 {
                        return Expression::Number(0.0);
                    }
                }

                // x / 1 = x
                if let Expression::Number(n) = sb {
                    if n == 1.0 {
                        return sa.clone();
                    }
                }

                // x / x = 1 (assuming x != 0)
                if sa == sb {
                    return Expression::Number(1.0);
                }

                // Constant folding
                if let (Expression::Number(n), Expression::Number(m)) = (&sa, &sb) {
                    return Expression::Number(n / m);
                }

                Expression::Divide(Box::new(sa), Box::new(sb))
            }
            Expression::Power(base, exp) => {
                let sbase = Self::simplify(base);
                let sexp = Self::simplify(exp);

                // x^0 = 1
                if let Expression::Number(n) = sexp {
                    if n == 0.0 {
                        return Expression::Number(1.0);
                    }
                    if n == 1.0 {
                        return sbase.clone();
                    }
                }

                // 0^x = 0
                if let Expression::Number(n) = sbase {
                    if n == 0.0 {
                        return Expression::Number(0.0);
                    }
                    if n == 1.0 {
                        return Expression::Number(1.0);
                    }
                }

                // Constant folding
                if let (Expression::Number(n), Expression::Number(m)) = (&sbase, &sexp) {
                    return Expression::Number(n.powf(*m));
                }

                Expression::Power(Box::new(sbase), Box::new(sexp))
            }
            Expression::FunctionCall(name, args) => {
                let sargs: Vec<Expression> = args.iter().map(Self::simplify).collect();
                Expression::FunctionCall(name.clone(), sargs)
            }
            _ => expr.clone(),
        }
    }
}
