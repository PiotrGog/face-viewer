use super::ModelParams;
use ndarray::Array1;

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

    pub fn calculate_color(&self) -> Array1<f32> {
        Self::calc_model_param(&self.color)
    }
    pub fn calculate_expression(&self) -> Array1<f32> {
        Self::calc_model_param(&self.expression)
    }
    pub fn calculate_shape(&self) -> Array1<f32> {
        Self::calc_model_param(&self.shape)
    }

    fn calc_model_param(model_param: &ModelParams) -> Array1<f32> {
        let mut result = model_param.pca_basis.dot(&model_param.pca_variance);
        result += &model_param.mean;
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn morphable_model_test() {
        assert!(false);
    }
}
