#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position);
