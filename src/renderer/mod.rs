use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::{
    GpuCanvasConfiguration, GpuColorDict, GpuLoadOp, GpuRenderPassColorAttachment,
    GpuRenderPassDescriptor, GpuStoreOp, GpuTextureFormat, js_sys,
};

use crate::{
    renderer::webgpu::{Adapter, CanvasContext, Device, Gpu},
    ui::{Ui3d, d2::geometry::Size2D},
};

pub mod webgpu;

pub struct Renderer {
    gpu: Gpu,
    context: CanvasContext,
    _adapter: Adapter,
    device: Device,
}

impl Renderer {
    pub(crate) async fn new(gpu: Gpu, context: CanvasContext) -> Result<Self, JsValue> {
        let adapter = gpu.request_adapter().await?;
        let device = adapter.request_device().await?;

        let configuration =
            GpuCanvasConfiguration::new(device.into_inner(), GpuTextureFormat::Bgra8unorm);
        context.configure(configuration)?;

        Ok(Self {
            gpu,
            context,
            _adapter: adapter,
            device,
        })
    }

    pub fn resize(&self, _size: &Size2D) {
        // gloo::console::log!("resize renderer", size.width(), size.height())
    }

    pub fn render(&self, ui: &Ui3d) {
        let texture = self
            .context
            .into_inner()
            .get_current_texture()
            .expect_throw("Failed to get current texture");
        let view = texture
            .create_view()
            .expect_throw("Failed to create texture view");
        let color_attachment =
            GpuRenderPassColorAttachment::new(GpuLoadOp::Clear, GpuStoreOp::Store, &view);

        let color = GpuColorDict::new(
            1.0,
            ui.background_color().blue(),
            ui.background_color().green(),
            ui.background_color().red(),
        );
        color_attachment.set_clear_value(&color);

        let color_attachments = js_sys::Array::new();
        color_attachments.push(&color_attachment);

        let render_pass_descriptor = GpuRenderPassDescriptor::new(&color_attachments);

        let encoder = self.device.into_inner().create_command_encoder();

        let render_pass = encoder
            .begin_render_pass(&render_pass_descriptor)
            .expect_throw("Failed to begin render pass");
        render_pass.end();

        let command_buffer = encoder.finish();

        let command_buffers = js_sys::Array::new();
        command_buffers.push(&command_buffer);

        let queue = self.device.into_inner().queue();
        queue.submit(&command_buffers);
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
