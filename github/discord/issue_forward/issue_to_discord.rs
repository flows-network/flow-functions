#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use serde_json::{Result, Value};

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
						"opened" => {
							if let Some(issue) = pl.get("issue") {
								event_type = "issue created";
								body = issue.get("body").unwrap().as_str().unwrap().to_string();
								html_url = issue.get("html_url").unwrap().as_str().unwrap();
							}
						}
						"edited" => {
							if let Some(comment) = pl.get("comment") {
								event_type = "issue comment edited";
								body = comment.get("body").unwrap().as_str().unwrap().to_string();
								html_url = comment.get("html_url").unwrap().as_str().unwrap();
							} else if let Some(issue) = pl.get("issue") {
								event_type = "issue edited";
								body = issue.get("body").unwrap().as_str().unwrap().to_string();
								html_url = issue.get("html_url").unwrap().as_str().unwrap();
							}
						}
						"created" => {
							if let Some(comment) = pl.get("comment") {
								event_type = "issue comment created";
								html_url = comment.get("html_url").unwrap().as_str().unwrap();
								body = comment.get("body").unwrap().as_str().unwrap().to_string();
							}
						}
						"assigned" => {
							if let Some(assignee) = pl.get("assignee") {
								event_type = "issue assigned";
								html_url = pl.get("issue").unwrap().get("html_url").unwrap().as_str().unwrap();
								body = format!("Issue assigned to @{}.", assignee.get("login").unwrap().as_str().unwrap());
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
			if body.contains("@chen982") {
				return format!("{}\n{}\n{}", event_type, body.replace("\\r\\n", "\n"), html_url);
			}
		}
	}


	return "".to_string();
}