use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use serde_json::{Result, Value, json};

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
	let result: Result<Value> = serde_json::from_str(s.as_str());

	if let Ok(pl) = result {
		match pl.get("sender_email") {
			Some(email) => {
				// ensure event is star or unstar
				if pl.get("starred_at").is_some() {
					if let Some(action) = pl["action"].as_str() {
						let sender = pl["sender"]["login"].as_str().unwrap();
						let repo = pl["repository"]["full_name"].as_str().unwrap();

						let mut subject: String;
						let mut content: String;

						if action == "created" {
							subject = "Thanks for your star!".to_string();
							content = format!(r#"Hi {}, we have received your star to our repository {}.
									We are so appreciative and wish you have more fun with open source.
									
									Best regards"#, sender, repo);
						} else {
							subject = "Sorry to lose you".to_string();
							content = format!(r#"Hi {}, we are very disappointed that you have unstarred our repository {}.
									Hope you can give us more advice to make us getting better.
									
									Best regards"#, sender, repo);
						}
						return json!({
							"to_email": email.as_str().unwrap(),
							"subject": subject,
							"content": content
						}).to_string();
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