/*
 * Edge Application API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchOriginsRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "origin_type", skip_serializing_if = "Option::is_none")]
    pub origin_type: Option<String>,
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<models::CreateOriginsRequestAddresses>>,
    #[serde(rename = "origin_protocol_policy", skip_serializing_if = "Option::is_none")]
    pub origin_protocol_policy: Option<String>,
    #[serde(rename = "host_header", skip_serializing_if = "Option::is_none")]
    pub host_header: Option<String>,
    #[serde(rename = "origin_path", skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    #[serde(rename = "hmac_authentication", skip_serializing_if = "Option::is_none")]
    pub hmac_authentication: Option<bool>,
    #[serde(rename = "hmac_region_name", skip_serializing_if = "Option::is_none")]
    pub hmac_region_name: Option<String>,
    #[serde(rename = "hmac_access_key", skip_serializing_if = "Option::is_none")]
    pub hmac_access_key: Option<String>,
    #[serde(rename = "hmac_secret_key", skip_serializing_if = "Option::is_none")]
    pub hmac_secret_key: Option<String>,
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl PatchOriginsRequest {
    pub fn new() -> PatchOriginsRequest {
        PatchOriginsRequest {
            name: None,
            origin_type: None,
            addresses: None,
            origin_protocol_policy: None,
            host_header: None,
            origin_path: None,
            hmac_authentication: None,
            hmac_region_name: None,
            hmac_access_key: None,
            hmac_secret_key: None,
            bucket: None,
            prefix: None,
        }
    }
}

