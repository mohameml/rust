use ndarray::{Array1, Array2};
use rand::Rng;
use serde_json::Value;

pub struct BlackScholesModel {
    pub model_size: usize,       // nombre d'actifs du modèle
    pub interest_rate: f64,      // taux d'intérêt
    pub correlation: f64,        // paramètre de corrélation
    pub volatility: Array1<f64>, // vecteur de volatilités
    pub spots: Array1<f64>,      // valeurs initiales des sous-jacents
    pub l: Array2<f64>,          // racine carrée de matrice de corrélation
    pub time_step: f64,          // time step = T / N
    pub g: Array1<f64>,          // vecteur pour simulation
}

impl BlackScholesModel {
    pub fn new() -> Self {
        // À adapter selon tes besoins
        BlackScholesModel {
            model_size: 0,
            interest_rate: 0.0,
            correlation: 0.0,
            volatility: Array1::zeros(0),
            spots: Array1::zeros(0),
            l: Array2::zeros((0, 0)),
            time_step: 0.0,
            g: Array1::zeros(0),
        }
    }
    /// Génère une trajectoire du modèle et la stocke dans path
    pub fn asset(&self, path: &mut Array2<f64>, rng: &mut impl Rng) {
        // À implémenter
    }

    /// Génère une trajectoire du modèle et la stocke dans path (simulation conditionnelle)
    pub fn asset_conditional(
        &self,
        past: &Array2<f64>,
        t: f64,
        t_maturity: f64,
        path: &mut Array2<f64>,
        rng: &mut impl Rng,
    ) {
        // À implémenter
    }

    /// Simuler 2 trajectoires utilisant les mêmes aléas Browniens mais shiftées l’une par rapport à l’autre
    pub fn shift_asset(&self, d: usize, h: f64, original_paths: &mut Array2<f64>) {
        // À implémenter
    }

    pub fn shift_asset_with_time(
        &self,
        d: usize,
        t: f64,
        h: f64,
        original_paths: &mut Array2<f64>,
    ) {
        // À implémenter
    }
}

impl BlackScholesModel {
    pub fn from_json(json: &Value) -> Self {
        let t = json["maturity"].as_f64().unwrap();
        let n = json["fixing dates number"].as_f64().unwrap();
        let time_step = t / n;

        let model_size = json["option size"].as_u64().unwrap() as usize;
        let interest_rate = json["interest rate"].as_f64().unwrap();
        let correlation = json["correlation"].as_f64().unwrap();

        // Volatility
        let mut volatility: Array1<f64> = {
            let arr = json["volatility"].as_array().unwrap();
            let mut v: Vec<f64> = arr.iter().map(|x| x.as_f64().unwrap()).collect();
            if v.len() == 1 && model_size > 1 {
                v = vec![v[0]; model_size];
            }
            Array1::from(v)
        };

        // Spots
        let mut spots: Array1<f64> = {
            let arr = json["spot"].as_array().unwrap();
            let mut v: Vec<f64> = arr.iter().map(|x| x.as_f64().unwrap()).collect();
            if v.len() == 1 && model_size > 1 {
                v = vec![v[0]; model_size];
            }
            Array1::from(v)
        };

        // Matrice de corrélation
        let mut l = Array2::<f64>::from_elem((model_size, model_size), correlation);
        for i in 0..model_size {
            l[(i, i)] = 1.0;
        }
        // À remplacer par une vraie décomposition de Cholesky si besoin
        // l = cholesky(&l);

        let g = Array1::<f64>::zeros(model_size);

        BlackScholesModel {
            model_size,
            interest_rate,
            correlation,
            volatility,
            spots,
            l,
            time_step,
            g,
        }
    }
}
