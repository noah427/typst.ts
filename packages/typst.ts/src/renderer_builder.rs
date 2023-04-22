use crate::{browser_world::TypstBrowserWorld, renderer::TypstRenderer, web_font::WebFont};

use super::browser_world::BrowserFontSearcher;
use js_sys::Uint8Array;
use typst::util::Buffer;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct TypstRendererBuilder {
    searcher: BrowserFontSearcher,
}

#[wasm_bindgen]
impl TypstRendererBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<TypstRendererBuilder, JsValue> {
        Ok(Self {
            searcher: BrowserFontSearcher::new(),
        })
    }

    pub async fn add_raw_font(&mut self, font_buffer: Uint8Array) -> Result<(), JsValue> {
        // let v: JsValue =
        //     format!("raw font loading: Buffer({:?})", font_buffer.byte_length()).into();
        // console::info_1(&v);

        self.add_raw_font_internal(font_buffer.to_vec().into());
        Ok(())
    }

    pub async fn add_web_font(&mut self, font: WebFont) -> Result<(), JsValue> {
        let v: JsValue = format!("web font loading: {:?}", font).into();
        console::info_1(&v);

        self.searcher.add_web_font(font).await;

        Ok(())
    }

    pub async fn build(self) -> Result<TypstRenderer, JsValue> {
        Ok(TypstRenderer::new(
            TypstBrowserWorld::new(self.searcher).await?,
        ))
    }
}

impl TypstRendererBuilder {
    pub fn add_raw_font_internal(&mut self, font_buffer: Buffer) {
        self.searcher.add_font_data(font_buffer);
    }
}
