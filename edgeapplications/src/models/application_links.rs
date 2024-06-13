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
pub struct ApplicationLinks {
    #[serde(rename = "previous", deserialize_with = "Option::deserialize")]
    pub previous: Option<String>,
    #[serde(rename = "next", deserialize_with = "Option::deserialize")]
    pub next: Option<String>,
}

impl ApplicationLinks {
    pub fn new(previous: Option<String>, next: Option<String>) -> ApplicationLinks {
        ApplicationLinks {
            previous,
            next,
        }
    }
}

