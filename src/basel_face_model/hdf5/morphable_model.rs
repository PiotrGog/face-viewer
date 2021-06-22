use super::ModelParams;
use ndarray::Array1;

pub struct MorphableModel {
    pub color: ModelParams,
    pub expression: ModelParams,
    pub shape: ModelParams,
    valexpr: f32,
    valshape: f32,
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
            valexpr: -0.1,
            valshape: -0.1,
        })
    }

    pub fn calculate_color(&self) -> Array1<f32> {
        Self::calc_model_param_c(&self.color)
    }
    pub fn calculate_expression(&mut self) -> Array1<f32> {
        self.valexpr += 0.02;
        if self.valexpr > 1.0 {
            self.valexpr = -1.0
        }
        Self::calc_model_param(self.valexpr, &self.expression)
    }
    pub fn calculate_shape(&mut self) -> Array1<f32> {
        self.valshape += 0.02;
        if self.valshape > 1.0 {
            self.valshape = -1.0
        }
        Self::calc_model_param(self.valshape, &self.shape)
    }

    fn calc_model_param(val: f32, model_param: &ModelParams) -> Array1<f32> {
        model_param
            .rescaled_pca_basis
            .dot(&model_param.pca_variance.map(|x| x * val))
            + &model_param.mean
    }

    fn calc_model_param_c(model_param: &ModelParams) -> Array1<f32> {
        model_param
            .rescaled_pca_basis
            .dot(&model_param.pca_variance)
            + &model_param.mean
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn morphable_model_test() {
        assert!(false);
    }
}
