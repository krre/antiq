use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Gpu, GpuCanvasConfiguration, GpuCanvasContext, GpuColorDict, GpuDevice, GpuLoadOp,
    GpuRenderPassColorAttachment, GpuRenderPassDescriptor, GpuStoreOp, GpuTextureFormat, js_sys,
};

use crate::ui::{Color, Ui3d, d2::geometry::Size2D};

pub struct Renderer {
    gpu: Gpu,
    context: GpuCanvasContext,
    device: GpuDevice,
}

impl Renderer {
    pub(crate) async fn new(gpu: Gpu, context: GpuCanvasContext) -> Result<Self, JsValue> {
        let adapter = JsFuture::from(gpu.request_adapter())
            .await?
            .dyn_into::<web_sys::GpuAdapter>()?;
        let device = JsFuture::from(adapter.request_device())
            .await?
            .dyn_into::<web_sys::GpuDevice>()?;

        let configuration = GpuCanvasConfiguration::new(&device, GpuTextureFormat::Bgra8unorm);
        context.configure(&configuration)?;

        Ok(Self {
            gpu,
            context,
            device,
        })
    }

    pub fn resize(&self, _size: &Size2D) {}

    pub fn render(&self, ui: &Ui3d) {
        self.clear(ui.background_color());
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }

    fn clear(&self, color: &Color) {
        let texture = self
            .context
            .get_current_texture()
            .expect_throw("Failed to get current texture");
        let view = texture
            .create_view()
            .expect_throw("Failed to create texture view");
        let color_attachment =
            GpuRenderPassColorAttachment::new(GpuLoadOp::Clear, GpuStoreOp::Store, &view);

        let color = GpuColorDict::new(
            1.0,
            color.blue().into(),
            color.green().into(),
            color.red().into(),
        );
        color_attachment.set_clear_value(&color);

        let color_attachments = js_sys::Array::new();
        color_attachments.push(&color_attachment);

        let render_pass_descriptor = GpuRenderPassDescriptor::new(&color_attachments);

        let encoder = self.device.create_command_encoder();

        let render_pass = encoder
            .begin_render_pass(&render_pass_descriptor)
            .expect_throw("Failed to begin render pass");
        render_pass.end();

        let command_buffer = encoder.finish();

        let command_buffers = js_sys::Array::new();
        command_buffers.push(&command_buffer);

        let queue = self.device.queue();
        queue.submit(&command_buffers);
    }
}
