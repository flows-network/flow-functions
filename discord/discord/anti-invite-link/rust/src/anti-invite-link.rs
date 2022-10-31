use flows_connector_dsi::discord::*;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let message = inbound(s)?;

    if message.content.contains("discord.gg") {
        outbound::ban(message.author.id, "Send group invite link", 0).build()
    } else {
        Ok(String::new())
    }
}
