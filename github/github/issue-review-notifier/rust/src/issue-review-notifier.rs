use serde_json::{Value};
use std::collections::{ HashSet};
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
    let sender = payload.get("sender").unwrap()["login"].as_str().unwrap();
    let issue = payload.get("issue").unwrap();
    let number = issue.get("number").unwrap().as_i64().unwrap();
    let title = issue["title"].as_str().unwrap().to_string();

    let mut is_compliant = false;
    let mut bodies = Vec::<String>::new();
    let mut assignees = HashSet::new();

    let title_as_tags = title.trim().split_whitespace().map(|word| {
        word.to_ascii_lowercase().chars().filter(|&c| c.is_alphanumeric()).collect::<String>()
    }).collect::<HashSet<String>>();


    for tag in title_as_tags {
        match tag.as_str() {
            "platforms" | "ci" => {
                is_compliant = true;
                assignees.insert("jaykchen");
            },
            "docs" => {
                is_compliant = true;
                assignees.insert("alabulei1");
            },
            _ => {},
        };
    }
    let assignees_str = assignees.iter().map(|person| {
        let mut name = String::from("@");
        name.push_str(person);
        name
    }).collect::<Vec<String>>().join(" ");

    if is_compliant {
        bodies.push(format!(
            "Thank you @{} for submitting this issue! It has been assigned to {}",
            sender,
            assignees_str
        ));
    } else {
        bodies.push(format!("This issue title is not compliant with our community rules. @{}, please
     update it. Valid types: docs, ci, platforms.", sender));
    }


    serde_json::json!({
    "issue_number": number,
    "body": bodies.join("\n"),
    "assignees": assignees,
    }).to_string()
}

