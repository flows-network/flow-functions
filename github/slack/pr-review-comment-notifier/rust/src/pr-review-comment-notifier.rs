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
        "{}({}) {} a Pull request review comment in `{}`({}):\n{}\nDetil: {}",
        comment.user.login,
        comment.author_association,
        payload.get_action()?,
        payload.get_pull_request()?.title,
        payload.get_repository()?.full_name,
        comment.body,
        comment.html_url
    ))
}
