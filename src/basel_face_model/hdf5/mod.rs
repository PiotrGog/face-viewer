pub mod model_params;
pub mod morphable_model;

use model_params::ModelParams;

pub fn load_from_file(file: &str) -> hdf5::Result<morphable_model::MorphableModel> {
    Ok(morphable_model::MorphableModel::load_from_file(file)?)
}
