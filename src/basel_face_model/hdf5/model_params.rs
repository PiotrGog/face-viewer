use hdf5::H5Type;
use ndarray::{Array, Array1, Array2, Dimension};

#[allow(unused)]
pub struct ModelParams {
    pub mean: Array1<f32>,
    pub noise_variance: Array1<f32>,
    pub pca_basis: Array2<f32>,
    pub pca_variance: Array1<f32>,
    pub representer_cells: Array2<u32>,
    pub representer_points: Array2<f32>,
    pub rescaled_pca_basis: Array2<f32>,
}

impl ModelParams {
    pub fn load_from_file(param: &str, file: &hdf5::File) -> hdf5::Result<ModelParams> {
        let pca_basis: Array2<f32> =
            Self::load_array(&file, &format!("{}{}", param, "/model/pcaBasis"))?;
        let pca_variance: Array1<f32> =
            Self::load_array(&file, &format!("{}{}", param, "/model/pcaVariance"))?;
        let sqrt_of_variance = pca_variance.map(|x| x.sqrt());
        let rescaled_pca_basis = &pca_basis / &sqrt_of_variance;
        Ok(ModelParams {
            mean: Self::load_array(&file, &format!("{}{}", param, "/model/mean"))?,
            noise_variance: Self::load_array(
                &file,
                &format!("{}{}", param, "/model/noiseVariance"),
            )?,
            pca_basis: pca_basis,
            pca_variance: pca_variance,
            representer_cells: Self::load_array(
                &file,
                &format!("{}{}", param, "/representer/cells"),
            )?,
            representer_points: Self::load_array(
                &file,
                &format!("{}{}", param, "/representer/points"),
            )?,
            rescaled_pca_basis: rescaled_pca_basis,
        })
    }

    fn load_array<Dim, T>(file: &hdf5::File, path: &str) -> hdf5::Result<Array<T, Dim>>
    where
        Dim: Dimension,
        T: H5Type,
    {
        let arr: Array<T, Dim> = file
            .dataset(path)?
            .read_dyn::<T>()?
            .into_dimensionality::<Dim>()
            .expect("msg");
        Ok(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn model_params_test() {
        assert!(false);
    }
}
