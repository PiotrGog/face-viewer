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

        let vertices_arr = model.shape.representer_points.t();

        let mut vertices: [view_3d::vertex::Vertex; 47439] = [Vertex {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0],
        }; 47439];

        fn calculate_normals(
            indices: &ArrayView2<i32>,
            positions: &ArrayView2<f32>,
        ) -> Vec<Vec<f32>> {
            let mut result = Vec::<Vec<f32>>::new();

            for (i, e) in indices.outer_iter().enumerate() {
                let p1 = positions.row(e[0] as usize);
                let p2 = positions.row(e[1] as usize);
                let p3 = positions.row(e[2] as usize);
                let u = &p2 - &p1;
                let v = &p3 - &p1;

                result.push(vec![
                    u[1] * v[2] - u[2] * v[1],
                    u[2] * v[0] - u[0] * v[2],
                    u[0] * v[1] - u[1] * v[0],
                ]);
            }

            result
        }

        let color = &model.color.mean;
        let expression = &model.expression.mean;
        for (i, e) in vertices_arr.outer_iter().enumerate() {
            vertices[i].position[0] = e.get(0).unwrap().to_owned(); // + expression.get(i * 3).unwrap().to_owned();
            vertices[i].position[1] = e.get(1).unwrap().to_owned(); // + expression.get(i * 3 + 1).unwrap().to_owned();
            vertices[i].position[2] = e.get(2).unwrap().to_owned(); // + expression.get(i * 3 + 2).unwrap().to_owned();

            vertices[i].color[0] = color.get(i * 3).unwrap().to_owned();
            vertices[i].color[1] = color.get(i * 3 + 1).unwrap().to_owned();
            vertices[i].color[2] = color.get(i * 3 + 2).unwrap().to_owned();
        }

        let indices_arr = model.shape.representer_cells.t();

        let mut indices: [u32; 3 * 94464] = [0; 3 * 94464];

        for (i, e) in indices_arr.outer_iter().enumerate() {
            indices[3 * i] = e.get(0).unwrap().to_owned() as u32;
            indices[3 * i + 1] = e.get(1).unwrap().to_owned() as u32;
            indices[3 * i + 2] = e.get(2).unwrap().to_owned() as u32;
        }

        let normals = calculate_normals(
            &model.shape.representer_cells.t(),
            &model.shape.representer_points.t(),
        );

        let mut normal: [view_3d::normal::Normal; 94464] = [view_3d::normal::Normal {
            normal: [0.0, 0.0, 0.0],
        }; 94464];

        for (i, v) in normals.into_iter().enumerate() {
            normal[i] = view_3d::normal::Normal {
                normal: [v[0], v[1], v[2]],
            };
            println!("{:?}", normal[i].normal);
        }

        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new();
        let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

        let positions = glium::VertexBuffer::new(&display, &vertices).unwrap();
        // let normals = glium::VertexBuffer::new(&display, &normal).unwrap();
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
                .draw(
                    &positions, // (&positions, &normals),
                    &indices, &program, &uniforms, &params,
                )
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
