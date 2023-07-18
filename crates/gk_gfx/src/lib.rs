use gk_app::{AppBuilder, BuildConfig, GKState, Plugin};
use gk_platform::{GKWindow, GKWindowId, Platform, WindowEvent, WindowEventId};
use hashbrown::HashMap;
use wgpu::{Adapter, Color, Device, Instance, Queue, Surface, SurfaceTexture};

pub struct Gfx {
    color: Color,
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    surfaces: HashMap<GKWindowId, Surface>,
}

impl Plugin for Gfx {}

impl Gfx {
    pub fn new() -> Result<Self, String> {
        pollster::block_on(Self::init())
    }

    async fn init() -> Result<Self, String> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                // Request an adapter which can render to our surface
                // compatible_surface: viewports.first().map(|desc| &desc.surface),
                ..Default::default()
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        Ok(Self {
            color: Color::WHITE,
            instance,
            adapter,
            device,
            queue,
            surfaces: Default::default(),
        })
    }

    pub fn create_surface<W: GKWindow>(&mut self, window: &W) {
        let surface = unsafe { self.instance.create_surface(window) }.unwrap();
        let caps = surface.get_capabilities(&self.adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: window.width(),
            height: window.height(),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&self.device, &config);
        self.surfaces.insert(window.id(), surface);
    }

    pub fn current_texture(&mut self, id: &GKWindowId) -> SurfaceTexture {
        self.surfaces
            .get(id)
            .unwrap()
            .get_current_texture()
            .unwrap()
    }

    pub fn draw(&mut self, id: &GKWindowId) {
        let frame = self.current_texture(id);
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        self.color.r = (self.color.r + 0.001) % 1.0;
        self.color.g = (self.color.g + 0.01) % 1.0;
        self.color.b = (self.color.b + 0.1) % 1.0;
    }
}

pub struct GfxConfig;

impl<S: GKState + 'static> BuildConfig<S> for GfxConfig {
    fn apply(&mut self, builder: AppBuilder<S>) -> Result<AppBuilder<S>, String> {
        let builder = builder.listen_event(
            |evt: &WindowEvent, gfx: &mut Gfx, platform: &mut Platform| match evt.event {
                WindowEventId::Init => gfx.create_surface(platform.window(evt.id).unwrap()),
                WindowEventId::Moved { .. } => {}
                WindowEventId::Resized { .. } => {}
                WindowEventId::Minimized => {}
                WindowEventId::Maximized => {}
                WindowEventId::FocusGained => {}
                WindowEventId::FocusLost => {}
                WindowEventId::Close => {}
            },
        );

        let gfx = Gfx::new()?;
        Ok(builder.add_plugin(gfx))
    }
}

/*
impl Viewport {
    // fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
    //     self.config.width = size.width;
    //     self.config.height = size.height;
    //     self.desc.surface.configure(device, &self.config);
    // }
    fn get_current_texture(&mut self) -> wgpu::SurfaceTexture {
        self.desc
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture")
    }
}

async fn run(event_loop: EventLoop<()>, viewports: Vec<(Window, wgpu::Color)>) {
    let instance = wgpu::Instance::default();
    let viewports: Vec<_> = viewports
        .into_iter()
        .map(|(window, color)| ViewportDesc::new(window, color, &instance))
        .collect();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            // Request an adapter which can render to our surface
            compatible_surface: viewports.first().map(|desc| &desc.surface),
            ..Default::default()
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let mut viewports: HashMap<WindowId, Viewport> = viewports
        .into_iter()
        .map(|desc| (desc.window.id(), desc.build(&adapter, &device)))
        .collect();

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&instance, &adapter);

        *control_flow = ControlFlow::Wait;
        match event {
            // Event::WindowEvent {
            //     window_id,
            //     event: WindowEvent::Resized(size),
            //     ..
            // } => {
            //     // Recreate the swap chain with the new size
            //     if let Some(viewport) = viewports.get_mut(&window_id) {
            //         viewport.resize(&device, size);
            //         // On macos the window needs to be redrawn manually after resizing
            //         viewport.desc.window.request_redraw();
            //     }
            // }
            Event::RedrawRequested(window_id) => {
                if let Some(viewport) = viewports.get_mut(&window_id) {
                    let frame = viewport.get_current_texture();
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    {
                        let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(viewport.desc.background),
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment: None,
                        });
                    }

                    queue.submit(Some(encoder.finish()));
                    frame.present();
                }
            }
            // Event::WindowEvent {
            //     window_id,
            //     event: WindowEvent::CloseRequested,
            //     ..
            // } => {
            //     viewports.remove(&window_id);
            //     if viewports.is_empty() {
            //         *control_flow = ControlFlow::Exit
            //     }
            // }
            _ => {}
        }
    });
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        const WINDOW_SIZE: u32 = 128;
        const WINDOW_PADDING: u32 = 16;
        const WINDOW_TITLEBAR: u32 = 32;
        const WINDOW_OFFSET: u32 = WINDOW_SIZE + WINDOW_PADDING;
        const ROWS: u32 = 4;
        const COLUMNS: u32 = 4;

        let event_loop = EventLoop::new();
        let mut viewports = Vec::with_capacity((ROWS * COLUMNS) as usize);
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                let window = winit::window::WindowBuilder::new()
                    .with_title(format!("x{column}y{row}"))
                    .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_SIZE, WINDOW_SIZE))
                    .build(&event_loop)
                    .unwrap();
                window.set_outer_position(winit::dpi::PhysicalPosition::new(
                    WINDOW_PADDING + column * WINDOW_OFFSET,
                    WINDOW_PADDING + row * (WINDOW_OFFSET + WINDOW_TITLEBAR),
                ));
                fn frac(index: u32, max: u32) -> f64 {
                    index as f64 / max as f64
                }
                viewports.push((
                    window,
                    wgpu::Color {
                        r: frac(row, ROWS),
                        g: 0.5 - frac(row * column, ROWS * COLUMNS) * 0.5,
                        b: frac(column, COLUMNS),
                        a: 1.0,
                    },
                ))
            }
        }

        env_logger::init();
        pollster::block_on(run(event_loop, viewports));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        panic!("wasm32 is not supported")
    }
}
*/
