/*
 * Web Application Firewall API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WafSensitivityChoices {
    #[serde(rename = "lowest")]
    Lowest,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "highest")]
    Highest,

}

impl ToString for WafSensitivityChoices {
    fn to_string(&self) -> String {
        match self {
            Self::Lowest => String::from("lowest"),
            Self::Low => String::from("low"),
            Self::Medium => String::from("medium"),
            Self::High => String::from("high"),
            Self::Highest => String::from("highest"),
        }
    }
}

impl Default for WafSensitivityChoices {
    fn default() -> WafSensitivityChoices {
        Self::Lowest
    }
}




