use connector_dsi::{github::inbound, twilio::outbound};
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;

    let repo = payload.get_repository()?;

    if payload.starred_at.is_none()
        || payload.get_action()? != "created"
        || repo.stargazers_count % 10 != 0
    {
        return Ok(String::new());
    }

    outbound("+8612345678901")
        .body(format!(
            "Congratulations on your repository {} with {} stars.",
            repo.full_name, repo.stargazers_count
        ))
        .build()
}
