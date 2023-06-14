/*
 * Intelligent DNS
 *
 * Azion Intelligent DNS API
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RecordPostOrPut {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "entry", skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "answers_list", skip_serializing_if = "Option::is_none")]
    pub answers_list: Option<Vec<String>>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "record_type", skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
}

impl RecordPostOrPut {
    pub fn new() -> RecordPostOrPut {
        RecordPostOrPut {
            id: None,
            entry: None,
            description: None,
            answers_list: None,
            policy: None,
            weight: None,
            record_type: None,
            ttl: None,
        }
    }
}

