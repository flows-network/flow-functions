use std::collections::HashSet;

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

    if payload.get("action").unwrap() != "opened" {
        return String::new();
    }

    let pull_request = payload.get("pull_request").unwrap();

    let number = pull_request.get("number").unwrap().as_i64().unwrap();
    let title = pull_request.get("title").unwrap().as_str().unwrap();

    let mut bodies: Vec<String> = vec![];
    let mut labels: HashSet<&str> = HashSet::new();
    let mut assignees: HashSet<&str> = HashSet::new();

    let mut chars = title.trim().chars();

    let first_char = chars.next().unwrap();
    let aspect: String = chars.take_while(|&c| c != ']').collect();

    if first_char != '['
        || aspect.len() >= title.len() - 2
        || !vec![
            // Platforms
            "Android",
            "OHOS",
            "OpenWrt",
            // Bindings
            "Rust",
            // Internal Components
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
            // WASM/WASI Proposals and extensions
            "EVMC",
            "SIMD",
            "WASI",
            "WASI-Crypto",
            "WASI-NN",
            "WASI-Socket",
            // Exposed API
            "API",
            // CI and Testing
            "CI",
            "Docker",
            "Test",
            // Misc
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
        ]
        .contains(&aspect.as_ref())
    {
        bodies.push(String::from(
            "This PR's title doesn't match our requirement. ",
        ));
        bodies.push(String::from(
            "Please check: https://hackmd.io/@n5yKF-gRQ0axsmsusU3rNA/ryRjtTnC5",
        ));
        bodies.push(format!(
            "@{}, please update it.",
            pull_request["user"]["login"].as_str().unwrap()
        ));

        labels.insert("invalid");
    } else {
        bodies.push(String::from("Welcome"));

        labels.insert("good first issue");

        assignees.insert("jetjinser");
    };

    serde_json::json!({
        "issue_number": number,
        "body": bodies.join("\n").to_owned(),
        "labels": labels,
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

        assert_eq!(r["labels"][0], "invalid");
    }

    #[test]
    fn valid() {
        use crate::_run;
        let s = String::from(
            r#"
            {
                "action": "opened",
                "pull_request": {
                    "number": 1,
                    "title": "[Rust] Ideas",
                    "user": {
                        "login": "someone"
                    }
                }
            }
        "#,
        );

        let r: serde_json::Value = serde_json::from_str(&_run(s)).unwrap();

        assert_eq!(r["labels"][0], "good first issue");
    }
}
