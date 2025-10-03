use wasm_bindgen::JsValue;
use web_sys::{GpuCanvasConfiguration, GpuCanvasContext};

pub struct CanvasContext {
    inner: web_sys::GpuCanvasContext,
}

impl CanvasContext {
    pub(crate) fn new(canvas_context: GpuCanvasContext) -> Self {
        Self {
            inner: canvas_context,
        }
    }

    pub fn configure(&self, configuration: GpuCanvasConfiguration) -> Result<(), JsValue> {
        self.inner.configure(&configuration)?;
        Ok(())
    }
}
