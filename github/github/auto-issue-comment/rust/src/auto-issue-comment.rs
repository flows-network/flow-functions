use wasmedge_bindgen_macro::*;
use connector_dsi::github::{inbound, outbound};

#[cfg(target_family = "wasm")]
#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;

    if payload.get_action()? != "opened" {
        return Ok(String::new());
    }

    outbound::modify_issue(payload.get_issue()?.number)
        .body("Thanks for your feedback. Our maintainer will respond to you soon.")
        .build()
}
