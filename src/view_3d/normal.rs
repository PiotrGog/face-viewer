#[derive(Clone, Copy)]
pub struct Normal {
    pub normal: [f32; 3],
}

implement_vertex!(Normal, normal);
