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
    if action != "opened" {
        return String::new();
    }

    let issue = payload.get("issue").unwrap();

    let number = issue.get("number").unwrap().as_i64().unwrap();
    let title = issue.get("title").unwrap().as_str().unwrap();
    let issue_owner = issue.get("user").unwrap()["login"].as_str().unwrap();

    let mut bodies: Vec<String> = vec![];

    let title_as_keys = title.trim().split_whitespace().map(|word| {
        word.chars().filter(|&c| c.is_alphanumeric() | (c == '-') | (c == '_') | (c == '/')).collect::<String>()
    }).collect::<Vec<String>>();

    let labels: Vec<&str> = assign_label(&title_as_keys);

    bodies.push(format!(
        "Thank you @{} for your feedback. Our maintainer will deal with the issue according to the assigned label.",
        issue_owner
    ));

    serde_json::json!({
        "issue_number": number,
        "body": bodies,
        "labels": labels,
    }).to_string()
}

pub fn assign_label(inp: &Vec<String>) -> Vec<&str> {
    let mut res = vec![];
    match inp {
        inp if contains_keyword(inp, vec!["Bug", "bug"]) => res.push("bug"),
        inp if contains_keyword(inp, vec!["documents", "doc", "docs", "documentation",
                                          "documentations"]) => res.push("documentation"),
        inp if contains_keyword(inp, vec!["copy", "duplicate"]) => res.push("duplicate"),
        inp if contains_keyword(inp, vec!["improved", "enhance", "enhancement", "enhancements"]) => res.push("enhancement"),
        inp if inp.is_empty() => res.push("good first issue"),
        inp if contains_keyword(inp, vec!["help", "need", "assistance"]) => res.push("help wanted"),
        inp if contains_keyword(inp, vec!["invalid", "wrong", "mistake"]) => res.push("invalid"),
        inp if contains_keyword(inp, vec!["ask", "question", "answer"]) => res.push("question"),
        inp if contains_keyword(inp, vec!["disaster"]) => res.push("wontfix"),
        _ => res.push(inp[0].as_str()),
    }
    res
}

pub fn contains_keyword(needle: &Vec<String>, keywords: Vec<&str>) -> bool {
    let mut count = 0;

    for word in needle {
        for key in &keywords {
            if word.starts_with(key) {
                count += 1;
                break;
            }
        }
    }
    count >= 1
}

