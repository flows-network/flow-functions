use wasmedge_bindgen_macro::*;
use connector_dsi::{github::inbound, notion::outbound};

#[cfg(target_family = "wasm")]
#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;

    if payload.get_action()? != "assigned" {
        return Ok(String::new());
    }

    outbound()
        .page("Name", &payload.get_issue()?.html_url)
        .build()
}
