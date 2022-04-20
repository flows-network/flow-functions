use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use serde_json::{Result, Value};

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
	let result: Result<Value> = serde_json::from_str(s.as_str());
	if let Ok(pl) = result {
		match pl.get("action") {
			Some(action) => {
				if let Some(action) = action.as_str() {
					match action {
						"opened" => {
							if let Some(issue) = pl.get("issue") {
								return format!("issue created\n{}", issue.get("body").unwrap().as_str().unwrap().replace("\\r\\n", "\n"));
							}
						}
						"edited" => {
							if let Some(comment) = pl.get("comment") {
								return format!("issue comment edited\n{}", comment.get("body").unwrap().as_str().unwrap().replace("\\r\\n", "\n"));
							} else if let Some(issue) = pl.get("issue") {
								return format!("issue edited\n{}", issue.get("body").unwrap().as_str().unwrap().replace("\\r\\n", "\n"));
							}
						}
						"created" => {
							if let Some(issue) = pl.get("starred_at") {
								return format!("start created");
							} else if let Some(comment) = pl.get("comment") {
								return format!("issue comment created\n{}", comment.get("body").unwrap().as_str().unwrap().replace("\\r\\n", "\n"));
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

	return s;
}