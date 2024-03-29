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
pub struct SetRateLimitBehaviorArgument {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "limit_by", skip_serializing_if = "Option::is_none")]
    pub limit_by: Option<LimitBy>,
    #[serde(rename = "average_rate_limit", skip_serializing_if = "Option::is_none")]
    pub average_rate_limit: Option<Box<crate::models::SetRateLimitBehaviorArgumentAverageRateLimit>>,
    #[serde(rename = "maximum_burst_size", skip_serializing_if = "Option::is_none")]
    pub maximum_burst_size: Option<Box<crate::models::SetRateLimitBehaviorArgumentAverageRateLimit>>,
}

impl SetRateLimitBehaviorArgument {
    pub fn new() -> SetRateLimitBehaviorArgument {
        SetRateLimitBehaviorArgument {
            r#type: None,
            limit_by: None,
            average_rate_limit: None,
            maximum_burst_size: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "minute")]
    Minute,
}

impl Default for Type {
    fn default() -> Type {
        Self::Second
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LimitBy {
    #[serde(rename = "client_ip")]
    ClientIp,
    #[serde(rename = "global")]
    Global,
}

impl Default for LimitBy {
    fn default() -> LimitBy {
        Self::ClientIp
    }
}

