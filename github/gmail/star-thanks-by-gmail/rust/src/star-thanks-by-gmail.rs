use wasmedge_bindgen_macro::*;
use connector_dsi::{github::inbound, gmail::outbound};

#[cfg(target_family = "wasm")]
#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;
    let email = match payload
        .starred_at
        .as_ref()
        .and(payload.sender.email.as_ref())
    {
        Some(e) => e,
        None => return Ok(String::new()),
    };

    if payload.get_action()? == "created" {
        outbound(email)
            .subject("Thanks for your star!")
            .content(format!(
                r#"
Hi {}, we have received your star to our repository {}.
We are so appreciative and wish you have more fun with open source.

Best regards"#,
                payload.sender.login,
                payload.get_repository()?.full_name
            ))
            .build()
    } else {
        outbound(email)
            .subject("Sorry to lose you")
            .content(format!(
                r#"
Hi {}, we are very disappointed that you have unstarred our repository {}.
Hope you can give us more advice to make us getting better.
                
Best regards"#,
                payload.sender.login,
                payload.get_repository()?.full_name
            ))
            .build()
    }
}
