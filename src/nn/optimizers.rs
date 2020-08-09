use ndarray::{Array, Array2};

pub trait Optimizer {
    fn learning_rate(&self) -> f64;
    fn delta(&self, index: usize, gradient: Array2<f64>) -> Array2<f64>;
}

pub struct SGD {
    momentum: f64,
    learning_rate: f64,
    velocities: Vec<Array2<f64>>,
}

impl SGD {
    pub fn new(momentum: f64, learning_rate: f64) -> SGD {
        SGD {
            momentum: momentum,
            learning_rate: learning_rate,
            velocities: vec![],
        }
    }
}

impl Optimizer for SGD {
    fn learning_rate(&self) -> f64 {
        self.learning_rate
    }

    fn delta(&self, index: usize, gradient: Array2<f64>) -> Array2<f64> {
        if self.velocities.len() <= index {
            self.velocities.push(Array::zeros(gradient.dim()));
        };

        let moment: Array2<f64> =
            (self.velocities[index] * self.momentum) + (gradient * self.learning_rate);

        self.velocities[index].assign(&moment);
        moment
    }
}