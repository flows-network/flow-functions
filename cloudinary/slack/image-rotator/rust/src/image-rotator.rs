#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use regex::Regex;

#[wasmedge_bindgen]
pub fn run(url: String) -> String {
    let reg = Regex::new("/v\\w+/").unwrap();
    return reg.replace(url.as_str(), "/a_90/").to_string();
}
