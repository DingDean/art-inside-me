use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use futures::executor::block_on;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Vertex {
    pos: [f32; 3],
}

struct Art {
    vertices: Vec<Vertex>,
}

impl Art {
    fn lines() -> Self {
        let vertices = vec![
            Vertex {
                pos: [-1.0, 1.0, 0.0],
            },
            Vertex {
                pos: [0.0, 0.0, 0.0],
            },
        ];
        Self { vertices }
    }
}

struct App {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc: wgpu::SwapChain,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

// 我想渲染一段直线
// 我需要指定 vertex 以及它们的顺序
// 在 wgpu 中如何指定
impl App {
    async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let sc_format = adapter.get_swap_chain_preferred_format(&surface);
        let sc = device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
                width: size.width,
                height: size.height,
                format: sc_format,
                present_mode: wgpu::PresentMode::Fifo,
            },
        );

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "shader.wgsl"
            ))),
        });
        let art = Art::lines();
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            usage: wgpu::BufferUsage::VERTEX,
            contents: &bytemuck::cast_slice(&art.vertices),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                buffers: &[wgpu::VertexBufferLayout {
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float3],
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                }],
                entry_point: "vs_main",
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[sc_format.into()],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        });

        Ok(Self {
            instance,
            surface,
            device,
            queue,
            sc,
            pipeline,
            vertex_buffer,
        })
    }

    pub fn render(&self) -> Result<()> {
        let frame = self.sc.get_current_frame().expect("无法获取下一帧").output;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: true,
                    },
                    resolve_target: None,
                    attachment: &frame.view,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.draw(0..2, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        Ok(())
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let app = block_on(App::new(&window)).unwrap();

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                }
                | KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Q),
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                app.render().unwrap();
            }
            _ => (),
        }
    });
}
