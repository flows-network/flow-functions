use serde_json::Value;
use wasmedge_bindgen_macro::*;

#[cfg(target_family = "wasm")]
#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

fn _run(s: String) -> String {
    let payload: Value = serde_json::from_str(&s).unwrap();

    let action = payload.get("action").unwrap().as_str().unwrap();
    if action != "opened" {
        return String::new();
    }

    let issue = payload.get("issue").unwrap();

    let number = issue.get("number").unwrap().as_i64().unwrap();

    serde_json::json!({
        "issue_number": number,
        "body": "Thanks for your feedback. Our maintainer will respond to you soon.",
   }).to_string()
}
