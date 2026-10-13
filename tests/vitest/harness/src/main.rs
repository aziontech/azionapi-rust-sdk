//! Test harness for the generated Azion Rust SDK crates.
//!
//! Usage: sdk-harness <case> <base_url> <api_token> <argument>
//!
//! Calls one generated API function against `base_url` and prints a JSON
//! line: `{"ok":true,"data":<deserialized model>}` on success, or
//! `{"ok":false,"status":<http status>,"error":"..."}` when the SDK returns
//! an error. The generated code is used as is.

use serde_json::{json, Value};
use std::fmt::Display;
use std::process::ExitCode;

/// Builds the `Configuration` of a generated crate: every crate has its own
/// copy of the type, with the same fields.
macro_rules! configuration {
    ($krate:ident, $base:expr, $token:expr) => {{
        let mut c = $krate::apis::configuration::Configuration::new();
        c.base_path = $base.to_owned();
        c.api_key = Some($krate::apis::configuration::ApiKey {
            prefix: Some("Token".to_owned()),
            key: $token.to_owned(),
        });
        c
    }};
}

/// Converts the result of a generated function into the harness output.
macro_rules! outcome {
    ($krate:ident, $result:expr) => {
        match $result {
            Ok(model) => json!({ "ok": true, "data": serde_json::to_value(&model).expect("model serializes") }),
            Err($krate::apis::Error::ResponseError(response)) => {
                json!({ "ok": false, "status": response.status.as_u16(), "error": response.content })
            }
            Err(other) => json!({ "ok": false, "status": Value::Null, "error": describe(other) }),
        }
    };
}

fn describe(error: impl Display) -> String {
    error.to_string()
}

async fn run(case: &str, base: &str, token: &str, arg: &str) -> Result<Value, String> {
    Ok(match case {
        "personal_tokens.get_personal_token" => {
            let c = configuration!(personal_tokens, base, token);
            outcome!(personal_tokens, personal_tokens::apis::personal_token_api::get_personal_token(&c, arg).await)
        }
        "variables.api_variables_retrieve" => {
            let c = configuration!(variables, base, token);
            outcome!(variables, variables::apis::variables_api::api_variables_retrieve(&c, arg).await)
        }
        "edgefunctions.edge_functions_id_get" => {
            let c = configuration!(edgefunctions, base, token);
            let id: i64 = arg.parse().map_err(|e| format!("invalid id: {e}"))?;
            outcome!(edgefunctions, edgefunctions::apis::edge_functions_api::edge_functions_id_get(&c, id).await)
        }
        "domains.get_domain" => {
            let c = configuration!(domains, base, token);
            outcome!(domains, domains::apis::domains_api::get_domain(&c, arg, None).await)
        }
        other => return Err(format!("unknown case: {other}")),
    })
}

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        eprintln!("usage: sdk-harness <case> <base_url> <api_token> <argument>");
        return ExitCode::from(2);
    }
    match run(&args[0], &args[1], &args[2], &args[3]).await {
        Ok(value) => {
            println!("{value}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}
