use wasm_bindgen::JsValue;
use web_sys::GpuCanvasConfiguration;

pub struct CanvasContext {
    inner: web_sys::GpuCanvasContext,
}

impl CanvasContext {
    pub(crate) fn new(gpu_canvas_context: web_sys::GpuCanvasContext) -> Self {
        Self {
            inner: gpu_canvas_context,
        }
    }

    pub fn configure(&self, configuration: GpuCanvasConfiguration) -> Result<(), JsValue> {
        self.inner.configure(&configuration)?;
        Ok(())
    }
}
