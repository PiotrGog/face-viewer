use face_viewer::{basel_face_model, view_3d::create_and_run_window};

fn main() -> hdf5::Result<()> {
    // create_and_run_window();
    let model =
        basel_face_model::hdf5::load_from_file("resources/basel_face_model/model2019_bfm.h5")?;
    Ok(())
}
