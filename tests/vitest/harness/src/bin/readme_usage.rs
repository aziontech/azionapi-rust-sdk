//! The usage example from the repository README, compiled by the test suite
//! so the documentation keeps matching the generated API.

use personal_tokens::apis::{configuration::{ApiKey, Configuration}, personal_token_api};

#[tokio::main]
async fn main() {
    let mut configuration = Configuration::new();
    configuration.api_key = Some(ApiKey {
        prefix: Some("Token".to_owned()),
        key: "YOUR_API_TOKEN".to_owned(),
    });

    match personal_token_api::list_personal_token(&configuration).await {
        Ok(tokens) => println!("{:?}", tokens),
        Err(error) => eprintln!("request failed: {}", error),
    }
}
