use flows_connector_dsi::{github::inbound, monday::outbound};
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let pull_request = payload.get_pull_request()?;

    outbound(&pull_request.html_url)
        .status_label("status", pr_state_to_label(&pull_request.state))
        .build()
}

fn pr_state_to_label(state: &String) -> String {
    match state.as_str() {
        "open" => "Opened",
        "closed" => "Closed",
        "merged" => "Merged",
        "draft" => "Draft",
        _ => "",
    }
    .to_string()
}
