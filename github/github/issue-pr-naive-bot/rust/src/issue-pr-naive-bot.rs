use flows_connector_dsi::github::{inbound, outbound, InboundData};
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

fn _run(s: String) -> Result<String, String> {
    let keywords_tags_map = [
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
    ];

    let payload = inbound(s)?;
    let mut data = extract_pr_or_issue(&payload)?;

    let labels = &mut data.labels;
    if labels.is_empty() {
        for (key, labs) in keywords_tags_map {
            if data.title.contains(key) {
                labels.append(
                    &mut labs
                        .into_iter()
                        .map(|name| name.to_string())
                        .collect::<Vec<_>>(),
                );
            }
        }
    }

    let assignees = &mut data.assignees;
    if assignees.is_empty() {
        assignees.push(data.owner.clone());
    }

    outbound::modify_issue(data.number)
        .assignees(assignees.into_iter().collect())
        .labels(labels.into_iter().collect())
        .body(format!(
            "Thank you @{} for submitting {}!\nThe task has been asigned to @{}.",
            payload.sender.login,
            data.event_type,
            assignees.join(", @")
        ))
        .build()
}

struct ExtractedData<'a> {
    number: u32,
    title: &'a String,
    event_type: String,
    labels: Vec<String>,
    assignees: Vec<String>,
    owner: &'a String,
}

fn extract_pr_or_issue<'a>(payload: &'a InboundData) -> Result<ExtractedData<'a>, String> {
    if let Some(issue) = payload.issue.as_ref() {
        Ok(ExtractedData {
            number: issue.number,
            title: &issue.title,
            event_type: "issue".to_string(),
            labels: issue.labels.iter().map(|lab| lab.name.clone()).collect(),
            assignees: issue
                .assignees
                .iter()
                .map(|assignee| assignee.login.clone())
                .collect(),
            owner: &issue.user.login,
        })
    } else if let Some(pull_request) = payload.pull_request.as_ref() {
        Ok(ExtractedData {
            number: pull_request.number,
            title: &pull_request.title,
            event_type: "pull request".to_string(),
            labels: pull_request
                .labels
                .iter()
                .map(|lab| lab.name.clone())
                .collect(),
            assignees: pull_request
                .assignees
                .iter()
                .map(|assignee| assignee.login.clone())
                .collect(),
            owner: &pull_request.user.login,
        })
    } else {
        Err("Missing issue and pull_request fields".to_string())
    }
}
