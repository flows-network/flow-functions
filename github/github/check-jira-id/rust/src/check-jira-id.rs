use flows_connector_dsi::github::{inbound, outbound};
use std::collections::HashSet;
use wasmedge_bindgen_macro::*;
use regex::Regex;

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

    let body;
    let mut assignees = pull_request
        .assignees
        .iter()
        .map(|u| u.login.as_str())
        .collect::<HashSet<_>>();

    let reg = Regex::new(r"^[A-Za-z]{3}-\d+").unwrap();

    if reg.is_match(&pull_request.title) {
        body = String::from("Welcome");

        assignees.insert("jetjinser");
    } else {
        body = format!(
            r#"This PR's title doesn't match our requirement.
Please make sure your title includes the corresponding Jira id.
@{}, please update it."#,
            pull_request.user.login
        );
    };

    outbound::modify_issue(pull_request.number)
        .assignees(assignees.into_iter().collect())
        .body(body)
        .build()
}
