use flows_connector_dsi::{github::inbound, jira::outbound};
use regex::Regex;
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

    let reg = Regex::new(r"\[([A-Z]+-\S+)\]").unwrap();

    if let Some(matches) = reg.captures(&pull_request.title) {
        let key = matches.get(1).unwrap().as_str();
        outbound::modify_issue(key)
            .transition(&pull_request.state)
            .build()
    } else {
        Ok(String::new())
    }
}
