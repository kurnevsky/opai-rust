use ndarray::{Array1, Array3, Array4};

pub trait Model {
  type E;

  fn predict(&self, inputs: Array4<f64>) -> Result<(Array3<f64>, Array1<f64>), Self::E>;
}

pub trait TrainableModel: Model {
  fn train(&self, inputs: Array4<f64>, policies: Array3<f64>, values: Array1<f64>) -> Result<(), Self::E>;

  fn save(&self) -> Result<(), Self::E>;
}
