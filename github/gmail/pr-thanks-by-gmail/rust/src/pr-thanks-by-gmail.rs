use wasmedge_bindgen_macro::*;
use connector_dsi::{github::inbound, gmail::outbound};

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
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

    outbound(email)
        .subject("Thank you for Pull Request this repository")
        .content(format!(
            r#"
Howdy {},
Welcome to the {} community, Thank you for Pull Request this repository.
            "#,
            payload.sender.login,
            payload.get_repository()?.full_name
        ))
        .build()
}
