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
    let pull_request = payload.get_pull_request()?;

    Ok(format!(
        "Pull request #{} {} was {} in {}",
        pull_request.number,
        pull_request.title,
        pull_request.html_url,
        payload.get_action()?
    ))
}
