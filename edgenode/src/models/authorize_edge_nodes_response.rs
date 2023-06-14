/*
 * Edgenode API
 *
 * Azion Orchestration
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AuthorizeEdgeNodesResponse {
    #[serde(rename = "authorized")]
    pub authorized: Vec<i64>,
    #[serde(rename = "errors")]
    pub errors: Vec<crate::models::UnauthorizedEdgeNodeInfo>,
}

impl AuthorizeEdgeNodesResponse {
    pub fn new(authorized: Vec<i64>, errors: Vec<crate::models::UnauthorizedEdgeNodeInfo>) -> AuthorizeEdgeNodesResponse {
        AuthorizeEdgeNodesResponse {
            authorized,
            errors,
        }
    }
}

