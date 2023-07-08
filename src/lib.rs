mod app;
pub mod grid;
pub mod particle;

use app::*;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("window");
    let document = window.document().expect("document in window");
    let body = document.body().expect("body in document");

    let canvas = document.create_element("canvas")?;
    canvas.set_id("canvas");
    canvas.set_attribute("width", "1280")?;
    canvas.set_attribute("height", "720")?;
    canvas.set_attribute("oncontextmenu", "return false;")?;

    body.append_child(&canvas)?;

    Ok(())
}
