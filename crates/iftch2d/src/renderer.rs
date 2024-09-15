use std::borrow::Cow;

use anyhow::{Context, Result};
use wgpu::{
    Adapter, CommandEncoder, Device, Instance, PowerPreference, Queue, RenderPass,
    RenderPassDescriptor, RenderPipeline, RequestAdapterOptions, Surface, SurfaceConfiguration,
    SurfaceTarget, TextureView,
};
struct Renderer<'window> {
    instance: Instance,
    surface: Surface<'window>,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
    config: SurfaceConfiguration,
}

struct RenderCommandEncoder<'encoder> {
    encoder: wgpu::CommandEncoder,
    active_rpass: Option<RenderPass<'encoder>>,
}

impl RenderCommandEncoder<'_> {
    fn create(mut encoder: CommandEncoder, view: &TextureView) -> Self {
        let rpass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        Self {
            encoder,
            active_rpass: Some(rpass),
        }
    }

    pub fn draw_triangle(&mut self) {}
}

impl<'window> Renderer<'window> {
    pub async fn init(
        target: impl Into<SurfaceTarget<'window>>,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(target).unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .with_context(|| "failed to create adapter from given surface")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
            .with_context(|| "Failed to create device")?;
        let pipeline = Self::create_pipeline(&device, &adapter, &surface);

        let config = surface
            .get_default_config(&adapter, width, height)
            .with_context(|| "surface config failed with width: {width} and heigth: {height}")?;
        surface.configure(&device, &config);

        Ok(Self {
            instance,
            surface,
            device,
            queue,
            pipeline,
            config,
        })
    }

    pub fn update_size(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    #[inline]
    fn create_pipeline(device: &Device, adapter: &Adapter, surface: &Surface) -> RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../shaders/default.wgsl"
            ))),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("default pipeline"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }
}
