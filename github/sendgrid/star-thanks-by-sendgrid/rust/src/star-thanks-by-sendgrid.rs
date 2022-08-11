use serde_json::{json, Result, Value};
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let result: Result<Value> = serde_json::from_str(s.as_str());

    if let Ok(pl) = result {
        match &pl["sender"]["email"] {
            Value::String(email) => {
                // ensure event is star or unstar
                if pl.get("starred_at").is_some() {
                    if let Some(action) = pl["action"].as_str() {
                        let sender = pl["sender"]["login"].as_str().unwrap();
                        let repo = pl["repository"]["full_name"].as_str().unwrap();

                        let subject: String;
                        let content: String;

                        if action == "created" {
                            subject = " 😉 Thank you for your star!".to_string();
                            content = format!(
                                r#"
Howdy {},<br/>
								
Welcome to the {} community! Thank you so much for starring our repository. We are very excited to have you here.<br/>

Look forward to your contributions,<br/>
Maintainers at repository {}"#,
                                sender, repo, repo
                            );
                        } else {
                            subject = " 😿 Sorry to lose you".to_string();
                            content = format!(
                                r#"
Hi {},<br/>
			
Sorry to see you go! We value your feedback and suggestions. Please do let us know how we might improve the repository {} (just reply to this email). We wish to see your around in the community!<br/>

Best Regards,<br/>
Maintainers at repository {}"#,
                                sender, repo, repo
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
                        }])
                        .to_string();
                    }
                }
            }
            _ => {
                return "".to_string();
            }
        }
    }

    return "".to_string();
}
