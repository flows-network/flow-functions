#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use flows_connector_dsi::discord::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let message = inbound(s)?;

    if message.content == "/ping" {
        outbound::say("PONG!", Some(message))
            .build()
    } else {
        Ok(String::new())
    }
}
