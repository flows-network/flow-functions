use flows_connector_dsi::github::inbound;
use wasmedge_bindgen_macro::*;

use std::collections::HashMap;

// {{{
lazy_static::lazy_static! {
    static ref REVIEWERS: HashMap<&'static str, Vec<&'static str>> = {
        let mut reviewers = HashMap::new();

        // Platforms
        reviewers.insert("reviewer 1", vec!["[Android]", "[OHOS]", "[OpenWrt]"]);

        // Bindings
        reviewers.insert("reviewer 2", vec!["[Rust]"]);

        // Internal Components
        reviewers.insert(
            "reviewer 3",
            vec![
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
            ],
        );

        // WASM/WASI Proposals and extensions
        reviewers.insert(
            "reviewer 4",
            vec![
                "[EVMC]",
                "[SIMD]",
                "[WASI]",
                "[WASI-Crypto]",
                "[WASI-NN]",
                "[WASI-Socket]",
            ],
        );

        // Exposed API
        reviewers.insert("reviewer 5", vec!["[API]"]);

        // CI and Testing
        reviewers.insert("reviewer 6", vec!["[CI]", "[Docker]", "[Test]"]);
        // Misc
        reviewers.insert(
            "reviewer 7",
            vec![
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
            ],
        );

        reviewers
    };
}
// }}}

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

    let mut reviewers = Vec::new();

    let title = &pull_request.title;

    let reason_or_welcome = if title.starts_with(" ") {
        Err("Title cannot start with blank.")
    } else {
        let mut t = Err("Invalid tag.");
        for (reviewer, valid_tags) in &*REVIEWERS {
            if valid_tags.iter().any(|t| title.starts_with(t)) {
                let mut iter = title.chars().skip_while(|&c| c != ']').skip(1);
                if iter.next().unwrap_or_default() != ' ' || iter.next().unwrap_or_default() == ' '
                {
                    t = Err("There must be a space between tag and content.");
                    break;
                }

                reviewers.push(reviewer);
                t = Ok("Welcome!");
                break;
            }
        }
        t
    };

    let body = match reason_or_welcome {
        Ok(b) => b.to_string(),
        Err(r) => {
            format!(
                r#"This PR's title doesn't match our requirement. {}
Please check: <https://hackmd.io/@n5yKF-gRQ0axsmsusU3rNA/ryRjtTnC5>
@{}, please update it."#,
                r, payload.sender.login
            )
        }
    };

    Ok(serde_json::json!({
        "pull_number": pull_request.number,
        "reviewers": reviewers,

        "issue_number": pull_request.number,
        "body": body,
    })
    .to_string())
}
