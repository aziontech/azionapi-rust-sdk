/*
 * Domain API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainLinks {
    #[serde(rename = "previous")]
    pub previous: String,
    #[serde(rename = "next")]
    pub next: String,
}

impl DomainLinks {
    pub fn new(previous: String, next: String) -> DomainLinks {
        DomainLinks {
            previous,
            next,
        }
    }
}


