use flows_connector_dsi::sendgrid::outbound;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    if s.trim().is_empty() {
        return Err("Wrong trigger is emitted somewhere".to_string());
    }
    outbound(vec!["jaykchen@gmail.com", "achenics@gmail.com"])
        .subject("This is reminder of your task on xxx")
        .content(
            r#"
Hi,<br/>

Please be kindly reminded that the monthly meeting is scheduled at 10am on 11, Oct."#)
        .build()
}
