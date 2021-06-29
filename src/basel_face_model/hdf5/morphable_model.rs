use super::ModelParams;
use ndarray::Array1;
use std::sync::{Arc, Mutex};

pub struct MorphableModel {
    pub color: ModelParams,
    pub expression: ModelParams,
    pub shape: ModelParams,
}

impl MorphableModel {
    pub fn load_from_file(file_path: &str) -> hdf5::Result<MorphableModel> {
        // so that libhdf5 doesn't print errors to stdout
        let _e = hdf5::silence_errors();

        let file = hdf5::File::open(file_path)?;

        Ok(MorphableModel {
            color: ModelParams::load_from_file("color", &file)?,
            expression: ModelParams::load_from_file("expression", &file)?,
            shape: ModelParams::load_from_file("shape", &file)?,
        })
    }

    pub fn calculate_color(&self, coefficients: Arc<Mutex<ndarray::Array1<f32>>>) -> Array1<f32> {
        Self::calc_model_param(&self.color, coefficients)
    }
    pub fn calculate_expression(
        &self,
        coefficients: Arc<Mutex<ndarray::Array1<f32>>>,
    ) -> Array1<f32> {
        Self::calc_model_param(&self.expression, coefficients)
    }
    pub fn calculate_shape(&self, coefficients: Arc<Mutex<ndarray::Array1<f32>>>) -> Array1<f32> {
        Self::calc_model_param(&self.shape, coefficients)
    }

    fn calc_model_param(
        model_param: &ModelParams,
        coefficients: Arc<Mutex<ndarray::Array1<f32>>>,
    ) -> Array1<f32> {
        let multipled_coefficients = &model_param.pca_variance * &*coefficients.lock().unwrap();
        model_param.rescaled_pca_basis.dot(&multipled_coefficients) + &model_param.mean
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn morphable_model_test() {
        assert!(false);
    }
}
