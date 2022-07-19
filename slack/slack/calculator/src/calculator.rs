use meval;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

use text_to_png::TextRenderer;

#[wasmedge_bindgen]
pub fn _run(exp: String) -> String {
    let r = meval::eval_str(&exp).unwrap();
    let e = format!("{} = {}", exp, r);
    return e;
}

// #[wasmedge_bindgen]
// pub fn run(exp: String) -> (String, String, String, Vec<u8>) {
// 	let r = meval::eval_str(&exp).unwrap();
// 	let e = format!("{} = {}", exp, r);

// 	let renderer = TextRenderer::try_new_with_ttf_font_data(include_bytes!("/home/darumadocker/Downloads/Monoton-Regular.ttf"))
//     	.expect("font is definitely loadable");

// 	let text_png = renderer.render_text_to_png_data(
// 		e,
// 		200,
// 		0x439EC2).unwrap();
// 	return (String::new(), "math.png".to_string(), "image/png".to_string(), text_png.data);
// }

// #[wasmedge_bindgen]
// pub fn run_file_1(exp: String, pic: Vec<u8>) -> (String, String, String, Vec<u8>) {
// 	let r = meval::eval_str(&exp).unwrap();
// 	return (format!("{} = {}", exp, r), "1.png".to_string(), "image/png".to_string(), pic);
// }
