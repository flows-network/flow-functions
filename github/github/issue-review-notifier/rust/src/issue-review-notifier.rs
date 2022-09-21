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

    let mut assignees = Vec::new();
    let mut body = String::new();

    if issue.title.contains("platforms") || issue.title.contains("ci") {
        assignees.push("jaykchen");
    } else if issue.title.contains("docs") {
        assignees.push("alabulei1");
    } else {
        body = format!(
            "This issue title is not compliant with our community rules. @{}, please
        update it. Valid types: docs, ci, platforms.",
            payload.sender.login
        );
    }

    outbound::modify_issue(issue.number)
        .body(if body.is_empty() {
            format!(
                "Thank you @{} for submitting this issue! It has been assigned to @{}",
                payload.sender.login,
                assignees.join(", @"),
            )
        } else {
            body
        })
        .assignees(assignees)
        .build()
}
