use std::collections::HashSet;

use serde_json::Value;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let payload: Value = serde_json::from_str(&s).unwrap();

    let pull_request = payload.get("pull_request").unwrap();

    let number = pull_request.get("number").unwrap().as_i64().unwrap();
    let title = pull_request.get("title").unwrap().as_str().unwrap();

    let mut bodies: Vec<&str> = vec![];
    let mut labels: HashSet<&str> = HashSet::new();
    let mut assignees: HashSet<&str> = HashSet::new();

    let mut chars = title.trim().chars();

    if chars.next().unwrap() != '[' || chars.find(|&c| c == ']').is_none() {
        bodies.push("invalid Pull Request format");
        bodies.push("please check: https://hackmd.io/@n5yKF-gRQ0axsmsusU3rNA/ryRjtTnC5");

        labels.insert("invalid");
    } else if {
        let descriptor: String = chars.take_while(|&c| c != ']').collect();
        !vec![
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
        .contains(&descriptor.as_ref())
    } {
        bodies.push("invalid descriptor");
        bodies.push("please check: https://hackmd.io/@n5yKF-gRQ0axsmsusU3rNA/ryRjtTnC5");

        labels.insert("invalid");
    } else {
        bodies.push("welcom");

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
