pub mod normal;
pub mod shaders;
pub mod vertex;

use crate::basel_face_model;

use glium::{glutin, uniform, Surface};

use std::sync::{Arc, Mutex};

pub fn create_and_run_window(
    model: basel_face_model::hdf5::morphable_model::MorphableModel,
    color: Arc<Mutex<ndarray::Array1<f32>>>,
    expression: Arc<Mutex<ndarray::Array1<f32>>>,
    shape: Arc<Mutex<ndarray::Array1<f32>>>,
) {
    let rotation = [
        [-1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, -1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];
    let scale = [
        [0.005, 0.0, 0.0, 0.0],
        [0.0, 0.005, 0.0, 0.0],
        [0.0, 0.0, 0.005, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

    let color_parameters_size = model.color.mean.shape()[0];
    let expression_parameters_size = model.expression.mean.shape()[0];
    let shape_parameters_size = model.shape.mean.shape()[0];

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("Model View Window");
    let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
    let program = glium::Program::from_source(
        &display,
        shaders::three_dim::VERTEX_SHADER_SRC,
        shaders::three_dim::FRAGMENT_SHADER_SRC,
        None,
    )
    .unwrap();

    let indices = get_indices(&model);
    let indices_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    )
    .unwrap();

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

    let mut shape_and_color = create_shape_and_color_zeroed_vector(shape_parameters_size);
    let mut shape_and_color_buffer =
        glium::VertexBuffer::dynamic(&display, &shape_and_color).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    println!("glutin::event::WindowEvent::CloseRequested");
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => {
                    return;
                }
            },
            _ => (),
        }

        update_shape_and_color_buffer(
            &mut shape_and_color_buffer,
            &model,
            Arc::clone(&color),
            Arc::clone(&expression),
            Arc::clone(&shape),
            color_parameters_size,
            expression_parameters_size,
            shape_parameters_size,
            &mut shape_and_color,
        );

        draw(
            &display,
            &shape_and_color_buffer,
            &indices_buffer,
            &program,
            &uniforms,
            &params,
        );
    });
}

fn create_shape_and_color_zeroed_vector(shape_parameters_size: usize) -> Vec<vertex::Vertex> {
    let mut shape_and_color = Vec::with_capacity(shape_parameters_size / 3);
    shape_and_color.resize(
        shape_parameters_size / 3,
        vertex::Vertex {
            position: [0.0; 3],
            color: [0.0; 3],
        },
    );

    shape_and_color
}

fn get_indices(model: &basel_face_model::hdf5::morphable_model::MorphableModel) -> Vec<u32> {
    let mut indices = Vec::new();
    let indices_arr = model.shape.representer_cells.t();
    for triangle_points_indexes in indices_arr.outer_iter() {
        indices.push(triangle_points_indexes[0]);
        indices.push(triangle_points_indexes[1]);
        indices.push(triangle_points_indexes[2]);
    }

    indices
}

fn draw<UniformType>(
    display: &glium::Display,
    shape_and_color_buffer: &glium::VertexBuffer<vertex::Vertex>,
    indices_buffer: &glium::IndexBuffer<u32>,
    program: &glium::Program,
    uniforms: &UniformType,
    params: &glium::DrawParameters,
) where
    UniformType: glium::uniforms::Uniforms,
{
    let mut target = display.draw();
    target.clear_color_and_depth((0.8, 0.8, 0.8, 1.0), 1.0);

    target
        .draw(
            shape_and_color_buffer,
            indices_buffer,
            &program,
            uniforms,
            &params,
        )
        .unwrap();

    target.finish().unwrap();
}

fn update_shape_and_color_buffer(
    shape_and_color_buffer: &mut glium::VertexBuffer<vertex::Vertex>,
    model: &basel_face_model::hdf5::morphable_model::MorphableModel,
    color: Arc<Mutex<ndarray::Array1<f32>>>,
    expression: Arc<Mutex<ndarray::Array1<f32>>>,
    shape: Arc<Mutex<ndarray::Array1<f32>>>,
    color_parameters_size: usize,
    expression_parameters_size: usize,
    shape_parameters_size: usize,
    shape_and_color: &mut Vec<vertex::Vertex>,
) {
    let color_calc = model.calculate_color(color);
    let color_arr = color_calc
        .view()
        .into_shape((color_parameters_size / 3, 3))
        .unwrap();
    let expression_calc = model.calculate_expression(expression);
    let expression_arr = expression_calc
        .view()
        .into_shape((expression_parameters_size / 3, 3))
        .unwrap();
    let shape_calc = model.calculate_shape(shape);
    let shape_arr = shape_calc
        .view()
        .into_shape((shape_parameters_size / 3, 3))
        .unwrap();

    let combined_shape_expression = &shape_arr + &expression_arr;

    for (i, (s, c)) in combined_shape_expression
        .outer_iter()
        .zip(color_arr.outer_iter())
        .enumerate()
    {
        shape_and_color[i] = vertex::Vertex {
            position: [s[0], s[1], s[2]],
            color: [c[0], c[1], c[2]],
        }
    }
    shape_and_color_buffer.write(&shape_and_color);
}
