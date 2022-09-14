use serde_json::{Value};
use std::collections::{HashMap, HashSet};
use wasmedge_bindgen_macro::*;

#[cfg(target_family = "wasm")]
#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

fn _run(s: String) -> String {
    let keywords_tags_map: HashMap<&str, Vec<&str>> = vec![
        ("Platforms", vec!["Android", "OHOS", "OpenWrt"]),
        ("Bindings", vec!["Rust"]),
        (
            "Internal Components",
            vec![
                "AOT",
                "AST",
                "Common",
                "Conf",
                "Core",
                "Driver",
                "Executor",
                "Interpreter",
                "Loader",
                "Log",
                "Runtime",
                "Statistics",
                "Support",
                "System",
                "PO",
                "Validator",
                "VM",
            ],
        ),
        ("WASM/WASI", vec!["TO-BE-SPECIFIED"]),
        (
            "Proposals and extensions",
            vec![
                "EVMC",
                "SIMD",
                "WASI",
                "WASI-Crypto",
                "WASI-NN",
                "WASI-Socket",
            ],
        ),
        ("Exposed API", vec!["API"]),
        ("CI and Testing", vec!["CI", "Docker", "Test"]),
        (
            "Misc",
            vec![
                "Changelog",
                "CMake",
                "Deps",
                "Docs",
                "Examples",
                "Installer",
                "Misc",
                "Plugin",
                "Refactoring",
                "RPM",
                "SRPM",
                "Thirdparty",
                "Tools",
                "Utils",
            ],
        ),
        ("Bindings", vec!["Rust"]),
        (
            "Internal Components",
            vec![
                "AOT",
                "AST",
                "Common",
                "Conf",
                "Core",
                "Driver",
                "Executor",
                "Interpreter",
                "Loader",
                "Log",
                "Runtime",
                "Statistics",
                "Support",
                "System",
                "PO",
                "Validator",
                "VM",
            ],
        ),
        ("bug", vec!["can\'t", "bug", "not", "fail", "error"]),
        (
            "documentation",
            vec!["doc", "book", "translat", "demo", ".md"],
        ),
        ("duplicate", vec!["copy", "duplicate"]),
        (
            "enhancement",
            vec![
                "improve",
                "enhance",
                "support",
                "implement",
                "remove",
                "enable",
                "roadmap",
            ],
        ),
        ("good first issue", vec!["demo", "add", "example"]),
        ("support", vec!["help", "need", "assistance"]),
        (
            "feature",
            vec!["design", "create", "propose", "support", "provide", "feat"],
        ),
        ("invalid", vec!["invalid", "wrong", "mistake"]),
        (
            "question",
            vec![
                "ask",
                "question",
                "answer",
                "possible",
                "how",
                "what",
                "potential",
                "likely",
            ],
        ),
        ("wontfix", vec!["TO-BE-SPECIFIED"]),
    ].into_iter().collect::<HashMap<&str, Vec<&str>>>();

    let payload: Value = serde_json::from_str(&s).unwrap();

    let data = match payload {
        payload if payload.get("issue").is_some() => extract_inner(&payload, "issue"),
        payload if payload.get("pull_request").is_some() => extract_inner(&payload, "pull_request"),
        _ => unreachable!(),
    };

    let title_as_tags: HashSet<String> = title_to_tags(&data.title);

    let mut labels = data.labels;
    if labels.is_empty() {
        labels = assign_labels(&keywords_tags_map, &title_as_tags);
    }

    let mut assignees = data.assignees;
    if assignees.is_empty() {
        assignees.insert(data.sender);
        assignees.insert(data.owner);
        // assignees = specify_assignee(&keywords_tags_map, &title_as_tags);
    }
    let mut bodies = data.bodies;
    bodies.push(format!("The task has been asigned to @{:?}.", assignees));

    let body = bodies.join("\n");

    serde_json::json!({
    "issue_number": data.number,
    "body": body,
    "labels": labels,
    "assignees": assignees,
    }).to_string()
}


pub fn title_to_tags(title: &String) -> HashSet<String> {
    title.trim().split_whitespace().map(|word| {
        word.chars().filter(|&c| {
            let inp = c;
            let special_chars = r#".'-_/"#;
            c.is_alphanumeric() | special_chars.chars().any(|c| c == inp)
        }).collect::<String>()
    }).collect::<HashSet<String>>()
}

pub struct ExtractedData {
    number: i64,
    title: String,
    bodies: Vec<String>,
    labels: HashSet<String>,
    assignees: HashSet<String>,
    owner: String,
    sender: String,
}

pub fn extract_inner(
    payload: &Value,
    element_type: &str,
) -> ExtractedData {
    let sender = payload.get("sender").unwrap()["login"].as_str().unwrap().to_string();
    let element = payload.get(element_type).unwrap();
    let owner = element.get("user").unwrap()["login"].as_str().unwrap().to_string();
    let number = element.get("number").unwrap().as_i64().unwrap();
    let title = element["title"].as_str().unwrap().to_string();
    let labels = match element.get("labels") {
        Some(list) => list.as_array().unwrap().iter().map(|label| label["name"].as_str().unwrap().to_string()).collect::<HashSet<String>>(),
        None => HashSet::new(),
    };


    let assignees = match element.get("assignees") {
        Some(list) => list.as_array().unwrap().iter().map(|person| person["login"].as_str().unwrap().to_string()).collect::<HashSet<String>>(),
        None => HashSet::new(),
    };

    let element_type_name = match element_type {
        "issue" => "issue",
        "pull_request" => "pull request",
        _ => unreachable!(),
    };
    let bodies = vec![format!(
        "Thank you @{} for submitting {}!",
        owner, element_type_name,
    )];

    ExtractedData {
        number,
        title,
        bodies,
        labels,
        assignees,
        owner,
        sender
    }
}

pub fn assign_labels(dic: &HashMap<&str, Vec<&str>>, title_as_tags: &HashSet<String>) -> HashSet<String> {
    let mut label_assigned: HashSet<String> = HashSet::<String>::new();
    for tag in title_as_tags {
        for (k, v) in dic.iter() {
            if v.contains(&tag.as_str()) {
                label_assigned.insert(k.to_string());
                break;
            }
        }
    }

    label_assigned
}

pub fn specify_assignee(
    dic: &HashMap<&str, Vec<&str>>,
    title_as_tags: &HashSet<String>,
) -> HashSet<String> {
    let mut res: HashSet<&str> = HashSet::new();
    let labels_and_tags: HashSet<String> = assign_labels(dic, &title_as_tags);

    for tag in labels_and_tags {
        match tag.as_str() {
            "Platforms" => res.insert("jaykchen"),
            "Bindings" => res.insert("amiiiiii830"),
            "Internal Components" => res.insert("jaykchen"),
            "WASM/WASI" => res.insert("amiiiiii830"),
            "Proposals and extensions" => res.insert("jaykchen"),
            "Exposed API" => res.insert("amiiiiii830"),
            "CI and Testing" => res.insert("jaykchen"),
            "Misc" => res.insert("jaykchen"),
            "bug" => res.insert("jaykchen"),
            "documentation" => res.insert("jaykchen"),
            "duplicate" => res.insert("jaykchen"),
            "enhancement" => res.insert("jaykchen"),
            "good first issue" => res.insert("jaykchen"),
            "support" => res.insert("jaykchen"),
            "feature" => res.insert("jaykchen"),
            "invalid" => res.insert("jaykchen"),
            "question" => res.insert("amiiiiii830"),
            _ => res.insert("SOMEONE"),
        };
    }
    res.into_iter().map(|x| x.to_owned()).collect::<HashSet<String>>()
}
