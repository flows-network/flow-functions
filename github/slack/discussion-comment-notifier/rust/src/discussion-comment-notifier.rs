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
    let comment = payload.get_comment()?;

    Ok(format!(
        "{}\nDiscussion: {}\n{}: \"{}\"\nDetil: {}",
        format!("Comment {}!", payload.get_action()?),
        payload.get_discussion()?.title,
        comment.user.login,
        comment.body.replace("\\r\\n", "\n"),
        comment.html_url
    ))
}
