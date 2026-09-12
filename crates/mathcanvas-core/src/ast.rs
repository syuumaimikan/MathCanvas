#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

impl Expression {
    pub fn to_latex(&self) -> String {
        match self {
            Expression::Number(n) => n.to_string(),
            Expression::Variable(v) => v.clone(),
            Expression::Add(a, b) => format!("{} + {}", a.to_latex(), b.to_latex()),
            Expression::Subtract(a, b) => format!("{} - {}", a.to_latex(), b.to_latex()),
            Expression::Multiply(a, b) => format!(r"{} \cdot {}", a.to_latex(), b.to_latex()),
            Expression::Divide(a, b) => format!(r"\frac{{{}}}{{{}}}", a.to_latex(), b.to_latex()),
            Expression::Power(a, b) => format!("{{{}}}^{{{}}}", a.to_latex(), b.to_latex()),
            Expression::FunctionCall(name, args) => {
                let args_str: Vec<String> = args.iter().map(|arg| arg.to_latex()).collect();
                format!(r"\{}({})", name, args_str.join(", "))
            }
        }
    }

    pub fn to_python(&self) -> String {
        match self {
            Expression::Number(n) => n.to_string(),
            Expression::Variable(v) => v.clone(),
            Expression::Add(a, b) => format!("({} + {})", a.to_python(), b.to_python()),
            Expression::Subtract(a, b) => format!("({} - {})", a.to_python(), b.to_python()),
            Expression::Multiply(a, b) => format!("({} * {})", a.to_python(), b.to_python()),
            Expression::Divide(a, b) => format!("({} / {})", a.to_python(), b.to_python()),
            Expression::Power(a, b) => format!("({} ** {})", a.to_python(), b.to_python()),
            Expression::FunctionCall(name, args) => {
                let args_str: Vec<String> = args.iter().map(|arg| arg.to_python()).collect();
                let py_name = match name.as_str() {
                    "sin" => "math.sin",
                    "cos" => "math.cos",
                    "tan" => "math.tan",
                    "log" => "math.log",
                    _ => name.as_str()
                };
                format!("{}({})", py_name, args_str.join(", "))
            }
        }
    }
}
