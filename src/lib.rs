use wasm_bindgen::prelude::*;

// This function is automatically called when the WASM is initialized.
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` and `document` objects
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Create a new `<h1>` element
    let val = document.create_element("h1")?;
    val.set_inner_html("Hello from Rust and WASM!");

    // Append the `<h1>` to the `<body>`
    body.append_child(&val)?;

    Ok(())
}
