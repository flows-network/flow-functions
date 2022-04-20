use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use meval;

#[wasmedge_bindgen]
pub fn run(exp: String) -> String {
	let r = meval::eval_str(&exp).unwrap();
	return format!("{} = {}", exp, r);
}