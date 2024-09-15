use anyhow::{Context, Result};
use wgpu::{Device, Instance, PowerPreference, Queue, RequestAdapterOptions, SurfaceTarget};
struct Renderer {
    instance: Instance,
    device: Device,
    queue: Queue,
}

impl Renderer {
    pub async fn init<'window>(target: impl Into<SurfaceTarget<'window>>) -> Result<Self> {
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
        Ok(Self {
            instance,
            device,
            queue,
        })
    }
}
