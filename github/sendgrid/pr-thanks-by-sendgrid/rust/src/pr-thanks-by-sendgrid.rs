use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
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

   if payload.get("pull_request").is_some() {
        if let Some(action) = payload["action"].as_str() {
            let sender = payload["sender"]["login"].as_str().unwrap();
            let repo = payload["repository"]["full_name"].as_str().unwrap();
            let email = payload["sender"]["email"].as_str().unwrap();

            let mut subject: &str = "";
            let mut content = String::new();

            if action == "opened" {
                subject = "Thank you for contributing to this repository";
                content = format!(
                    r#"
Hi {}, <br/>

Welcome to the {} community, thank you for your contribution!
                        "#,
                    sender, repo
                );
            }

            return json!([{
                            "personalizations": [
                                {
                                    "to": [
                                        {
                                            "email": email
                                        }
                                    ]
                                }
                            ],
                            "subject": subject,
                            "content": [
                                {
                                    "type": "text/html",
                                    "value": content
                                }
                            ]
                        }]).to_string();
        }
    }

     "".to_string()
}
