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
    let fork = payload.get_fork()?;
    let repo = payload.get_repository()?;

    Ok(format!(
        "{} forked your {}({}) to {}({})!\n{}",
        payload.sender.login,
        repo.full_name,
        repo.visibility,
        fork.full_name,
        fork.visibility,
        fork.html_url
    ))
}
