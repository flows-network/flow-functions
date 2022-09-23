use flows_connector_dsi::{github::inbound};
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen;
    _run(s)
}

fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;

    let commit = payload.get_head_commit()?;
    Ok(format!(
        "@DarumaDocker, @{} has commited to {}",
        payload.sender.login,
        commit.url,
    ))
}

