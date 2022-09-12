use std::collections::{HashMap, HashSet};
use serde_json::{to_string, Value};
use wasmedge_bindgen_macro::*;

#[cfg(target_family = "wasm")]
#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

fn _run(s: String) -> String {
    let keywords_label_table: Vec<(&str, Vec<&str>)> = vec![("Platforms", vec!["Android", "OHOS", "OpenWrt"]),
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
                                                            ("question", vec!["ask", "question", "answer", "possible", "how", "what", "potential", "likely"])];


    let keywords_label_map = keywords_label_table.into_iter().map(|(word, arr)| {
        let word_string = word.to_string();
        let vec_string = arr.into_iter().map(|x| x.to_ascii_lowercase().to_string()).collect::<Vec<String>>();
        (word_string, vec_string)
    }).collect::<HashMap<String, Vec<String>>>();


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
        word.to_ascii_lowercase().chars().filter(|&c| c.is_alphanumeric() | is_special_char(c)).collect::<String>()
    }).collect::<Vec<String>>();

    let labels: Vec<String> = assign_label(keywords_label_map, &title_as_keys);

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

pub fn is_special_char(inp: char) -> bool {
    let special_chars = r#".'-_/"#;
    special_chars.chars().any(|c| c == inp)
}

pub fn assign_label(dic: HashMap<String, Vec<String>>, inp: &Vec<String>) -> Vec<String> {
    let mut res = HashSet::<String>::new();
    for needle in inp {
        for (k, v) in dic.iter() {
            if v.contains(needle) {
                res.insert(k.clone());
            }
        }
    }
    res.into_iter().collect::<Vec<String>>()
}

