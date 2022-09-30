use flows_connector_dsi::{github::inbound, jira::outbound};
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;

    if payload.get_action()? != "opened" {
        return Ok(String::new());
    }

    let pull_request = payload.get_pull_request()?;

    outbound::create_issue(&pull_request.title)
        .description(
            pull_request
                .body
                .as_ref()
                .unwrap_or(&"No description.".to_string()),
        )
        .transition(&pull_request.state)
        .build()
}
