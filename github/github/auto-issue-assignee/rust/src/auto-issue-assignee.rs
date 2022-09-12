use std::collections::{HashMap, HashSet};
use serde_json::{Value};
use wasmedge_bindgen_macro::*;

#[cfg(target_family = "wasm")]
#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

fn _run(s: String) -> String {
    let keywords_tags_table: Vec<(&str, Vec<&str>)> = vec![("Platforms", vec!["Android", "OHOS", "OpenWrt"]),
                                                           ("Bindings", vec!["Rust"]),
                                                           ("Internal Components", vec!["AOT", "AST", "Common", "Conf", "Core", "Driver", "Executor", "Interpreter", "Loader", "Log", "Runtime", "Statistics", "Support", "System", "PO", "Validator", "VM"]),
                                                           ("WASM/WASI", vec!["TO-BE-SPECIFIED"]),
                                                           ("Proposals and extensions", vec!["EVMC", "SIMD", "WASI", "WASI-Crypto", "WASI-NN", "WASI-Socket"]),
                                                           ("Exposed API", vec!["API"]),
                                                           ("CI and Testing", vec!["CI", "Docker", "Test"]),
                                                           ("Misc", vec!["Changelog", "CMake", "Deps", "Docs", "Examples", "Installer", "Misc", "Plugin", "Refactoring", "RPM", "SRPM", "Thirdparty", "Tools", "Utils"]),
                                                           ("Bindings", vec!["Rust"]),
                                                           ("Internal Components", vec!["AOT", "AST", "Common", "Conf", "Core", "Driver", "Executor", "Interpreter", "Loader", "Log", "Runtime", "Statistics", "Support", "System", "PO", "Validator", "VM"]),
                                                           ("bug", vec!["can\'t", "bug", "not", "fail", "error"]),
                                                           ("documentation", vec!["doc", "book", "translat", "demo", ".md"]),
                                                           ("duplicate", vec!["copy", "duplicate"]),
                                                           ("enhancement", vec!["improve", "enhance", "support", "implement", "remove", "enable", "roadmap"]),
                                                           ("good first issue", vec!["demo", "add", "example"]),
                                                           ("support", vec!["help", "need", "assistance"]),
                                                           ("feature", vec!["design", "create", "propose", "support", "provide", "feat"]),
                                                           ("invalid", vec!["invalid", "wrong", "mistake"]),
                                                           ("question", vec!["ask", "question",
                                                                             "answer", "possible", "how", "what", "potential", "likely"]),
                                                           ("wontfix", vec!["TO-BE-SPECIFIED"])];
    let keywords_tags_map = keywords_tags_table.into_iter().map(|(word, arr)| {
        let word_string = word.to_string();
        let vec_string = arr.into_iter().map(|x| x.to_ascii_lowercase().to_string()).collect::<Vec<String>>();
        (word_string, vec_string)
    }).collect::<HashMap<String, Vec<String>>>();

    let payload: Value = serde_json::from_str(&s).unwrap();


    let mut labels: HashSet<String> = HashSet::new();

    let is_issue_task = payload.get("issue").is_some();
    let is_pr_task = payload.get("pull_request").is_some();


    let (number, title, mut bodies, mut assignee, mut assignees) = if is_issue_task {
        extract_inner(&payload, "issue")
    } else if is_pr_task {
        extract_inner(&payload, "pull_request")
    } else {
        (0, "NO-TITLE".to_string(), vec!["EMPTY-BODY".to_string()], "NO-ASSIGNEE".to_string(),
         HashSet::<String>::new())
    };


    let title_as_tags: HashSet<String> = title_to_tags(&title);
    labels = assign_labels(&keywords_tags_map, &title_as_tags);
    assignees = specify_assignee(&keywords_tags_map, &title_as_tags);
    bodies.push(format!(
        "The task has been asigned to @{:?}.",
        assignees,
    ));

    let body = bodies.join("\n");
    labels.insert("whatever".to_string());
    assignees.insert("jaykchen".to_string());
    assignees.insert("amiiiiii830".to_string());

    serde_json::json!({
                "issue_number": number,
                "body": body,
                "labels": labels,
                "assignees": assignees,
                }).to_string()
}

pub fn is_special_char(inp: char) -> bool {
    let special_chars = r#".'-_/"#;
    special_chars.chars().any(|c| c == inp)
}

pub fn assign_labels(dic: &HashMap<String, Vec<String>>, inp: &HashSet<String>) -> HashSet<String> {
    let mut issue_or_pr_tags: HashSet<String> = HashSet::<String>::new();
    for needle in inp {
        let needle = needle.to_string();
        for (k, v) in dic.iter() {
            if v.contains(&needle) {
                issue_or_pr_tags.insert(k.clone());
            }
        }
    }

    issue_or_pr_tags.into_iter().map(|x| x.to_string()).collect::<HashSet<String>>()
}

pub fn extract_inner(payload: &Value, element_type: &str) -> (i64, String, Vec<String>,
                                                              String,
                                                              HashSet<String>) {
    let element = payload.get(element_type).unwrap();
    let element_owner = element.get("user").unwrap()["login"].as_str().unwrap();
    let number = element.get("number").unwrap().as_i64().unwrap();
    let title = element["title"].as_str().unwrap();
    let assignee = element.get("assignee").unwrap()["login"].as_str().unwrap_or("");

    let assignees = match element.get("assignees") {
        Some(list) => list.as_array().unwrap().iter().map(|person|
            person["login"].as_str().unwrap().to_string()).collect::<HashSet<String>>(),
        None => HashSet::new(),
    };

    let element_type_name = match element_type {
        "issue" => "issue",
        "pull_request" => "pull request",
        _ => unreachable!(),
    };
    let bodies = vec![format!(
        "Thank you @{} for submitting {}!",
        element_owner,
        element_type_name,
    )];

    (number, title.to_string(), bodies, assignee.to_string(), assignees)
}


pub fn title_to_tags(title: &String) -> HashSet<String> {
    title.trim().split_whitespace().map(|word| {
        word.chars().filter(|&c| c.is_alphanumeric() | is_special_char(c)).collect::<String>()
    }).collect::<HashSet<String>>()
}

pub fn specify_assignee(dic: &HashMap<String, Vec<String>>, inp: &HashSet<String>) -> HashSet<String> {
    let mut res: HashSet<&str> = HashSet::new();
    let mut issue_or_pr_tags: HashSet<String> = HashSet::<String>::new();
    for needle in inp {
        for (k, v) in dic.iter() {
            if v.contains(needle) {
                issue_or_pr_tags.insert(k.clone());
                break;
            }
        }
    }

    for tag in issue_or_pr_tags {
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
