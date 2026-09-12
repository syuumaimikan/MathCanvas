use mathcanvas_core::ast::Expression;
use mathcanvas_core::evaluator::{evaluate, Environment};

pub struct AdaptiveSampler;

impl AdaptiveSampler {
    pub fn sample(expr: &Expression, x_min: f64, x_max: f64, resolution: usize) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        let step = (x_max - x_min) / (resolution as f64);

        let mut env = Environment::default();

        for i in 0..=resolution {
            let x = x_min + step * (i as f64);
            env.variables.insert("x".to_string(), x);

            let y = evaluate(expr, &env);

            // Simple filtering of infinities or NaNs to prevent drawing artifacts
            if y.is_finite() {
                points.push((x, y));
            }
        }

        points
    }
}
