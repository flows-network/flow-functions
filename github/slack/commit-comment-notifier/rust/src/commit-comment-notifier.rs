use wasmedge_bindgen_macro::*;
use flows_connector_dsi::github::inbound;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let comment = payload.get_comment()?;

    Ok(format!(
        "{}\n{}\n{}",
        format!("Commit comment {}", payload.get_action()?),
        comment.body,
        comment.html_url,
    ))
}
