use flows_connector_dsi::github::{inbound, outbound};
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let issue = payload.get_issue()?;

    let assignees = match issue.title.as_str() {
        t if t.starts_with("platforms") || issue.title.starts_with("ci") => {
            vec!["jaykchen"]
        }
        t if t.starts_with("docs") => vec!["alabulei1"],
        _ => vec![],
    };

    outbound::modify_issue(issue.number)
        .body(if assignees.is_empty() {
            format!(
                "This issue title is not compliant with our community rules. @{}, please \
update it. Valid types: docs, ci, platforms.",
                payload.sender.login
            )
        } else {
            format!(
                "Thank you @{} for submitting this issue! It has been assigned to @{}",
                payload.sender.login,
                assignees.join(", @"),
            )
        })
        .assignees(assignees)
        .build()
}
