/*
 * Data Streaming - OpenAPI
 *
 * The Data Streaming API allows you to manage your existing data streamings and templates. Data Streaming allows you to feed your stream processing, SIEM, and big data platforms with the event logs from your applications on Azion in real time. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointGoogleBigQueryServiceAccountKey {
    #[serde(rename = "service_account_key", skip_serializing_if = "Option::is_none")]
    pub service_account_key: Option<String>,
}

impl EndpointGoogleBigQueryServiceAccountKey {
    pub fn new() -> EndpointGoogleBigQueryServiceAccountKey {
        EndpointGoogleBigQueryServiceAccountKey {
            service_account_key: None,
        }
    }
}


