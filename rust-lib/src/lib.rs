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

    let mut running = true;
    while running {
        let mut new_size: Option<winit::dpi::LogicalSize> = None;
        events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => running = false,
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::Resized(size),
                    ..
                } => new_size = Some(size),
                _ => {},
            }
        });

        if !running {
            break;
        }

        if let Some(size) = new_size {
            let dpi_factor = window.get_hidpi_factor();
            let size = size.to_physical(dpi_factor);

            println!("Window resized! {} x {}", size.width as f32, size.height as f32);

            let mut dpi_factor = window.get_hidpi_factor();
            let mut size = window.get_inner_size().unwrap().to_physical(dpi_factor);
            let mut swap_chain_description = wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8Unorm,
                width: size.width as u32,
                height: size.height as u32,
                present_mode: wgpu::PresentMode::Vsync,
            };
            swap_chain = device.create_swap_chain(&surface, &swap_chain_description);
        }

        let frame = swap_chain.get_next_texture();
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::GREEN,
                }],
                depth_stencil_attachment: None,
            });
        }

        device.get_queue().submit(&[encoder.finish()]);
    }
}