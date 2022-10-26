use flows_connector_dsi::telegram::{outbound, *};
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let message = payload.as_message()?;

    match message.text.contains("shit") {
        true => outbound::ban(
            message.chat.id,
            message.from.as_ref().unwrap().id.to_string(),
        )
        .build(),
        false => Ok(String::new()),
    }
}
