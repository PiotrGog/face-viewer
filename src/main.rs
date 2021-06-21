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
    pub fn run() {
        let model =
            basel_face_model::hdf5::load_from_file("resources/basel_face_model/model2019_bfm.h5")
                .expect("aaaa");

        let vertices_arr = model.shape.representer_points.t();

        println!("{:?}", vertices_arr);
        println!("{:?}", model.calculate_shape());

        let mut vertices: [view_3d::vertex::Vertex; 47439] = [Vertex {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 1.0, 0.0],
        }; 47439];

        let color = &model.color.mean;
        let expression = &model.expression.mean;
        for (i, e) in vertices_arr.outer_iter().enumerate() {
            vertices[i].position[0] =
                e.get(0).unwrap().to_owned() + expression.get(i * 3).unwrap().to_owned();
            vertices[i].position[1] =
                e.get(1).unwrap().to_owned() + expression.get(i * 3 + 1).unwrap().to_owned();
            vertices[i].position[2] =
                e.get(2).unwrap().to_owned() + expression.get(i * 3 + 2).unwrap().to_owned();

            vertices[i].color[0] = color.get(i * 3).unwrap().to_owned();
            vertices[i].color[1] = color.get(i * 3 + 1).unwrap().to_owned();
            vertices[i].color[2] = color.get(i * 3 + 2).unwrap().to_owned();
            println!("{:?}", vertices[i].color);
        }

        let indices_arr = model.shape.representer_cells.t();

        let mut indices: [u32; 3 * 94464] = [0; 3 * 94464];

        for (i, e) in indices_arr.outer_iter().enumerate() {
            indices[3 * i] = e.get(0).unwrap().to_owned() as u32;
            indices[3 * i + 1] = e.get(1).unwrap().to_owned() as u32;
            indices[3 * i + 2] = e.get(2).unwrap().to_owned() as u32;
        }

        println!("{},{},{}", indices[0], indices[1], indices[2]);
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new();
        let context_builder = glutin::ContextBuilder::new();
        let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

        let positions = glium::VertexBuffer::new(&display, &vertices).unwrap();

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
            target.clear_color(0.8, 0.8, 0.8, 1.0);

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
            target
                .draw(
                    &positions,
                    &indices,
                    &program,
                    &uniforms,
                    &Default::default(),
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
