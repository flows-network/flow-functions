use serde_json::{Result, Value};
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let result: Result<Value> = serde_json::from_str(s.as_str());
    if let Ok(pl) = result {
        match pl.get("action") {
            Some(action) => {
                if let Some(action) = action.as_str() {
                    match action {
                        "created" => {
                            if let Some(comment) = pl.get("comment") {
                                let html_url = comment.get("html_url").unwrap().as_str().unwrap();
                                let body = comment.get("body").unwrap().as_str().unwrap().to_string();
                                return format!(
                                    "{}\n{}",
                                    body.replace("\\r\\n", "\n"),
                                    html_url
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    return String::new();
}