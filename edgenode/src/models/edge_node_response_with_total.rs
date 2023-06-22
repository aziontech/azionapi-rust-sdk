/*
 * Edge Node API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EdgeNodeResponseWithTotal {
    #[serde(rename = "nodes")]
    pub nodes: Vec<crate::models::EdgeNodeResponse>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl EdgeNodeResponseWithTotal {
    pub fn new(nodes: Vec<crate::models::EdgeNodeResponse>, total: i64) -> EdgeNodeResponseWithTotal {
        EdgeNodeResponseWithTotal {
            nodes,
            total,
        }
    }
}


