use flows_connector_dsi::github::inbound;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let review = payload.get_review()?;

    Ok(format!(
        "Pull request review {} in {}({})!\n[{}] `{}`\nDetil: {}",
        payload.get_action()?,
        payload.get_pull_request()?.title,
        payload.get_repository()?.full_name,
        review.state,
        review
            .body
            .as_ref()
            .unwrap_or(&String::from("no description"))
            .replace("\\r\\n", "\n"),
        review.html_url
    ))
}
