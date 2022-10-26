use flows_connector_dsi::{github::inbound, sendgrid::outbound};
use rand::seq::SliceRandom;
use rand::thread_rng;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    #[allow(unused_imports)]
    use wasmedge_bindgen::*;
    _run(s)
}

pub fn _run(s: String) -> Result<String, String> {
    let payload = inbound(s)?;

    if payload.get_action()? != "created" {
        return Ok(String::new());
    }

    let email = match payload
        .starred_at
        .as_ref()
        .and(payload.sender.email.as_ref())
    {
        Some(e) => e,
        None => return Ok(String::new()),
    };

    let sender = &payload.sender.login;
    let repo = &payload.get_repository()?.full_name;

    let body = pick_question();
    outbound(vec![email])
        .subject(
            "Thanks for your star! Get your WasmEdge SWAG by answering a simple question.",
        )
        .content(format!(
            r#"
Hi {}!<br/>

Welcome to the {} community! Here comes the question:.<br/>
{}

Bring the email to  WasmEdge kiosk at KubeCon and CloudNativeCon NA 2022 & tell us the answer to claim your WasmEdge SWAG.<br/>
"#,
            sender, repo, body
        ))
        .build()
}

pub fn pick_question() -> String {
    let q_1: &str = r#"Which foundation does WasmEdge belong to?
A CNCF
B LF Edge
"#;

    let q_2: &str = r#"Which AI frameworks has WasmEdge already supported? (multiple choice)
A TensorFlow
B Pytorch
C OpenVino
"#;

    let q_3: &str = r#"Tokio-based async apps can be compiled into Wasm and run in WasmEdge. Is this correct?
A Yes
B No
"#;

    let q_4: &str = r#"WasmEdge could run Nodejs applications. Is this correct?
A Yes
B No
C working in progress
"#;

    let q_5: &str = r#"WasmEdge supports both HTTP and HTTPS. Is this correct?
A Yes
B HTTP only
C HTTPS only
D None of them
"#;

    let q_6: &str = r#"Which relational database management system does WasmEdge support?(multiple choice)
A MySQL
B MariaDB
C TiDB
D Postgres
"#;

    let q_7: &str = r#"WasmEdge is OCI compatible and could be managed by K8s. Right?
A Yes
B No
"#;

    let q_8: &str = r#"Which platforms does WasmEdge support in the following list?(multiple choice)
A Android
B OpenWRT
C Sel4
D centos 7
"#;

    let q_9: &str = r#"What's the latest version of WasmEdge
A 0.10.0
B 0.11.0
C 0.11.1
D 0.11.2
"#;

    let q_10: &str = r#"When did WebAssembly become the fourth standard of Web
A 2016
B 2019
C 2020
D2021
"#;

    let arr = [q_1, q_2, q_3, q_4, q_5, q_6, q_7, q_8, q_9, q_10];
    arr.choose(&mut thread_rng()).unwrap().to_string()
}
