use serde_json::{Result, Value};
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let result: Result<Value> = serde_json::from_str(s.as_str());
    let mut body: String = String::new();
    let mut event_type: &str = "";
    let mut html_url: &str = "";
    if let Ok(pl) = result {
        match pl.get("action") {
            Some(action) => {
                if let Some(action) = action.as_str() {
                    match action {
                        "deleted" => {
                            if let Some(label) = pl.get("label") {
                                return label.to_string();
                                event_type = "label deleted";
                                body = label.get("name").unwrap().to_string();

                                html_url = pl.get("repository").unwrap().get("html_url").unwrap().as_str().unwrap();
                            }
                        }

                        _ => {
                            return "".to_string();
                        }
                    }
                }
            }
            None => {
                return "".to_string();
            }
        }
    }

    return "".to_string();
}
