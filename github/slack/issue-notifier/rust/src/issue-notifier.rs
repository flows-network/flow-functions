use connector_dsi::github::inbound;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let issue = payload.get_issue()?;

    let mut event_type = "Issue ".to_string();
    let mut body = issue.body.clone().unwrap_or_default();
    let mut html_url = &issue.html_url;

    if let Some(ref comment) = payload.comment {
        event_type.push_str("comment ");
        body = format!("{}: \"{}\"", payload.sender.login, comment.body);
        html_url = &comment.html_url;
    } else if !issue.assignees.is_empty() {
        body = format!("Issue assigned to {}", issue.assignees.iter().map(|u| format!("@{}", u.login)).collect::<Vec<_>>().join(", "))
    }

    Ok(format!(
        "{}!\n{}\n{}",
        event_type + payload.get_action()?,
        body.replace("\\r\\n", "\n"),
        html_url
    ))
}
