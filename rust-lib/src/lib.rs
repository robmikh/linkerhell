extern crate winit;

#[no_mangle]
pub unsafe extern "C" fn rust_lib_main() {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::Window::new(&events_loop).unwrap();

    events_loop.run_forever(|event| {
        match event {
                winit::Event::WindowEvent {
                event: winit::WindowEvent::CloseRequested,
                ..
            } => winit::ControlFlow::Break,
            winit::Event::WindowEvent {
                event: winit::WindowEvent::Resized(_),
                ..
            } => {
                let dpi_factor = window.get_hidpi_factor();
                let size = window.get_inner_size().unwrap().to_physical(dpi_factor);

                println!("Window resized! {} x {}", size.width as f32, size.height as f32);

                winit::ControlFlow::Continue
            },
            _ => winit::ControlFlow::Continue,
        }
    });
}