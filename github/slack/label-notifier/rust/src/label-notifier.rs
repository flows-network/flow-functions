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

    Ok(format!(
        "Label \"{}\" {}!\n{}",
        payload.get_label()?.name,
        payload.get_action()?,
        payload.get_repository()?.html_url
    ))
}
