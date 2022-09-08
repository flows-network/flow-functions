#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(user: String) -> String {
    serde_json::json!({
        "text": "welcome",
        "user": user,
    })
    .to_string()
}
