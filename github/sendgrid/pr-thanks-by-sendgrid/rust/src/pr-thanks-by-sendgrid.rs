use wasmedge_bindgen_macro::*;
use flows_connector_dsi::{github::inbound, sendgrid::outbound};

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen;
    _run(s)
}

fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let email = match payload
        .pull_request
        .as_ref()
        .and(payload.sender.email.as_ref())
    {
        Some(e) => e,
        None => return Ok(String::new()),
    };

    if payload.get_action()? != "opened" {
        return Ok(String::new());
    }

    outbound(vec![email])
        .subject("Thank you for contributing to this repository")
        .content(format!(
            r#"
Hi {}, <br/>

Welcome to the {} community, thank you for your contribution!"#,
            payload.sender.login,
            payload.get_repository()?.full_name
        ))
        .build()
}
