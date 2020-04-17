use js_sys::Array;
use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::{console, window};

// Javascript bindings

// First method to bind to `console.log`
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(a: &str);
// }

/// Second Method to bind to `console.log`
/// This leverages on the rust provided `js-sys` binding already developed
/// by the rust wasm team.
///
/// If they hadn't written the binding, I would have to use the custom
/// defined binding I declared in method 1 above (it's currently commented).
macro_rules! console_log {
    ($($t:tt)*) => {
        let js_value = JsValue::from_str((&format_args!($($t)*).to_string()));
        console::log(&Array::from(&js_value))
    }
}

/// Parse a stringified version of a json file
#[wasm_bindgen]
pub fn parse(text: &str) -> Result<JsValue, JsValue> {
    // Get DOM Instance
    let window = window().expect("window missing");
    let document = window.document().expect("document missing");
    let body = document.body().expect("body missing");
    let elem = document
        .create_element("p")
        .expect("Failed to create paragraph");
    let performance = window
        .performance()
        .expect("Performance not available in browser window");
    let curr_time = performance.now();

    // Start benchmarking
    let start = performance.timing().request_start();
    let json: serde_json::Value =
        serde_json::from_str(&text).map_err(|_| JsValue::from_str("Failed to serialize json"))?;
    let end = performance.timing().response_end();

    // Append Result to Browser's DOM Element
    body.append_child(&elem).unwrap();
    elem.set_inner_html(&format!(
        r#"
        The current time (in ms) is {} <br>
        Started by {} <br>
        Ended by {} <br>
        Difference is {}
        "#,
        curr_time as u64 / 1_000,
        start as u64 / 1_000,
        end as u64 / 1_000,
        (end - start) as u64 / 1_000
    ));

    console_log!("Hello from rust"); //  logs to the browser tab.
    JsValue::from_serde(&json).map_err(|_| JsValue::from_str("failed to serialize json"))
}

/// Multiply a number by 30
#[wasm_bindgen(js_name = multiplyBy30)]
pub fn multiply_by_30(value: i32) -> i32 {
    value * 30
}
