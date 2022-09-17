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
    let discussion = payload.get_discussion()?;

    Ok(format!(
        "{}\nTitle: {}\nBody: {}\nDetil: {}",
        format!("Discussion {}", payload.get_action()?),
        discussion.title,
        discussion.body,
        discussion.html_url
    ))
}

