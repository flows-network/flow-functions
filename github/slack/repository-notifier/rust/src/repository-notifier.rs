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
    let release = payload.get_release()?;

    Ok(format!(
        "Repository {} {} by {}!\nDetil: {}",
        payload.get_repository()?.full_name,
        payload.get_action()?,
        payload.sender.login,
        release.html_url
    ))
}
