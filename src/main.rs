use face_viewer::{basel_face_model, view_3d::create_and_run_window};

mod test {
    use face_viewer::{
        basel_face_model,
        view_3d::{
            self,
            shaders::three_dim::{FRAGMENT_SHADER_SRC, VERTEX_SHADER_SRC},
            vertex::Vertex,
        },
    };
    use glium::{glutin, uniform, Surface};
    use ndarray::{Array2, ArrayView, ArrayView2};
    pub fn run() {
        let model =
            basel_face_model::hdf5::load_from_file("resources/basel_face_model/model2019_bfm.h5")
                .expect("aaaa");

        let shape_arr = model.calculate_shape().into_shape((142317 / 3, 3)).unwrap();
        let expression_arr = model
            .calculate_expression()
            .into_shape((142317 / 3, 3))
            .unwrap();
        let color_arr = model.calculate_color().into_shape((142317 / 3, 3)).unwrap();
        let mut shape_color = Vec::new();

        for i in 0..(142317 / 3) {
            shape_color.push(Vertex {
                position: [
                    shape_arr[(i, 0)] + expression_arr[(i, 0)],
                    shape_arr[(i, 1)] + expression_arr[(i, 1)],
                    shape_arr[(i, 2)] + expression_arr[(i, 2)],
                ],
                color: [color_arr[(i, 0)], color_arr[(i, 1)], color_arr[(i, 2)]],
            })
        }

        let mut indices = Vec::new();
        let indices_arr = model.shape.representer_cells.t();
        for triangle_points_indexes in indices_arr.outer_iter() {
            indices.push(triangle_points_indexes[0]);
            indices.push(triangle_points_indexes[1]);
            indices.push(triangle_points_indexes[2]);
        }

        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new();
        let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

        let positions = glium::VertexBuffer::new(&display, &shape_color).unwrap();

        let indices = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();

        let program =
            glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None)
                .unwrap();

        let mut t = 0.0 as f32;
        event_loop.run(move |ev, _, control_flow| {
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                _ => (),
            }

            let mut target = display.draw();
            target.clear_color_and_depth((0.8, 0.8, 0.8, 1.0), 1.0);

            t += 0.002;
            if t > 2.0 * 3.14 {
                t = 0.0;
            }
            let rotation = [
                [t.cos(), 0.0, t.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-t.sin(), 0.0, t.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ];
            let scale = [
                [0.005, 0.0, 0.0, 0.0],
                [0.0, 0.005, 0.0, 0.0],
                [0.0, 0.0, 0.005, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ];

            let uniforms = uniform! {
                scale: scale,
                rotation: rotation,
            };

            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };
            target
                .draw(&positions, &indices, &program, &uniforms, &params)
                .unwrap();

            target.finish().unwrap();
        });
    }
}

fn main() -> hdf5::Result<()> {
    test::run();
    // create_and_run_window();
    // let model =
    //     basel_face_model::hdf5::load_from_file("resources/basel_face_model/model2019_bfm.h5")?;
    Ok(())
}
