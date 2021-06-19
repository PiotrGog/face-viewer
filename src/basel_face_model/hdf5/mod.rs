mod model_params;
mod morphable_model;

use model_params::ModelParams;

pub fn load_from_file(file: &str) -> hdf5::Result<morphable_model::MorphableModel> {
    Ok(morphable_model::MorphableModel::load_from_file(file)?)
}

mod helper_test_tmp {
    use ndarray::{Array1, Array2};
    use opencv::{core, highgui, imgproc};

    #[allow(unused)]
    fn show_dots2(result: &Array2<f32>) {
        let result = result + 150.0;
        println!("{:?}", &result);

        let mut vector_of_points2i: core::Vector<core::Point2i> = core::Vector::new();
        for e in result.t().outer_iter() {
            vector_of_points2i.push(core::Point2i::new(
                e.get(0).unwrap().to_owned() as i32,
                e.get(1).unwrap().to_owned() as i32,
            ));
        }

        highgui::named_window("wiiiiiiii", 1).unwrap();
        let mut frame =
            core::Mat::new_rows_cols_with_default(400, 400, core::CV_8U, core::Scalar::all(0.0))
                .expect("msg");
        let radius = 1;
        let color = core::Scalar::all(255.0);
        let thickness = 3;
        let line_type = 1;
        let shift = 0;

        for point in vector_of_points2i {
            imgproc::circle(
                &mut frame, point, radius, color, thickness, line_type, shift,
            )
            .unwrap_or_else(|error| eprintln!("{}", error));
        }
        highgui::imshow("wiiiiiiii", &frame).unwrap_or_else(|error| eprintln!("{}", error));

        highgui::wait_key(0).unwrap();
    }

    #[allow(unused)]
    fn show_dots(result: &Array1<f32>) {
        let mut max: f32 = -100000.;
        let mut min: f32 = 100000.;
        for val in result.iter() {
            if val < &min {
                min = *val;
            }
            if val > &max {
                max = *val;
            }
        }

        println!("max: {}, min: {}", max, min);
        max += min;
        let result = result + min.abs();
        let shape = result.shape()[0];
        let result = result / 10.0;
        let result = result.into_shape((shape / 3, 3)).expect("msg");
        println!("{:?}", &result);

        let mut vector_of_points2i: core::Vector<core::Point2i> = core::Vector::new();
        for e in result.outer_iter() {
            vector_of_points2i.push(core::Point2i::new(
                e.get(0).unwrap().to_owned() as i32,
                e.get(1).unwrap().to_owned() as i32,
            ));
        }

        highgui::named_window("wiiiiiiii", 1).unwrap();
        let mut frame =
            core::Mat::new_rows_cols_with_default(1500, 1500, core::CV_8U, core::Scalar::all(0.0))
                .expect("msg");
        let radius = 1;
        let color = core::Scalar::all(255.0);
        let thickness = 3;
        let line_type = 1;
        let shift = 0;

        for point in vector_of_points2i {
            imgproc::circle(
                &mut frame, point, radius, color, thickness, line_type, shift,
            )
            .unwrap_or_else(|error| eprintln!("{}", error));
        }
        highgui::imshow("wiiiiiiii", &frame).unwrap_or_else(|error| eprintln!("{}", error));

        highgui::wait_key(0).unwrap();
    }
}
