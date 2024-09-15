use anyhow::Result;
use fft_vis_app::app::FFTVisApp;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<()> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = FFTVisApp::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
