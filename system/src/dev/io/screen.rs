use js_sys::Map;

use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::{JsCast, JsValue};

use super::super::{DeviceTrait, AddressableDeviceTrait, DeviceId};

const COLOR_PALETTE: [&'static str; 16] = [
    "black",
    "white",
    "silver",
    "gray",
    "red",
    "maroon",
    "yellow",
    "olive",
    "lime",
    "green",
    "aqua",
    "teal",
    "blue",
    "navy",
    "fuchsia",
    "purple",
];

const BACKGROUND_COLOR: &'static str = COLOR_PALETTE[0];

const DEFAULT_WIDTH: u16 = 50;
const DEFAULT_HEIGHT: u16 = 50;

pub struct PixelScreen {
    canvas: HtmlCanvasElement,
    canvas_context: CanvasRenderingContext2d,
}

impl PixelScreen {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let canvas = document.create_element("canvas").unwrap();
        let canvas = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(DEFAULT_WIDTH as u32);
        canvas.set_height(DEFAULT_HEIGHT as u32);

        let canvas_context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let mut tmp = PixelScreen {
            canvas,
            canvas_context,
        };

        tmp.reset_system();

        tmp
    }
}

impl DeviceTrait for PixelScreen {
    fn reset_system(&mut self) {
        self.canvas_context.set_fill_style(&JsValue::from_str(BACKGROUND_COLOR));
        self.canvas_context.fill_rect(0.0, 0.0, DEFAULT_WIDTH as f64, DEFAULT_HEIGHT as f64);
    }

    fn reset_hard(&mut self) {
        self.reset_system();
    }

    fn setup_widget(&mut self, pkg: &Map) {
        pkg.set(&JsValue::from_str("canvas"), &self.canvas);
    }

    fn device_id(&self) -> DeviceId {
        DeviceId::PixelScreen
    }
}

impl AddressableDeviceTrait for PixelScreen {
    fn size(&self) -> u16 {
        DEFAULT_WIDTH * DEFAULT_HEIGHT
    }

    fn read_unchecked(&self, offset: u16) -> u8 {
        0 //TODO
    }

    fn write_unchecked(&mut self, offset: u16, value: u8) {
        let x = offset % 50;
        let y = offset / 50;

        let color = COLOR_PALETTE.get(value as usize)
            .unwrap_or(&BACKGROUND_COLOR);

        self.canvas_context.set_fill_style(&JsValue::from_str(color));
        self.canvas_context.fill_rect(x as f64, y as f64, 1.0, 1.0);
    }
}
