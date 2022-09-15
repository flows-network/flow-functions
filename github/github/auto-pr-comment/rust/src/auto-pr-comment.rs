use wasmedge_bindgen_macro::*;
use connector_dsi::github::{inbound, outbound};

#[cfg(target_family = "wasm")]
#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;

    let action = payload.get_action()?;

    if action != "opened" && action != "edited" {
        return Ok(String::new());
    }

    let pull_request = payload.get_pull_request()?;

    let vaild_tags = [
        // Platforms
        "[Android]",
        "[OHOS]",
        "[OpenWrt]",
        // Bindings
        "[Rust]",
        // Internal Components
        "[AOT]",
        "[AST]",
        "[Common]",
        "[Conf]",
        "[Core]",
        "[Driver]",
        "[Executor]",
        "[Interpreter]",
        "[Loader]",
        "[Log]",
        "[Runtime]",
        "[Statistics]",
        "[Support]",
        "[System]",
        "[PO]",
        "[Validator]",
        "[VM]",
        // WASM/WASI Proposals and extensions
        "[EVMC]",
        "[SIMD]",
        "[WASI]",
        "[WASI-Crypto]",
        "[WASI-NN]",
        "[WASI-Socket]",
        // Exposed API
        "[API]",
        // CI and Testing
        "[CI]",
        "[Docker]",
        "[Test]",
        // Misc
        "[Changelog]",
        "[CMake]",
        "[Deps]",
        "[Docs]",
        "[Examples]",
        "[Installer]",
        "[Misc]",
        "[Plugi]",
        "[Refactoring]",
        "[RPM]",
        "[SRPM]",
        "[Thirdparty]",
        "[Tools]",
        "[Utils]",
    ];

    let body;
    let mut labels = Vec::new();
    let mut assignees = Vec::new();

    if vaild_tags.iter().any(|s| pull_request.title.contains(s)) {
        body = String::from("Welcome");
        labels.push("good first issue");
        assignees.push(&payload.get_repository()?.owner.login);
    } else {
        body = format!(r#"
This PR's title doesn't match our requirement.
Please check: <https://hackmd.io/@n5yKF-gRQ0axsmsusU3rNA/ryRjtTnC5>
@{}, please update it.
        "#,
        payload.sender.login
        );

        // labels.push("invalid");
    };

    outbound::modify_issue(pull_request.number)
        .assignees(assignees)
        .labels(labels)
        .body(body)
        .build()
}
