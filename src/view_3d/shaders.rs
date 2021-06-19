#[allow(unused)]
pub mod two_dim {
    pub const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 position;
    out vec2 my_attr;
    uniform mat4 matrix;

    void main() {
        my_attr = position;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

    pub const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    in vec2 my_attr;
    out vec4 color;

    void main() {
        color = vec4(my_attr, 0.0, 1.0);
    }
"#;
}

#[allow(unused)]
pub mod three_dim {
    pub const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec3 position;
    uniform mat4 rotation;
    uniform mat4 scale;

    void main() {
        gl_Position = scale * rotation * vec4(position, 1.0);
    }
"#;

    pub const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }
"#;
}
