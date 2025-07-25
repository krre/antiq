pub struct Canvas {
    _inner: web_sys::HtmlCanvasElement,
}

impl Canvas {
    pub(crate) fn new(inner: web_sys::HtmlCanvasElement) -> Self {
        Self { _inner: inner }
    }
}
