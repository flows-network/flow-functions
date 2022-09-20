#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use flows_connector_dsi::github::inbound;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let comment = payload.get_comment()?;

    if payload.get_action()? != "created" {
        return Ok(String::new());
    }

    Ok(format!(
        "{}\n{}",
        comment.body.replace("\\r\\n", "\n"),
        comment.html_url
    ))
}