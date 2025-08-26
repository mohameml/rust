use crate::options::option::Option;

pub struct AsianOption {
    pub strike: f64,
}

impl AsianOption {
    pub fn new(strike: f64) -> Self {
        AsianOption { strike: strike }
    }
}

impl Option for AsianOption {
    fn payoff(&self, path: &ndarray::Array2<f64>) -> f64 {
        let avg = path.column(0).mean().unwrap();

        return (avg - self.strike).max(0.0);
    }
}
