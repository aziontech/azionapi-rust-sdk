/*
 * Intelligent DNS API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsSecDelegationSigner {
    #[serde(rename = "digest_type", skip_serializing_if = "Option::is_none")]
    pub digest_type: Option<Box<crate::models::DnsSecDelegationSignerDigestType>>,
    #[serde(rename = "algorithm_type", skip_serializing_if = "Option::is_none")]
    pub algorithm_type: Option<Box<crate::models::DnsSecDelegationSignerDigestType>>,
    #[serde(rename = "digest", skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "key_tag", skip_serializing_if = "Option::is_none")]
    pub key_tag: Option<i32>,
}

impl DnsSecDelegationSigner {
    pub fn new() -> DnsSecDelegationSigner {
        DnsSecDelegationSigner {
            digest_type: None,
            algorithm_type: None,
            digest: None,
            key_tag: None,
        }
    }
}


