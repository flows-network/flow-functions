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
							subject = " 😉 Thank you for your star!".to_string();
							content = format!(r#"
Howdy {}, 
								
Welcome to the {} community! Thank you so much for starring our repository. We are very excited to have you here.

Look forward to your contributions,
Maintainers at repository https://github.com/{}"#, sender, repo, repo);
						} else {
							subject = " 😿 Sorry to lose you".to_string();
							content = format!(r#"
Hi {},
			
Sorry to see you go! We value your feedback and suggestions. Please do let us know how we might improve the repository {} (just reply to this email). We wish to see your around in the community!

Best Regards,
Maintainers at repository https://github.com/{}"#, sender, repo, repo);
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
