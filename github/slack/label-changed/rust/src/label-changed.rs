use serde_json::{Result, Value};
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let result: Result<Value> = serde_json::from_str(s.as_str());
    let mut body: String = String::new();
    let mut event_type: &str = "";
    let mut repo_name = String::new();
    if let Ok(pl) = result {
        match pl.get("action") {
            Some(action) => {
                if let Some(action) = action.as_str() {
                    match action {
                        "created" => {
                            if let Some(label) = pl.get("label") {
                                event_type = "labels added";
                                body = label.get("name").unwrap().to_string();
                                repo_name = pl.get("repository").unwrap().as_str().unwrap().get
                                ("full_name").unwrap();
                            }
                        }
                        "edited" => {
                            if let Some(label) = pl.get("changes") {
                                event_type = "labels changed";
                                body = label.to_string();
                                // body = label.get("name").unwrap().to_string();

                                repo_name = pl.get("repository").unwrap().as_str().unwrap().get("full_name").unwrap();
                            }
                        }
                        "deleted" => {
                            if let Some(label) = pl.get("label") {
                                event_type = "label deleted";
                                body = label.get("name").unwrap().to_string();

                                repo_name = pl.get("repository").unwrap().as_str().unwrap().get("full_name").unwrap();
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
        if event_type != "" {
            return format!(
                "{}\n{}\n{}",
                event_type,
                body.replace("\\r\\n", "\n"),
                repo_name
            );
        }
    }

    return "".to_string();
}
