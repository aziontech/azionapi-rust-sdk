/*
 * Edge Firewall API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeFirewall {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "last_editor", skip_serializing_if = "Option::is_none")]
    pub last_editor: Option<String>,
    #[serde(rename = "last_modified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "edge_functions_enabled", skip_serializing_if = "Option::is_none")]
    pub edge_functions_enabled: Option<bool>,
    #[serde(rename = "network_protection_enabled", skip_serializing_if = "Option::is_none")]
    pub network_protection_enabled: Option<bool>,
    #[serde(rename = "waf_enabled", skip_serializing_if = "Option::is_none")]
    pub waf_enabled: Option<bool>,
    #[serde(rename = "debug_rules", skip_serializing_if = "Option::is_none")]
    pub debug_rules: Option<bool>,
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<i64>>,
}

impl EdgeFirewall {
    pub fn new() -> EdgeFirewall {
        EdgeFirewall {
            id: None,
            name: None,
            is_active: None,
            last_editor: None,
            last_modified: None,
            edge_functions_enabled: None,
            network_protection_enabled: None,
            waf_enabled: None,
            debug_rules: None,
            domains: None,
        }
    }
}


