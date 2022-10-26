use flows_connector_dsi::telegram::{outbound::ParseMode, *};
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let message = payload.as_message()?;

    if message.text == "/ping" {
        outbound::message(message.chat.id, "__PONG\\!__")
            .reply(message.message_id.to_string())
            .parse_mode(ParseMode::MarkdownV2)
            .build()
    } else {
        Ok(String::new())
    }
}
