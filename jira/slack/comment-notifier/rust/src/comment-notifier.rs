use flows_connector_dsi::jira::inbound;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;

    Ok(format!(
        "{} on issue `{}`",
        event_map_action(&payload.webhook_event, &payload.get_comment()?.body),
        &payload.get_issue()?.key
    ))
}

fn event_map_action(event: &String, content: &String) -> String {
    match event.as_str() {
        "comment_created" => format!("Created new comment `{}`", content),
        "comment_updated" => format!("Updated `{}` comment", content),
        "comment_deleted" => format!("Deleted `{}` comment", content),
        _ => unreachable!(),
    }
}
