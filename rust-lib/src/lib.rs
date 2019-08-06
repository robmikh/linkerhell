extern crate winit;
extern crate wgpu;

#[no_mangle]
pub unsafe extern "C" fn rust_lib_main() {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::Window::new(&events_loop).unwrap();

    let instance = wgpu::Instance::new();
    let adapter = instance.get_adapter(&wgpu::AdapterDescriptor{
        power_preference: wgpu::PowerPreference::LowPower,
    });
    let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    let surface = instance.create_surface(&window);
    let mut dpi_factor = window.get_hidpi_factor();
    let mut size = window.get_inner_size().unwrap().to_physical(dpi_factor);
    let mut swap_chain_description = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Vsync,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_description);

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