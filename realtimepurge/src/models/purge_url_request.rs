/*
 * Purge API
 *
 * Azion Real-Time Purge
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PurgeUrlRequest {
    #[serde(rename = "urls")]
    pub urls: Vec<String>,
    #[serde(rename = "method")]
    pub method: String,
}

impl PurgeUrlRequest {
    pub fn new(urls: Vec<String>, method: String) -> PurgeUrlRequest {
        PurgeUrlRequest {
            urls,
            method,
        }
    }
}

