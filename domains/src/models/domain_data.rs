/*
 * Domain API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainData {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "cnames", skip_serializing_if = "Option::is_none")]
    pub cnames: Option<Vec<String>>,
    #[serde(rename = "cname_access_only", skip_serializing_if = "Option::is_none")]
    pub cname_access_only: Option<bool>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "edge_application_id", skip_serializing_if = "Option::is_none")]
    pub edge_application_id: Option<i64>,
    #[serde(rename = "digital_certificate_id", skip_serializing_if = "Option::is_none")]
    pub digital_certificate_id: Option<String>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    #[serde(rename = "is_mtls_enabled", skip_serializing_if = "Option::is_none")]
    pub is_mtls_enabled: Option<bool>,
    #[serde(rename = "mtls_trusted_ca_certificate_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mtls_trusted_ca_certificate_id: Option<Option<i64>>,
    #[serde(rename = "edge_firewall_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edge_firewall_id: Option<Option<i64>>,
    #[serde(rename = "mtls_verification", skip_serializing_if = "Option::is_none")]
    pub mtls_verification: Option<MtlsVerification>,
    #[serde(rename = "crl_list", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub crl_list: Option<Option<Vec<i64>>>,
}

impl DomainData {
    pub fn new() -> DomainData {
        DomainData {
            name: None,
            cnames: None,
            cname_access_only: None,
            is_active: None,
            edge_application_id: None,
            digital_certificate_id: None,
            environment: None,
            is_mtls_enabled: None,
            mtls_trusted_ca_certificate_id: None,
            edge_firewall_id: None,
            mtls_verification: None,
            crl_list: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "production")]
    Production,
    #[serde(rename = "preview")]
    Preview,
}

impl Default for Environment {
    fn default() -> Environment {
        Self::Production
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MtlsVerification {
    #[serde(rename = "enforce")]
    Enforce,
    #[serde(rename = "permissive")]
    Permissive,
}

impl Default for MtlsVerification {
    fn default() -> MtlsVerification {
        Self::Enforce
    }
}

