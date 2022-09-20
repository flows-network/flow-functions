use serde_json::json;
use flows_connector_dsi::{github::inbound};
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen;
    _run(s)
}

fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    
    match payload.pull_request {
        None => { return Err("no pull request found".to_string()); },
        Some(_) => {},
    }

    let mut subject: &str = "";
    let mut content = String::new();

    if payload.action.unwrap() == "opened" {
        subject = "Thank you for contributing to this repository";
        content = format!(
            r#"
Hi {}, <br/>

Welcome to the {} community, thank you for your contribution!
                        "#,
            payload.sender.login, payload.repository.unwrap().full_name
        );
    }

    Ok(json!([{
                            "personalizations": [
                                {
                                    "to": [
                                        {
                                            "email": payload.sender.email,
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
                        }]).to_string())
}
