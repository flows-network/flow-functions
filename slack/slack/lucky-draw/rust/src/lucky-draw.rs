#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;
use rand::{thread_rng, Rng};

#[wasmedge_bindgen]
pub fn run(s: String) -> String {
    let mut rng = thread_rng();

    match s.to_ascii_lowercase().as_str() {
        "lucky draw" | "y" => {
            let is_lucky = rng.gen_bool(1.0 / 2.0);
            if is_lucky {
                return "You are lucky!".to_string();
            } else { return "Try your luck next time!".to_string(); }
        },

        word if word.starts_with(&"luck") | word.ends_with(&"draw") => {
            return r#"
            Do you mean "lucky draw"?
            If Yes, please type "Y". Plase enter "N" to abort."#.to_string();
        },
        "N" | "n" => { return "".to_string(); },
        _ => { return "I don't quite understand you. Please reenter your instruction!".to_string
        (); }
    }
}
