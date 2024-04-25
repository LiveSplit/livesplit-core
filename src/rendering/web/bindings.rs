// Taken from web-sys, but optimized to take JsStrings.

use js_sys::JsString;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = CanvasRenderingContext2D, typescript_type = "CanvasRenderingContext2D")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type CanvasRenderingContext2d;
    #[wasm_bindgen(structural, method, setter, js_class = "CanvasRenderingContext2D", js_name = strokeStyle)]
    pub fn set_stroke_style(this: &CanvasRenderingContext2d, value: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(structural, method, setter, js_class = "CanvasRenderingContext2D", js_name = fillStyle)]
    pub fn set_fill_style(this: &CanvasRenderingContext2d, value: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(structural, method, setter, js_class = "CanvasRenderingContext2D", js_name = filter)]
    pub fn set_filter(this: &CanvasRenderingContext2d, value: &JsString);
    #[wasm_bindgen(structural, method, setter, js_class = "CanvasRenderingContext2D", js_name = lineWidth)]
    pub fn set_line_width(this: &CanvasRenderingContext2d, value: f64);
    #[wasm_bindgen(structural, method, setter, js_class = "CanvasRenderingContext2D", js_name = font)]
    pub fn set_font(this: &CanvasRenderingContext2d, value: &js_sys::JsString);
    #[wasm_bindgen(structural, method, setter, js_class = "CanvasRenderingContext2D", js_name = fontKerning)]
    pub fn set_font_kerning(this: &CanvasRenderingContext2d, value: &JsString);
    #[wasm_bindgen(catch, method, structural, js_class = "CanvasRenderingContext2D", js_name = drawImage)]
    pub fn draw_image_with_image_bitmap_and_dw_and_dh(
        this: &CanvasRenderingContext2d,
        image: &web_sys::ImageBitmap,
        dx: f64,
        dy: f64,
        dw: f64,
        dh: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, structural, js_class = "CanvasRenderingContext2D", js_name = fill)]
    pub fn fill_with_path_2d(this: &CanvasRenderingContext2d, path: &web_sys::Path2d);
    #[wasm_bindgen(method, structural, js_class = "CanvasRenderingContext2D", js_name = stroke)]
    pub fn stroke_with_path(this: &CanvasRenderingContext2d, path: &web_sys::Path2d);
    #[wasm_bindgen(method, structural, js_class = "CanvasRenderingContext2D", js_name = createLinearGradient)]
    pub fn create_linear_gradient(
        this: &CanvasRenderingContext2d,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> web_sys::CanvasGradient;
    #[wasm_bindgen(method, structural, js_class = "CanvasRenderingContext2D", js_name = clearRect)]
    pub fn clear_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(method, structural, js_class = "CanvasRenderingContext2D", js_name = fillRect)]
    pub fn fill_rect(this: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64);
    #[wasm_bindgen(catch, method, structural, js_class = "CanvasRenderingContext2D", js_name = fillText)]
    pub fn fill_text(
        this: &CanvasRenderingContext2d,
        text: &JsString,
        x: f64,
        y: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, structural, js_class = "CanvasRenderingContext2D", js_name = measureText)]
    pub fn measure_text(
        this: &CanvasRenderingContext2d,
        text: &JsString,
    ) -> Result<web_sys::TextMetrics, JsValue>;
    #[wasm_bindgen(catch, method, structural, js_class = "CanvasRenderingContext2D", js_name = resetTransform)]
    pub fn reset_transform(this: &CanvasRenderingContext2d) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, structural, js_class = "CanvasRenderingContext2D", js_name = setTransform)]
    pub fn set_transform(
        this: &CanvasRenderingContext2d,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
    ) -> Result<(), JsValue>;
}
