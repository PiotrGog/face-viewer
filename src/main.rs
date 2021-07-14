use face_viewer::{basel_face_model, view_3d};

use std::sync::{atomic::AtomicBool, Arc, Mutex};
use std::thread;

fn main() {
    let model =
        basel_face_model::hdf5::load_from_file("resources/basel_face_model/model2019_bfm.h5")
            .expect("Failed to load Basel Face Model");

    let is_running = Arc::new(AtomicBool::new(true));
    let model_updated = Arc::new(AtomicBool::new(false));

    let color = Arc::new(Mutex::new(ndarray::Array1::zeros(199)));
    let expression = Arc::new(Mutex::new(ndarray::Array1::zeros(100)));
    let shape = Arc::new(Mutex::new(ndarray::Array1::zeros(199)));

    let color_control_panel_thread_handle = Arc::clone(&color);
    let expression_control_panel_thread_handle = Arc::clone(&expression);
    let shape_control_panel_thread_handle = Arc::clone(&shape);
    let is_running_control_panel_thread_handle = Arc::clone(&is_running);
    let model_updated_control_panel_thread_handle = Arc::clone(&model_updated);

    let control_panel_thread = thread::spawn(move || {
        use relm::Widget;
        face_viewer::control_panel::ControlPanel::run((
            color_control_panel_thread_handle,
            expression_control_panel_thread_handle,
            shape_control_panel_thread_handle,
            is_running_control_panel_thread_handle,
            model_updated_control_panel_thread_handle,
        ))
        .unwrap();
    });

    let color_morphable_model_thread_handle = Arc::clone(&color);
    let expression_morphable_model_thread_handle = Arc::clone(&expression);
    let shape_morphable_model_thread_handle = Arc::clone(&shape);
    let is_running_morphable_model_thread_handle = Arc::clone(&is_running);
    let model_updated_morphable_model_thread_handle = Arc::clone(&model_updated);
    view_3d::create_and_run_window(
        model,
        color_morphable_model_thread_handle,
        expression_morphable_model_thread_handle,
        shape_morphable_model_thread_handle,
        is_running_morphable_model_thread_handle,
        model_updated_morphable_model_thread_handle,
    );

    control_panel_thread.join().unwrap();
}
