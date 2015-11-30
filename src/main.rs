extern crate glium;

use glium::Surface;

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

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

        frame.clear_color(0.0, 0.0, 0.0, 1.0); // clear the display
            // insert shader here
        frame.finish()
             .expect("Could not draw frame!");
    }
}
