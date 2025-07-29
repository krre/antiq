pub struct CanvasContext {
    _inner: web_sys::GpuCanvasContext,
}

impl CanvasContext {
    pub(crate) fn new(gpu_canvas_context: web_sys::GpuCanvasContext) -> Self {
        Self {
            _inner: gpu_canvas_context,
        }
    }
}
