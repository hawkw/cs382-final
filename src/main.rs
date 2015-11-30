#[macro_use] extern crate glium;

use glium::Surface;
use glium::index;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

    // vertex shader source
    let vertex_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    // fragment shader source
    let frag_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    loop {
        // Iterate through the events that the window recieved this frame
        for event in display.poll_events() {
            match event {
                // if the window recieved the `Closed` event, break
                glium::glutin::Event::Closed => return
                // don't handle any other events (yet)
              , _ => {}
            }
        };

        let mut frame = display.draw();

        let shape = vec![ Vertex { position: [-0.5, -0.5] }
                        , Vertex { position: [-0.5,  0.5] }
                        , Vertex { position: [0.5,  -0.5]}
                        ];

        let vertex_buffer
            = glium::VertexBuffer::new(&display, &shape)
                .expect("Could not create vertex buffer!");

        let indices
            = index::NoIndices(index::PrimitiveType::TrianglesList);

        let program
            = glium::Program::from_source( &display
                                         , vertex_src
                                         , frag_src
                                         , None)
                .expect("Could not compile shader program!");

        frame.clear_color(0.0, 0.0, 0.0, 1.0); // clear the display
        frame.draw( &vertex_buffer
                  , &indices
                  , &program
                  , &glium::uniforms::EmptyUniforms
                  , &Default::default()
                  ).expect("Could not draw shaders!");
        frame.finish() // draw this frame, panic if the frame was not drawn
             .expect("Could not finish frame!");
    }
}
