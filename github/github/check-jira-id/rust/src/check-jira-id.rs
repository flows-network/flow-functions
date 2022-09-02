use std::collections::HashSet;

use regex::Regex;
use serde_json::Value;
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

    let action = payload.get("action").unwrap().as_str().unwrap();
    if action != "opened" && action != "edited" {
        return String::new();
    }

    let pull_request = payload.get("pull_request").unwrap();

    let number = pull_request.get("number").unwrap().as_i64().unwrap();
    let title = pull_request.get("title").unwrap().as_str().unwrap();

    let mut bodies: Vec<String> = vec![];
    let mut assignees: HashSet<&str> = HashSet::new();

    let re = Regex::new(r"^[A-Za-z]{3}-\d+").unwrap();

    if !re.is_match(title) {
        bodies.push(String::from(
            "This PR's title doesn't match our requirement.",
        ));
        bodies.push(String::from("Please make sure your titile includes the corresponding Jira id."));
        bodies.push(format!(
            "@{}, please update it.",
            pull_request["user"]["login"].as_str().unwrap()
        ));
    } else {
        bodies.push(String::from("Welcome"));

        assignees.insert("jetjinser");
    };

    serde_json::json!({
        "issue_number": number,
        "body": bodies.join("\n").to_owned(),
        "assignees": assignees,
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn invalid() {
        use crate::_run;
        let s = String::from(
            r#"
            {
                "action": "opened",
                "pull_request": {
                    "number": 1,
                    "title": "untitled",
                    "user": {
                        "login": "someone"
                    }
                }
            }
        "#,
        );

        let r: serde_json::Value = serde_json::from_str(&_run(s)).unwrap();

        assert_ne!(r["assignees"][0], "jetjinser");
    }

    #[test]
    fn valid() {
        use crate::_run;
        let s = String::from(
            r#"
            {
                "action": "edited",
                "pull_request": {
                    "number": 1,
                    "title": "ABC-1234 abstraction",
                    "user": {
                        "login": "someone"
                    }
                }
            }
        "#,
        );

        let r: serde_json::Value = serde_json::from_str(&_run(s)).unwrap();

        assert_eq!(r["assignees"][0], "jetjinser");
    }
}
