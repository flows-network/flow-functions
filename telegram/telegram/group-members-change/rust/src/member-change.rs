use flows_connector_dsi::telegram::{outbound::*, *};
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let message = payload.as_message()?;

    if let Some(n) = message.new_chat_members.first() {
        outbound::message(
            message.chat.id,
            format!("Welcome [{}](tg://user?id={})\\!", n.first_name, n.id),
        )
        .parse_mode(ParseMode::MarkdownV2)
        .build()
    } else if let Some(l) = &message.left_chat_member {
        outbound::message(
            message.chat.id,
            format!("[{}](tg://user?id={}) left\\!", l.first_name, l.id),
        )
        .parse_mode(ParseMode::MarkdownV2)
        .build()
    } else {
        Ok(String::new())
    }
}
