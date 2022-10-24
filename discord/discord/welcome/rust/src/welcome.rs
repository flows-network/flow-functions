use flows_connector_dsi::discord::*;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let message = inbound(s)?;

    if message.kind == MessageType::MemberJoin {
        outbound::say(format!("Welcome <@{}>!", message.author.id), None).build()
    } else {
        Ok(String::new())
    }
}
