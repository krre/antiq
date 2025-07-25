pub struct Canvas {
    inner: web_sys::HtmlCanvasElement,
}

impl Canvas {
    pub(crate) fn new(inner: web_sys::HtmlCanvasElement) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &web_sys::HtmlCanvasElement {
        &self.inner
    }
}
