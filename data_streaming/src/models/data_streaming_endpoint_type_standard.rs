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
pub struct DataStreamingEndpointTypeStandard {
    #[serde(rename = "endpoint_type", skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "log_line_separator", skip_serializing_if = "Option::is_none")]
    pub log_line_separator: Option<String>,
    #[serde(rename = "payload_format", skip_serializing_if = "Option::is_none")]
    pub payload_format: Option<String>,
    #[serde(rename = "max_size", skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Box<crate::models::DataStreamingEndpointTypeStandardHeadersExample>>,
}

impl DataStreamingEndpointTypeStandard {
    pub fn new() -> DataStreamingEndpointTypeStandard {
        DataStreamingEndpointTypeStandard {
            endpoint_type: None,
            url: None,
            log_line_separator: None,
            payload_format: None,
            max_size: None,
            headers: None,
        }
    }
}


