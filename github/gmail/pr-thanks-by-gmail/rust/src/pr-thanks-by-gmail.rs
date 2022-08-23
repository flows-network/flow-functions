use serde_json::{Result, Value, json};

#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let res: Result<Value> = serde_json::from_str(&s);
    if let Ok(v) = res {
        if v.get("sender_email").is_some() && v.get("pull_request").is_some() {
            if let Some(action) = v["action"].as_str() {
                let sender = v["sender"]["login"].as_str().unwrap();
                let repo = v["repository"]["full_name"].as_str().unwrap();
                let email = v["sender_email"].as_str().unwrap();

                let mut subject = String::new();
                let mut content = String::new();

                if action == "opened" {
                    subject = "Thank you for Pull Request this repository".to_string();
                    content = format!(
                        r#"
Howdy {}, <br/>
Welcome to the {} community, Thank you for Pull Request this repository.
                        "#,
                        sender, repo
                    );
                }

                return json!({
                    "to_email": email,
                    "subject": subject,
                    "content": content,
                })
                .to_string();
            }
        }
    };

    return "".to_string();
}