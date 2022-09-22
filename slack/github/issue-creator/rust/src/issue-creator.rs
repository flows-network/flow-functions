#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use flows_connector_dsi::github::outbound;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    outbound::create_issue(&s)
        .body(s)
        .build()
}
