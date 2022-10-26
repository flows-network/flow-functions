use flows_connector_dsi::telegram::{outbound, *};
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let message = payload.as_message()?;
    let except = "secondstate.io";

    match message.entities.first() {
        Some(n) if n.r#type.eq("url") && message.text.contains(except) => {
            outbound::message(message.chat.id, "Can't send strange url in this group!").build()
        }
        _ => Ok(String::new()),
    }
}
