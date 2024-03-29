/*
 * Network Lists API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNetworkListsRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "items_values", skip_serializing_if = "Option::is_none")]
    pub items_values: Option<Vec<String>>,
    #[serde(rename = "list_type", skip_serializing_if = "Option::is_none")]
    pub list_type: Option<ListType>,
}

impl CreateNetworkListsRequest {
    pub fn new() -> CreateNetworkListsRequest {
        CreateNetworkListsRequest {
            name: None,
            items_values: None,
            list_type: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ListType {
    #[serde(rename = "ip_cidr")]
    IpCidr,
    #[serde(rename = "asn")]
    Asn,
    #[serde(rename = "countries")]
    Countries,
}

impl Default for ListType {
    fn default() -> ListType {
        Self::IpCidr
    }
}

